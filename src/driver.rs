//! Core BQ76952 Driver Logic

use crate::error::Error;
use crate::registers::*;
use crate::types::*;
use embedded_hal::i2c::I2c;

pub struct Bq76952<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Bq76952<I2C>
where
    I2C: I2c<Error = E>,
{
    pub const fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Helper to compute checksum for data memory writes
    fn calculate_checksum(old_checksum: u8, data: u8) -> u8 {
        let mut chk = old_checksum;
        if chk == 0 {
            chk = data;
        } else {
            chk = (!chk).wrapping_add(data);
        }
        !chk
    }

    /// Send a direct command and read 2 bytes back
    pub fn direct_command(&mut self, command: u8) -> Result<u16, Error<E>> {
        let cmd_bytes = [command];
        let mut read_buf = [0u8; 2];

        self.i2c
            .write_read(self.address, &cmd_bytes, &mut read_buf)
            .map_err(Error::I2c)?;

        Ok(u16::from_le_bytes(read_buf))
    }

    /// Send a subcommand
    pub fn sub_command(&mut self, subcommand: u16) -> Result<(), Error<E>> {
        let bytes = [
            CMD_DIR_SUBCMD_LOW,
            (subcommand & 0xFF) as u8,
            ((subcommand >> 8) & 0xFF) as u8,
        ];
        self.i2c.write(self.address, &bytes).map_err(Error::I2c)
    }

    fn select_data_memory(&mut self, addr: u16) -> Result<(), Error<E>> {
        let addr_bytes = [
            CMD_DIR_SUBCMD_LOW,
            (addr & 0xFF) as u8,
            ((addr >> 8) & 0xFF) as u8,
        ];

        self.i2c
            .write(self.address, &addr_bytes)
            .map_err(Error::I2c)
    }

    pub fn read_data_memory_u8(&mut self, addr: u16) -> Result<u8, Error<E>> {
        self.select_data_memory(addr)?;

        let mut buf = [0u8; 1];

        self.i2c.read(self.address, &mut buf).map_err(Error::I2c)?;

        Ok(buf[0])
    }

    pub fn read_data_memory_u16(&mut self, addr: u16) -> Result<u16, Error<E>> {
        self.select_data_memory(addr)?;

        let mut buf = [0u8; 2];

        self.i2c.read(self.address, &mut buf).map_err(Error::I2c)?;

        Ok(u16::from_le_bytes(buf))
    }

    /// Write data memory with automatic checksum calculation
    pub fn write_data_memory(
        &mut self,
        addr: u16,
        data: u16,
        no_bytes: u8,
    ) -> Result<(), Error<E>> {
        let addr_l = (addr & 0xFF) as u8;
        let addr_h = ((addr >> 8) & 0xFF) as u8;
        let data_l = (data & 0xFF) as u8;
        let data_h = ((data >> 8) & 0xFF) as u8;

        let mut chk = 0;
        chk = Self::calculate_checksum(chk, addr_l);
        chk = Self::calculate_checksum(chk, addr_h);
        chk = Self::calculate_checksum(chk, data_l);
        if no_bytes == 2 {
            chk = Self::calculate_checksum(chk, data_h);
        }

        // Enter configuration update mode first
        self.sub_command(0x0090)?;

        // Write subcommand address and data
        let mut write_buf = [0u8; 5];
        write_buf[0] = CMD_DIR_SUBCMD_LOW;
        write_buf[1] = addr_l;
        write_buf[2] = addr_h;
        write_buf[3] = data_l;
        let len = if no_bytes == 2 {
            write_buf[4] = data_h;
            5
        } else {
            4
        };
        self.i2c
            .write(self.address, &write_buf[..len])
            .map_err(Error::I2c)?;

        // Write checksum and size indicator
        let check_buf = [
            CMD_DIR_RESP_CHKSUM,
            chk,
            if no_bytes == 1 { 0x05 } else { 0x06 },
        ];
        self.i2c
            .write(self.address, &check_buf)
            .map_err(Error::I2c)?;

        // Exit configuration update mode
        self.sub_command(0x0092)?;

        Ok(())
    }

    /// Reset the device
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.sub_command(0x0012)
    }

    /// Check connection status via device number direct command
    pub fn is_connected(&mut self) -> bool {
        self.direct_command(0x01).is_ok()
    }

    /// Get individual cell voltage (cell_number 1..16)
    pub fn get_cell_voltage(&mut self, cell_number: u8) -> Result<u16, Error<E>> {
        let cmd = 0x14 + (cell_number - 1) * 2;
        self.direct_command(cmd)
    }

    /// Get stack voltage
    pub fn get_stack_voltage(&mut self) -> Result<u16, Error<E>> {
        self.direct_command(0x34)
    }

    /// Get current (mA)
    pub fn get_current(&mut self) -> Result<i16, Error<E>> {
        Ok(self.direct_command(0x3A)? as i16)
    }

    /// Set Cell Overvoltage Protection limits
    pub fn set_cell_overvoltage_protection(&mut self, mv: u16, ms: u16) -> Result<(), Error<E>> {
        let thresh = (mv as f32 / 50.6).clamp(20.0, 110.0) as u8;
        let dly = ((ms as f32 / 3.3) - 2.0).clamp(1.0, 2047.0) as u16;

        self.write_data_memory(0x9278, thresh as u16, 1)?;
        self.write_data_memory(0x9279, dly, 2)?;
        Ok(())
    }

    /// Set Cell Undervoltage Protection limits
    pub fn set_cell_undervoltage_protection(&mut self, mv: u16, ms: u16) -> Result<(), Error<E>> {
        let thresh = (mv as f32 / 50.6).clamp(10.0, 90.0) as u8;
        let dly = ((ms as f32 / 3.3) - 2.0).clamp(1.0, 2047.0) as u16;

        self.write_data_memory(0x9275, thresh as u16, 1)?;
        self.write_data_memory(0x9276, dly, 2)?;
        Ok(())
    }

    /// Configure pre-regulator
    pub fn set_enable_pre_regulator(&mut self) -> Result<(), Error<E>> {
        self.write_data_memory(REG0_CONFIG, 0x01, 1)
    }

    /// Set protection configurations securely
    pub fn set_protection_configuration(&mut self) -> Result<(), Error<E>> {
        self.write_data_memory(PROTECTION_CONFIGURATION, 0x0600, 2)
    }

    /// Control FET states directly
    pub fn set_fet(&mut self, fet: Fet, state: FetState) -> Result<(), Error<E>> {
        let sub_cmd = match state {
            FetState::Off => match fet {
                Fet::Dch => 0x0093,
                Fet::Chg => 0x0094,
                Fet::All => 0x0095,
            },

            FetState::On => 0x0096,
        };

        self.sub_command(sub_cmd)
    }

    /// Get internal die temperature
    pub fn get_internal_temperature(&mut self) -> Result<i16, Error<E>> {
        self.direct_command(0x68).map(|val| val as i16)
    }

    /// Get thermistor temperature (TS1, TS2, TS3)
    pub fn get_temperature(&mut self, thermistor: Thermistor) -> Result<i16, Error<E>> {
        let cmd = match thermistor {
            Thermistor::Ts1 => 0x70,
            Thermistor::Ts2 => 0x72,
            Thermistor::Ts3 => 0x74,
            _ => 0x70, // Fallback or handle other variants if needed
        };
        self.direct_command(cmd).map(|val| val as i16)
    }

    /// Read Battery Status flags
    pub fn get_battery_status(&mut self) -> Result<BatteryStatus, Error<E>> {
        let raw = self.direct_command(0x12)?;
        Ok(BatteryStatus::from(raw))
    }

    /// Unseal the device using security keys
    pub fn unseal(&mut self, key_step1: u16, key_step2: u16) -> Result<(), Error<E>> {
        self.sub_command(key_step1)?;
        self.sub_command(key_step2)?;
        Ok(())
    }

    /// Read all 16 cell voltages into a fixed-size array at once
    pub fn get_all_cell_voltages(&mut self) -> Result<[u16; 16], Error<E>> {
        let mut voltages = [0u16; 16];
        #[allow(clippy::needless_range_loop)]
        for i in 0..16 {
            voltages[i] = self.get_cell_voltage((i + 1) as u8)?;
        }
        Ok(voltages)
    }

    /// Get Minimum Cell Voltage (Direct command 0x58 or similar depending on map)
    pub fn get_min_cell_voltage(&mut self) -> Result<u16, Error<E>> {
        self.direct_command(0x58)
    }

    /// Get Maximum Cell Voltage
    pub fn get_max_cell_voltage(&mut self) -> Result<u16, Error<E>> {
        self.direct_command(0x5A)
    }

    /// Read and parse Safety Status A flags
    pub fn get_safety_status_a(&mut self) -> Result<SafetyStatusA, Error<E>> {
        let raw = self.direct_command(0x03)?;
        Ok(SafetyStatusA::from(raw))
    }

    /// Read and parse Alarm Status flags
    pub fn get_alarm_status(&mut self) -> Result<AlarmStatus, Error<E>> {
        let raw = self.direct_command(0x60)?;
        Ok(AlarmStatus::from(raw))
    }

    /// Get internal die temperature in degrees Celsius (f32)
    pub fn get_internal_temperature_celsius(&mut self) -> Result<f32, Error<E>> {
        let raw = self.get_internal_temperature()?;
        // BQ76952 returns Kelvin in 0.1K steps -> Convert to Celsius: (K - 2731.5) / 10.0
        Ok((raw as f32 - 2731.5) / 10.0)
    }

    /// Get thermistor temperature (TS1, TS2, TS3) in degrees Celsius (f32)
    pub fn get_temperature_celsius(&mut self, thermistor: Thermistor) -> Result<f32, Error<E>> {
        let raw = self.get_temperature(thermistor)?;
        Ok((raw as f32 - 2731.5) / 10.0)
    }

    /// Get the current device security state (Sealed, Unsealed, FullAccess)
    pub fn get_security_state(&mut self) -> Result<SecurityState, Error<E>> {
        let status = self.get_battery_status()?;
        Ok(status.security_state)
    }

    /// Read and parse Safety Status B flags (Register 0x05)
    pub fn get_safety_status_b(&mut self) -> Result<SafetyStatusB, Error<E>> {
        let raw = self.direct_command(0x05)?;
        Ok(SafetyStatusB::from(raw))
    }

    /// Read and parse Safety Status C flags (Register 0x06)
    pub fn get_safety_status_c(&mut self) -> Result<SafetyStatusC, Error<E>> {
        let raw = self.direct_command(0x06)?;
        Ok(SafetyStatusC::from(raw))
    }

    /// Request the device to enter Sleep mode
    pub fn enter_sleep_mode(&mut self) -> Result<(), Error<E>> {
        self.sub_command(0x0099)
    }

    /// Request the device to exit Sleep mode (wake up)
    pub fn exit_sleep_mode(&mut self) -> Result<(), Error<E>> {
        self.sub_command(0x009A)
    }

    /// Request the device to enter Shutdown mode
    pub fn enter_shutdown_mode(&mut self) -> Result<(), Error<E>> {
        // BQ76952 requires sending shutdown command twice or a specific subcommand sequence
        // Subcommand 0x0014 is Shutdown
        self.sub_command(0x0014)
    }

    /// Read and parse Permanent Faults status flags
    pub fn get_permanent_faults(&mut self) -> Result<PermanentFaults, Error<E>> {
        let raw = self.direct_command(0x6E)?; // Register 0x6E / standard PF status location
        Ok(PermanentFaults::from(raw))
    }

    /// Manually enable or configure cell balancing using active cells subcommand (0x0083).
    /// The BQ76952 expects the 2-byte mask (where bits 0-15 correspond to cells 1-16)
    /// to be written using the subcommand data registers (`0x3E`/`0x3F`) followed by issuing `0x0083`.
    pub fn set_cell_balancing(&mut self, mask: u16) -> Result<(), Error<E>> {
        let low_byte = (mask & 0xFF) as u8;
        let high_byte = ((mask >> 8) & 0xFF) as u8;

        // Write subcommand code (0x0083) and the 2-byte mask payload to the command buffer area
        let bytes = [
            CMD_DIR_SUBCMD_LOW,
            (SUBCMD_CB_ACTIVE_CELLS & 0xFF) as u8,
            ((SUBCMD_CB_ACTIVE_CELLS >> 8) & 0xFF) as u8,
            low_byte,
            high_byte,
        ];
        self.i2c.write(self.address, &bytes).map_err(Error::I2c)
    }
}
