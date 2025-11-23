use embassy_futures::select::{select3, Either3};
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c;
use esp_hal::gpio::Input;

#[embassy_executor::task]
pub async fn ds1307_task(mut i2c0: &'static crate::I2c0) {
    Timer::after(Duration::from_millis(5000)).await; // wait for other init
    let mut i2c_device = embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice::new(&mut i2c0);
    loop {
        let mut buffer = [0u8; 3];
        i2c_device.write_read(0x68, &[0x00], &mut buffer).await.unwrap();
        info!("readed: {:02x} {:02x} {:02x}", buffer[0], buffer[1], buffer[2]);
        Timer::after(Duration::from_millis(1000)).await;
    }
}
