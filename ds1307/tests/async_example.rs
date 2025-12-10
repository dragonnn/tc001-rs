use ds1307::{Ds1307, async_api::AsyncRtc};
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

#[tokio::test]
async fn test_get_datetime_async() {
    use ds1307::registers::Register;
    let expectations = vec![I2cTransaction::write_read(
        0x68,
        vec![Register::Seconds.addr()],
        vec![0x00, 0x00, 0x12, 0x01, 0x01, 0x01, 0x24],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut rtc = Ds1307::new(i2c);
    let dt = rtc.get_datetime().await.unwrap();
    assert_eq!(dt.year(), 2024);
}
