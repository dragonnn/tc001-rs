use embassy_net::IpEndpoint;

use crate::udp::UdpBuffers;

mod sntpc;

async fn ntp_request(stack: embassy_net::Stack<'static>) -> Result<chrono::NaiveDateTime, ()> {
    info!("Prepare NTP request");
    let mut addrs = stack.dns_query("pl.pool.ntp.org", smoltcp::wire::DnsQueryType::A).await.unwrap_or_default();
    let addr = addrs.pop().ok_or(())?;
    info!("NTP DNS: {:?}", addr);

    let ntp_packet = sntpc::NtpPacket::new();
    let raw_ntp = sntpc::RawNtpPacket::from(&ntp_packet);

    let mut buffers = UdpBuffers::new();

    let (rx_meta, rx_buffer, tx_meta, tx_buffer) = buffers.as_static_mut();

    let mut socket = embassy_net::udp::UdpSocket::new(stack, rx_meta, rx_buffer, tx_meta, tx_buffer);

    socket.bind(11770).map_err(|e| ())?;
    info!("UDP socket bound");

    socket.send_to(&raw_ntp.0, IpEndpoint::new(addr, 123)).await.map_err(|e| ())?;
    info!("NTP request sent");

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
pub async fn ntp_task(stack: embassy_net::Stack<'static>, rtc: &'static esp_hal::rtc_cntl::Rtc<'static>) {
    loop {
        match ntp_request(stack.clone()).await {
            Ok(date) => {
                info!("NTP date: {:?}", date);
                rtc.set_current_time_us(date.and_utc().timestamp_micros() as u64);
            }
            Err(_) => {
                error!("NTP request failed");
            }
        }
        embassy_time::Timer::after(embassy_time::Duration::from_secs(20)).await;
    }
}
