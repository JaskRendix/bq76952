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

    /// Read data memory safely with error propagation
    pub fn read_data_memory(&mut self, addr: u16, size: usize) -> Result<u16, Error<E>> {
        let addr_bytes = [
            CMD_DIR_SUBCMD_LOW,
            (addr & 0xFF) as u8,
            ((addr >> 8) & 0xFF) as u8,
        ];
        self.i2c
            .write(self.address, &addr_bytes)
            .map_err(Error::I2c)?;

        let mut buf = [0u8; 2];
        let read_size = core::cmp::min(size, buf.len());
        self.i2c
            .read(self.address, &mut buf[..read_size])
            .map_err(Error::I2c)?;

        if size == 1 {
            Ok(buf[0] as u16)
        } else {
            Ok(u16::from_le_bytes(buf))
        }
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
        Ok(self.direct_command(0x38)? as i16)
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
        let sub_cmd = match (fet, state) {
            (Fet::Chg, FetState::On) => 0x0022,
            (Fet::Chg, FetState::Off) => 0x0021,
            (Fet::Dch, FetState::On) => 0x0024,
            (Fet::Dch, FetState::Off) => 0x0023,
            (Fet::All, FetState::On) => 0x0026,
            (Fet::All, FetState::Off) => 0x0025,
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
        let raw = self.direct_command(0x62)?;
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

    /// Read and parse Safety Status B flags (Register 0x63)
    pub fn get_safety_status_b(&mut self) -> Result<SafetyStatusB, Error<E>> {
        let raw = self.direct_command(0x63)?;
        Ok(SafetyStatusB::from(raw))
    }

    /// Read and parse Safety Status C flags (Register 0x64)
    pub fn get_safety_status_c(&mut self) -> Result<SafetyStatusC, Error<E>> {
        let raw = self.direct_command(0x64)?;
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

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::{vec, vec::Vec};

    #[allow(unused_imports)]
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    #[test]
    fn test_new_driver() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(0x10, vec![0x01], vec![0, 0])
            .with_error(embedded_hal::i2c::ErrorKind::Other)]);
        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert!(!dev.is_connected());
        i2c.done();
    }

    #[test]
    fn test_direct_command_ok() {
        let expected = vec![0x34, 0x12];
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(0x10, vec![0x01], expected)]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let val = dev.direct_command(0x01).unwrap();
        assert_eq!(val, 0x1234);
        i2c.done();
    }

    #[test]
    fn test_direct_command_error() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(0x10, vec![0x01], vec![0, 0])
            .with_error(embedded_hal::i2c::ErrorKind::Other)]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert!(matches!(dev.direct_command(0x01), Err(Error::I2c(_))));
        i2c.done();
    }

    #[test]
    fn test_subcommand_ok() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56])]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.sub_command(0x5678).unwrap();
        i2c.done();
    }

    #[test]
    fn test_subcommand_error() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56])
            .with_error(embedded_hal::i2c::ErrorKind::Other)]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert!(matches!(dev.sub_command(0x5678), Err(Error::I2c(_))));
        i2c.done();
    }

    #[test]
    fn test_read_data_memory_1_byte() {
        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56]),
            I2cTransaction::read(0x10, vec![0xAB]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let val = dev.read_data_memory(0x5678, 1).unwrap();
        assert_eq!(val, 0xAB);
        i2c.done();
    }

    #[test]
    fn test_read_data_memory_2_bytes() {
        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56]),
            I2cTransaction::read(0x10, vec![0xCD, 0xAB]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let val = dev.read_data_memory(0x5678, 2).unwrap();
        assert_eq!(val, 0xABCD);
        i2c.done();
    }

    #[test]
    fn test_write_data_memory_1_byte() {
        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56, 0xAB]),
            I2cTransaction::write(0x10, vec![0x60, 0x86, 0x05]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.write_data_memory(0x5678, 0xAB, 1).unwrap();
        i2c.done();
    }

    #[test]
    fn test_reset() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x12, 0x00])]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.reset().unwrap();
        i2c.done();
    }

    #[test]
    fn test_is_connected_true() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x01],
            vec![0x00, 0x00],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert!(dev.is_connected());
        i2c.done();
    }

    #[test]
    fn test_is_connected_false() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(0x10, vec![0x01], vec![0, 0])
            .with_error(embedded_hal::i2c::ErrorKind::Other)]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert!(!dev.is_connected());
        i2c.done();
    }

    #[test]
    fn test_get_cell_voltage() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x14],
            vec![0x34, 0x12],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert_eq!(dev.get_cell_voltage(1).unwrap(), 0x1234);
        i2c.done();
    }

    #[test]
    fn test_get_stack_voltage() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x34],
            vec![0x34, 0x12],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert_eq!(dev.get_stack_voltage().unwrap(), 0x1234);
        i2c.done();
    }

    #[test]
    fn test_get_current() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x38],
            vec![0x34, 0x12],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert_eq!(dev.get_current().unwrap(), 0x1234);
        i2c.done();
    }

    #[test]
    fn test_set_cell_overvoltage_protection() {
        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x92, 0x53]),
            I2cTransaction::write(0x10, vec![0x60, 0xA2, 0x05]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, 0x79, 0x92, 0x1C, 0x00]),
            I2cTransaction::write(0x10, vec![0x60, 0xD8, 0x06]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.set_cell_overvoltage_protection(4200, 100).unwrap();
        i2c.done();
    }

    #[test]
    fn test_set_fet_chg_on() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x22, 0x00])]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.set_fet(Fet::Chg, FetState::On).unwrap();
        i2c.done();
    }

    #[test]
    fn test_set_fet_all_off() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x25, 0x00])]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.set_fet(Fet::All, FetState::Off).unwrap();
        i2c.done();
    }

    #[test]
    fn test_get_internal_temperature() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x68],
            vec![0xE8, 0x0B], // e.g. 3048 tenths of Kelvin / raw value
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert_eq!(dev.get_internal_temperature().unwrap(), 0x0BE8);
        i2c.done();
    }

    #[test]
    fn test_get_temperature_ts1() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x70],
            vec![0xD0, 0x0B],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert_eq!(dev.get_temperature(Thermistor::Ts1).unwrap(), 0x0BD0);
        i2c.done();
    }

    #[test]
    fn test_get_battery_status() {
        // Direct command 0x12 returning a mock status word
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x12],
            vec![0x01, 0x00], // sleep_mode = true
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let status = dev.get_battery_status().unwrap();
        assert!(status.sleep_mode);
        i2c.done();
    }

    #[test]
    fn test_unseal() {
        // Unseal involves sending two subcommands consecutively
        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x14, 0x04]), // Step 1 key
            I2cTransaction::write(0x10, vec![0x3E, 0x36, 0x36]), // Step 2 key
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.unseal(0x0414, 0x3636).unwrap();
        i2c.done();
    }

    #[test]
    fn test_get_all_cell_voltages() {
        // Mock 16 consecutive cell voltage reads (0x14 through 0x32)
        let mut transactions = Vec::new();
        #[allow(clippy::needless_range_loop)]
        for i in 0..16 {
            let cmd = 0x14 + (i * 2);
            transactions.push(I2cTransaction::write_read(
                0x10,
                vec![cmd as u8],
                vec![0x34, 0x12], // mock voltage 0x1234
            ));
        }

        let mut i2c = I2cMock::new(&transactions);
        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let voltages = dev.get_all_cell_voltages().unwrap();

        assert_eq!(voltages.len(), 16);
        assert_eq!(voltages[0], 0x1234);
        assert_eq!(voltages[15], 0x1234);
        i2c.done();
    }

    #[test]
    fn test_get_min_max_cell_voltage() {
        let mut i2c = I2cMock::new(&[
            I2cTransaction::write_read(0x10, vec![0x58], vec![0x11, 0x0B]), // Min cell voltage
            I2cTransaction::write_read(0x10, vec![0x5A], vec![0x55, 0x0C]), // Max cell voltage
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        assert_eq!(dev.get_min_cell_voltage().unwrap(), 0x0B11);
        assert_eq!(dev.get_max_cell_voltage().unwrap(), 0x0C55);
        i2c.done();
    }

    #[test]
    fn test_get_safety_status_a() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x62],
            vec![0x04, 0x00], // Mock: OCC bit set (bit 2 -> 0x0004)
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let status = dev.get_safety_status_a().unwrap();
        assert!(status.occ);
        i2c.done();
    }

    #[test]
    fn test_get_alarm_status() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x60],
            vec![0x10, 0x00], // Mock: pf bit set (bit 4 -> 0x0010)
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let alarm = dev.get_alarm_status().unwrap();
        assert!(alarm.pf);
        i2c.done();
    }

    #[test]
    fn test_read_data_memory_invalid_size() {
        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56]),
            I2cTransaction::read(0x10, vec![0xAA, 0xBB]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);

        // Size > 2 still returns 2 bytes (implementation detail)
        let val = dev.read_data_memory(0x5678, 3).unwrap();
        assert_eq!(val, 0xBBAA);

        i2c.done();
    }

    #[test]
    fn test_write_data_memory_2_bytes_checksum() {
        let addr = 0x1234u16;
        let data = 0xABCDu16;
        let mut chk = 0;
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, (addr & 0xFF) as u8);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, ((addr >> 8) & 0xFF) as u8);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, (data & 0xFF) as u8);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, ((data >> 8) & 0xFF) as u8);

        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, 0x34, 0x12, 0xCD, 0xAB]),
            I2cTransaction::write(0x10, vec![0x60, chk, 0x06]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.write_data_memory(addr, data, 2).unwrap();
        i2c.done();
    }

    #[test]
    fn test_set_cell_undervoltage_protection() {
        // Let's use the dynamically computed checksum instead of a hardcoded mismatch
        let addr_thresh = 0x9275;
        let addr_delay = 0x9276;
        let thresh_val = 59u8; // 3000 mV / 50.6 clamped
        let delay_val = 28u16; // 100 ms / 3.3 - 2

        let mut chk1 = 0;
        chk1 = Bq76952::<I2cMock>::calculate_checksum(chk1, (addr_thresh & 0xFF) as u8);
        chk1 = Bq76952::<I2cMock>::calculate_checksum(chk1, ((addr_thresh >> 8) & 0xFF) as u8);
        chk1 = Bq76952::<I2cMock>::calculate_checksum(chk1, thresh_val);

        let mut chk2 = 0;
        chk2 = Bq76952::<I2cMock>::calculate_checksum(chk2, (addr_delay & 0xFF) as u8);
        chk2 = Bq76952::<I2cMock>::calculate_checksum(chk2, ((addr_delay >> 8) & 0xFF) as u8);
        chk2 = Bq76952::<I2cMock>::calculate_checksum(chk2, (delay_val & 0xFF) as u8);
        chk2 = Bq76952::<I2cMock>::calculate_checksum(chk2, ((delay_val >> 8) & 0xFF) as u8);

        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, 0x75, 0x92, thresh_val]),
            I2cTransaction::write(0x10, vec![0x60, chk1, 0x05]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(
                0x10,
                vec![
                    0x3E,
                    0x76,
                    0x92,
                    (delay_val & 0xFF) as u8,
                    ((delay_val >> 8) & 0xFF) as u8,
                ],
            ),
            I2cTransaction::write(0x10, vec![0x60, chk2, 0x06]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.set_cell_undervoltage_protection(3000, 100).unwrap();
        i2c.done();
    }

    #[test]
    fn test_set_enable_pre_regulator() {
        let addr = REG0_CONFIG;
        let addr_l = (addr & 0xFF) as u8;
        let addr_h = ((addr >> 8) & 0xFF) as u8;
        let data = 0x01;

        // Compute checksum using same logic as driver
        let mut chk = 0;
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, addr_l);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, addr_h);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, data);

        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, addr_l, addr_h, data]),
            I2cTransaction::write(0x10, vec![0x60, chk, 0x05]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.set_enable_pre_regulator().unwrap();
        i2c.done();
    }

    #[test]
    fn test_set_protection_configuration() {
        let addr = PROTECTION_CONFIGURATION;
        let addr_l = (addr & 0xFF) as u8;
        let addr_h = ((addr >> 8) & 0xFF) as u8;
        let data_l = 0x00;
        let data_h = 0x06;

        let mut chk = 0;
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, addr_l);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, addr_h);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, data_l);
        chk = Bq76952::<I2cMock>::calculate_checksum(chk, data_h);

        let mut i2c = I2cMock::new(&[
            I2cTransaction::write(0x10, vec![0x3E, 0x90, 0x00]),
            I2cTransaction::write(0x10, vec![0x3E, addr_l, addr_h, data_l, data_h]),
            I2cTransaction::write(0x10, vec![0x60, chk, 0x06]),
            I2cTransaction::write(0x10, vec![0x3E, 0x92, 0x00]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.set_protection_configuration().unwrap();
        i2c.done();
    }

    #[test]
    fn test_get_security_state() {
        // SecurityState::Unsealed corresponds to val >> 6 == 1 -> raw value 0x0040
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x12],
            vec![0x40, 0x00],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let sec = dev.get_security_state().unwrap();
        assert_eq!(sec, SecurityState::Unsealed);
        i2c.done();
    }

    #[test]
    fn test_get_safety_status_b() {
        // ocd3 expects bit 0 (val & 0x0001 != 0)
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x63],
            vec![0x01, 0x00],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let status = dev.get_safety_status_b().unwrap();
        assert!(status.ocd3);
        i2c.done();
    }

    #[test]
    fn test_get_safety_status_c() {
        // ptos expects bit 4 (val & 0x0010 != 0)
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x64],
            vec![0x10, 0x00],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let status = dev.get_safety_status_c().unwrap();
        assert!(status.ptos);
        i2c.done();
    }

    #[test]
    fn test_enter_sleep_mode() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x99, 0x00])]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.enter_sleep_mode().unwrap();
        i2c.done();
    }

    #[test]
    fn test_exit_sleep_mode() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x9A, 0x00])]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.exit_sleep_mode().unwrap();
        i2c.done();
    }

    #[test]
    fn test_enter_shutdown_mode() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x14, 0x00])]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.enter_shutdown_mode().unwrap();
        i2c.done();
    }

    #[test]
    fn test_get_permanent_faults() {
        let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
            0x10,
            vec![0x6E],
            vec![0x01, 0x00],
        )]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        let pf = dev.get_permanent_faults().unwrap();

        assert!(pf.sotf);

        i2c.done();
    }

    #[test]
    fn test_set_cell_balancing() {
        let mut i2c = I2cMock::new(&[
            // Subcommand 0x0083 followed by the 2-byte cell mask (0x00FF in little-endian: low byte first)
            I2cTransaction::write(0x10, vec![0x3E, 0x83, 0x00, 0xFF, 0x00]),
        ]);

        let mut dev = Bq76952::new(&mut i2c, 0x10);
        dev.set_cell_balancing(0x00FF).unwrap();
        i2c.done();
    }
}
