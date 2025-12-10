use embedded_hal_async::i2c::I2c;

use crate::{error::Error, registers::*};

pub const I2C_ADDR: u8 = 0x68;

pub struct Ds1307<I2C> {
    i2c: I2C,
}

impl<I2C> Ds1307<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}

impl<I2C> Ds1307<I2C>
where
    I2C: I2c,
{
    /// Write a single byte to a DS1307 register
    pub(crate) async fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(I2C_ADDR, &[register.addr(), value]).await?;

        Ok(())
    }

    /// Read a single byte from a DS1307 register
    pub(crate) async fn read_register(&mut self, register: Register) -> Result<u8, Error<I2C::Error>> {
        let mut data = [0u8; 1];
        self.i2c.write_read(I2C_ADDR, &[register.addr()], &mut data).await?;

        Ok(data[0])
    }

    /// Read multiple bytes from DS1307 starting at a register
    pub(crate) async fn read_register_bytes(
        &mut self,
        register: Register,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        self.i2c.write_read(I2C_ADDR, &[register.addr()], buffer).await?;

        Ok(())
    }

    /// Read multiple bytes from DS1307 starting at a raw address
    pub(crate) async fn read_bytes_at_address(
        &mut self,
        register_addr: u8,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        self.i2c.write_read(I2C_ADDR, &[register_addr], buffer).await?;

        Ok(())
    }

    /// Write raw bytes directly to DS1307 via I2C (register address must be first byte)
    pub(crate) async fn write_raw_bytes(&mut self, data: &[u8]) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(I2C_ADDR, data).await?;

        Ok(())
    }

    // Read-modify-write operation for setting bits
    ///
    /// Performs a read-modify-write operation to set the bits specified by the mask
    /// while preserving all other bits in the register. Only performs a write if
    /// the register value would actually change, optimizing I2C bus usage.
    ///
    /// # Parameters
    /// - `register`: The DS1307 register to modify
    /// - `mask`: Bit mask where `1` bits will be set, `0` bits will be ignored
    ///
    /// # Example
    /// ```ignore
    /// // Set bits 2 and 4 in the control register
    /// self.set_register_bits(Register::Control, 0b0001_0100)?;
    /// ```
    ///
    /// # I2C Operations
    /// - 1 read + 1 write (if change needed)
    /// - 1 read only (if no change needed)
    pub(crate) async fn set_register_bits(&mut self, register: Register, mask: u8) -> Result<(), Error<I2C::Error>> {
        let current = self.read_register(register).await?;
        let new_value = current | mask;
        if new_value != current { self.write_register(register, new_value).await } else { Ok(()) }
    }

    /// Read-modify-write operation for clearing bits
    ///
    /// Performs a read-modify-write operation to clear the bits specified by the mask
    /// while preserving all other bits in the register. Only performs a write if
    /// the register value would actually change, optimizing I2C bus usage.
    ///
    /// # Parameters
    /// - `register`: The DS1307 register to modify
    /// - `mask`: Bit mask where `1` bits will be cleared, `0` bits will be ignored
    ///
    /// # Example
    /// ```ignore
    /// // Clear the Clock Halt bit (bit 7) in seconds register
    /// self.clear_register_bits(Register::Seconds, 0b1000_0000)?;
    /// ```
    ///
    /// # I2C Operations
    /// - 1 read + 1 write (if change needed)
    /// - 1 read only (if no change needed)
    pub(crate) async fn clear_register_bits(&mut self, register: Register, mask: u8) -> Result<(), Error<I2C::Error>> {
        let current = self.read_register(register).await?;
        let new_value = current & !mask;
        if new_value != current { self.write_register(register, new_value).await } else { Ok(()) }
    }

    /// Set the output pin to a static high state
    pub async fn set_output_high(&mut self) -> Result<(), Error<I2C::Error>> {
        let current = self.read_register(Register::Control).await?;
        let mut new_value = current;

        // Disable square wave and set OUT bit high
        new_value &= !SQWE_BIT;
        new_value |= OUT_BIT;

        if new_value != current { self.write_register(Register::Control, new_value).await } else { Ok(()) }
    }

    /// Set the output pin to a static low state
    pub async fn set_output_low(&mut self) -> Result<(), Error<I2C::Error>> {
        let current = self.read_register(Register::Control).await?;
        let mut new_value = current;

        // Disable square wave and set OUT bit low
        new_value &= !SQWE_BIT;
        new_value &= !OUT_BIT;

        if new_value != current { self.write_register(Register::Control, new_value).await } else { Ok(()) }
    }

    /// Validate NVRAM offset and length parameters before accessing memory.
    ///
    /// Returns an error if:
    /// - The starting offset is outside the available NVRAM range
    /// - The requested length goes beyond the end of NVRAM
    pub(crate) fn validate_nvram_bounds(&self, offset: u8, len: usize) -> Result<(), Error<I2C::Error>> {
        // Check if offset is within bounds
        if offset >= NVRAM_SIZE {
            return Err(Error::NvramOutOfBounds);
        }

        // Check if remaining space is sufficient
        let remaining_space = NVRAM_SIZE - offset;
        if len > remaining_space as usize {
            return Err(Error::NvramOutOfBounds);
        }

        Ok(())
    }
}

impl<I2C> rtc_hal::error::ErrorType for Ds1307<I2C>
where
    I2C: I2c,
{
    type Error = Error<I2C::Error>;
}
