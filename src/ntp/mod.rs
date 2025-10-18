use embassy_net::IpEndpoint;
use embassy_time::Instant;

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
pub async fn ntp_task(stack: embassy_net::Stack<'static>, rtc: &'static esp_hal::rtc_cntl::Rtc<'static>) {
    let mut last_ntp_date = None;
    let mut now = Instant::now();
    loop {
        match ntp_request(stack.clone()).await {
            Ok(date) => {
                info!("NTP date: {:?}", date);
                let rtc_now = chrono::NaiveDateTime::from_timestamp_micros(rtc.current_time_us() as i64).unwrap();
                let rtc_delta = date.signed_duration_since(rtc_now);
                info!("RTC delta: {} seconds", rtc_delta.as_seconds_f64());

                rtc.set_current_time_us(date.and_utc().timestamp_micros() as u64);

                if let Some(last_date) = last_ntp_date {
                    let delta = date.signed_duration_since(last_date);
                    let elapsed = now.elapsed();
                    info!(
                        "Time since last NTP: {} seconds (RTC delta: {} seconds)",
                        elapsed.as_millis() as f32 / 1000.0,
                        delta.as_seconds_f64()
                    );
                }

                now = Instant::now();
                last_ntp_date = Some(date);
            }
            Err(_) => {
                error!("NTP request failed");
            }
        }
        embassy_time::Timer::after(embassy_time::Duration::from_secs(20)).await;
    }
}
