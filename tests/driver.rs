extern crate alloc;

use alloc::{vec, vec::Vec};

use bq76952::{
    Bq76952, Error, Fet, FetState, PROTECTION_CONFIGURATION, REG0_CONFIG, SecurityState, Thermistor,
};

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

fn calculate_checksum(old_checksum: u8, data: u8) -> u8 {
    let mut chk = old_checksum;
    if chk == 0 {
        chk = data;
    } else {
        chk = (!chk).wrapping_add(data);
    }
    !chk
}

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
fn test_read_data_memory_u8() {
    let mut i2c = I2cMock::new(&[
        I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56]),
        I2cTransaction::read(0x10, vec![0xAB]),
    ]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);
    let val = dev.read_data_memory_u8(0x5678).unwrap();

    assert_eq!(val, 0xAB);

    i2c.done();
}

#[test]
fn test_read_data_memory_u16() {
    let mut i2c = I2cMock::new(&[
        I2cTransaction::write(0x10, vec![0x3E, 0x78, 0x56]),
        I2cTransaction::read(0x10, vec![0xCD, 0xAB]),
    ]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);
    let val = dev.read_data_memory_u16(0x5678).unwrap();

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
        vec![0x3A],
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
    let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x96, 0x00])]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);
    dev.set_fet(Fet::Chg, FetState::On).unwrap();
    i2c.done();
}

#[test]
fn test_set_fet_all_off() {
    let mut i2c = I2cMock::new(&[I2cTransaction::write(0x10, vec![0x3E, 0x95, 0x00])]);

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
fn test_get_cell_voltage_invalid_cell() {
    let mut i2c = I2cMock::new(&[]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);

    assert!(matches!(
        dev.get_cell_voltage(0),
        Err(Error::InvalidParameter)
    ));

    i2c.done();
}

#[test]
fn test_get_cell_voltage_invalid_cell_high() {
    let mut i2c = I2cMock::new(&[]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);

    assert!(matches!(
        dev.get_cell_voltage(17),
        Err(Error::InvalidParameter)
    ));

    i2c.done();
}

#[test]
fn test_get_battery_status() {
    // Direct command 0x12 returning a mock status word
    let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
        0x10,
        vec![0x12],
        vec![0x00, 0x80], // sleep_mode = true
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
        vec![0x03],
        vec![0x10, 0x00],
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
        vec![0x62],
        vec![0x00, 0x20],
    )]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);
    let alarm = dev.get_alarm_status().unwrap();

    assert!(alarm.pf);

    i2c.done();
}

#[test]
fn test_write_data_memory_2_bytes_checksum() {
    let addr = 0x1234u16;
    let data = 0xABCDu16;
    let mut chk = 0;
    chk = calculate_checksum(chk, (addr & 0xFF) as u8);
    chk = calculate_checksum(chk, ((addr >> 8) & 0xFF) as u8);
    chk = calculate_checksum(chk, (data & 0xFF) as u8);
    chk = calculate_checksum(chk, ((data >> 8) & 0xFF) as u8);

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
    chk1 = calculate_checksum(chk1, (addr_thresh & 0xFF) as u8);
    chk1 = calculate_checksum(chk1, ((addr_thresh >> 8) & 0xFF) as u8);
    chk1 = calculate_checksum(chk1, thresh_val);

    let mut chk2 = 0;
    chk2 = calculate_checksum(chk2, (addr_delay & 0xFF) as u8);
    chk2 = calculate_checksum(chk2, ((addr_delay >> 8) & 0xFF) as u8);
    chk2 = calculate_checksum(chk2, (delay_val & 0xFF) as u8);
    chk2 = calculate_checksum(chk2, ((delay_val >> 8) & 0xFF) as u8);

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
    chk = calculate_checksum(chk, addr_l);
    chk = calculate_checksum(chk, addr_h);
    chk = calculate_checksum(chk, data);

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
    chk = calculate_checksum(chk, addr_l);
    chk = calculate_checksum(chk, addr_h);
    chk = calculate_checksum(chk, data_l);
    chk = calculate_checksum(chk, data_h);

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
        vec![0x00, 0x02],
    )]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);
    let sec = dev.get_security_state().unwrap();
    assert_eq!(sec, SecurityState::Unsealed);
    i2c.done();
}

#[test]
fn test_get_safety_status_b() {
    let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
        0x10,
        vec![0x05],
        vec![0x01, 0x00],
    )]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);
    let status = dev.get_safety_status_b().unwrap();

    assert!(status.utc);

    i2c.done();
}

#[test]
fn test_get_safety_status_c() {
    let mut i2c = I2cMock::new(&[I2cTransaction::write_read(
        0x10,
        vec![0x07],
        vec![0x04, 0x00],
    )]);

    let mut dev = Bq76952::new(&mut i2c, 0x10);
    let status = dev.get_safety_status_c().unwrap();
    assert!(status.pto);
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
        vec![0x0B],
        vec![0x40, 0x00],
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
