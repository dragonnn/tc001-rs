// Async power control (oscillator start/stop)

pub use crate::async_api::AsyncRtcPowerControl;
use crate::{Ds1307, registers::*};

impl<I2C> AsyncRtcPowerControl for Ds1307<I2C>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    async fn start_clock(&mut self) -> Result<(), Self::Error> {
        self.clear_register_bits(Register::Seconds, CH_BIT).await
    }

    async fn halt_clock(&mut self) -> Result<(), Self::Error> {
        self.set_register_bits(Register::Seconds, CH_BIT).await
    }
}
