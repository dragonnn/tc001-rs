use embassy_net::IpEndpoint;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{with_timeout, Duration, Instant, Timer};

use crate::udp::UdpBuffers;

mod sntpc;

static NTP_SYNC: Signal<CriticalSectionRawMutex, chrono::NaiveDateTime> = Signal::new();

async fn ntp_request(stack: embassy_net::Stack<'static>) -> Result<chrono::NaiveDateTime, ()> {
    let mut addrs = stack.dns_query("pl.pool.ntp.org", smoltcp::wire::DnsQueryType::A).await.unwrap_or_default();
    let addr = addrs.pop().ok_or(())?;

    let ntp_packet = sntpc::NtpPacket::new();
    let raw_ntp = sntpc::RawNtpPacket::from(&ntp_packet);

    let mut buffers = UdpBuffers::new();

    let (rx_meta, rx_buffer, tx_meta, tx_buffer) = buffers.as_mut();

    let mut socket = embassy_net::udp::UdpSocket::new(stack, rx_meta, rx_buffer, tx_meta, tx_buffer);

    socket.bind(11770).map_err(|e| ())?;

    socket.send_to(&raw_ntp.0, IpEndpoint::new(addr, 123)).await.map_err(|e| ())?;

    let mut buffer = [0u8; 48];

    socket.recv_from(&mut buffer).await.map_err(|e| ())?;

    let mut raw_ntp = sntpc::RawNtpPacket::default();
    raw_ntp.0 = buffer;

    let recv_timestamp = embassy_time::Instant::now();
    let recv_timestamp = sntpc::get_ntp_timestamp(&recv_timestamp);

    let result = sntpc::process_response(ntp_packet.into(), raw_ntp, recv_timestamp);

    match result {
        Ok(packet) => match packet.to_datetime() {
            Some(datetime) => {
                let datetime = datetime.with_timezone(&chrono_tz::Europe::Warsaw);
                // info!("NTP time received: {:?}", defmt::Debug2Format(&datetime));

                return Ok(datetime.naive_local());
            }
            None => {
                error!("Failed to convert NTP packet to NaiveDateTime");
            }
        },
        Err(e) => {
            error!("Failed to process NTP response: {:?}", e);
        }
    }

    Err(())
}

#[embassy_executor::task]
pub async fn ntp_task(stack: embassy_net::Stack<'static>) {
    crate::wifi::wait_for_connection(&stack).await;
    Timer::after_secs(5).await;

    let mut ntp_error_count: u8 = 0;
    loop {
        match with_timeout(Duration::from_secs(5), ntp_request(stack.clone())).await {
            Ok(Ok(date)) => {
                ntp_error_count = 0;
                NTP_SYNC.signal(date);
            }
            Err(_) => {
                error!("NTP request timed out");
                ntp_error_count += 1;
            }
            Ok(Err(_)) => {
                error!("NTP request failed");
                ntp_error_count += 1;
            }
        }
        if ntp_error_count >= 10 {
            esp_hal::system::software_reset();
        }
        embassy_time::Timer::after(embassy_time::Duration::from_secs(120)).await;
    }
}

pub async fn wait_for_ntp_sync() -> chrono::NaiveDateTime {
    NTP_SYNC.wait().await
}
