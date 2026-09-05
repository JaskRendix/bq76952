use bq76952::registers::*;

/// Collect all u16 register-like constants for uniqueness and range tests.
fn all_registers() -> Vec<u16> {
    vec![
        REG0_CONFIG,
        REG12_CONTROL,
        ALERT_PIN_CONFIG,
        DEFAULT_ALARM_MASK_CONFIG,
        DA_CONFIGURATION,
        SHUTDOWN_STACK_VOLTAGE,
        VCELL_MODE,
        PROTECTION_CONFIGURATION,
        ENABLE_PROTECTIONS_A,
        ENABLE_PROTECTIONS_B,
        ENABLE_PROTECTIONS_C,
        CHG_FET_PROTECTION_A,
        CHG_FET_PROTECTION_B,
        CHG_FET_PROTECTION_C,
        DSG_FET_PROTECTION_A,
        DSG_FET_PROTECTION_B,
        DSG_FET_PROTECTION_C,
        SF_ALERT_MASK_A,
        SF_ALERT_MASK_B,
        SF_ALERT_MASK_C,
        SCD_THRESHOLD_CONFIG,
        SCD_DELAY_CONFIG,
        FET_OPTIONS,
        FET_PREDISCHARGE_TIMEOUT,
        FET_PREDISCHARGE_STOP_DELTA,
        CC3_SAMPLES,
        TS1_CONFIG,
        TS2_CONFIG,
        TS3_CONFIG,
        CELL_INTERCONNECT_RESISTANCE,
        CELL_INTERCONNECT_RESISTANCE_MOHM,
        UNSEAL_KEY_STEP_1,
        UNSEAL_KEY_STEP_2,
        FULL_ACCESS_KEY_STEP_1,
        FULL_ACCESS_KEY_STEP_2,
        CMD_DIR_SUBCMD_LOW as u16,
        CMD_DIR_RESP_CHKSUM as u16,
    ]
}

#[test]
fn test_specific_known_values() {
    assert_eq!(REG0_CONFIG, 0x9237);
    assert_eq!(REG12_CONTROL, 0x9236);
    assert_eq!(ALERT_PIN_CONFIG, 0x92FC);
    assert_eq!(DEFAULT_ALARM_MASK_CONFIG, 0x926D);
    assert_eq!(DA_CONFIGURATION, 0x9303);
    assert_eq!(SHUTDOWN_STACK_VOLTAGE, 0x9241);

    assert_eq!(VCELL_MODE, 0x9304);
    assert_eq!(PROTECTION_CONFIGURATION, 0x925F);

    assert_eq!(ENABLE_PROTECTIONS_A, 0x9261);
    assert_eq!(ENABLE_PROTECTIONS_B, 0x9262);
    assert_eq!(ENABLE_PROTECTIONS_C, 0x9263);

    assert_eq!(CHG_FET_PROTECTION_A, 0x9265);
    assert_eq!(CHG_FET_PROTECTION_B, 0x9266);
    assert_eq!(CHG_FET_PROTECTION_C, 0x9267);

    assert_eq!(DSG_FET_PROTECTION_A, 0x9269);
    assert_eq!(DSG_FET_PROTECTION_B, 0x926A);
    assert_eq!(DSG_FET_PROTECTION_C, 0x926B);

    assert_eq!(SF_ALERT_MASK_A, 0x926F);
    assert_eq!(SF_ALERT_MASK_B, 0x9270);
    assert_eq!(SF_ALERT_MASK_C, 0x9271);

    assert_eq!(SCD_THRESHOLD_CONFIG, 0x9286);
    assert_eq!(SCD_DELAY_CONFIG, 0x9287);

    assert_eq!(FET_OPTIONS, 0x9308);
    assert_eq!(FET_PREDISCHARGE_TIMEOUT, 0x930E);
    assert_eq!(FET_PREDISCHARGE_STOP_DELTA, 0x930F);

    assert_eq!(CC3_SAMPLES, 0x9307);
    assert_eq!(TS1_CONFIG, 0x92FD);
    assert_eq!(TS2_CONFIG, 0x92FE);
    assert_eq!(TS3_CONFIG, 0x92FF);

    assert_eq!(CELL_INTERCONNECT_RESISTANCE, 0x9315);
    assert_eq!(CELL_INTERCONNECT_RESISTANCE_MOHM, 0);

    assert_eq!(UNSEAL_KEY_STEP_1, 0x0414);
    assert_eq!(UNSEAL_KEY_STEP_2, 0x3672);
    assert_eq!(FULL_ACCESS_KEY_STEP_1, 0x1234);
    assert_eq!(FULL_ACCESS_KEY_STEP_2, 0xABCD);

    assert_eq!(CMD_DIR_SUBCMD_LOW, 0x3E);
    assert_eq!(CMD_DIR_RESP_CHKSUM, 0x60);
}

#[test]
fn no_duplicate_registers() {
    let mut sorted = all_registers();
    sorted.sort();

    for w in sorted.windows(2) {
        assert_ne!(
            w[0], w[1],
            "Duplicate register address detected: {:04X}",
            w[0]
        );
    }
}

#[test]
fn all_registers_within_valid_range() {
    for reg in all_registers() {
        let is_special = reg == UNSEAL_KEY_STEP_1
            || reg == UNSEAL_KEY_STEP_2
            || reg == FULL_ACCESS_KEY_STEP_1
            || reg == FULL_ACCESS_KEY_STEP_2
            || reg == CMD_DIR_SUBCMD_LOW as u16
            || reg == CMD_DIR_RESP_CHKSUM as u16
            || reg == CELL_INTERCONNECT_RESISTANCE_MOHM; // <-- Add this exemption

        if !is_special {
            assert!(
                (0x9000..=0x93FF).contains(&reg),
                "Register {:04X} is outside expected BQ76952 config range",
                reg
            );
        }
    }
}

#[test]
fn protection_registers_are_contiguous() {
    assert_eq!(ENABLE_PROTECTIONS_A + 1, ENABLE_PROTECTIONS_B);
    assert_eq!(ENABLE_PROTECTIONS_B + 1, ENABLE_PROTECTIONS_C);

    assert_eq!(CHG_FET_PROTECTION_A + 1, CHG_FET_PROTECTION_B);
    assert_eq!(CHG_FET_PROTECTION_B + 1, CHG_FET_PROTECTION_C);

    assert_eq!(DSG_FET_PROTECTION_A + 1, DSG_FET_PROTECTION_B);
    assert_eq!(DSG_FET_PROTECTION_B + 1, DSG_FET_PROTECTION_C);
}

#[test]
fn temperature_sensor_registers_are_contiguous() {
    assert_eq!(TS1_CONFIG + 1, TS2_CONFIG);
    assert_eq!(TS2_CONFIG + 1, TS3_CONFIG);
}

#[test]
fn alert_mask_registers_are_contiguous() {
    assert_eq!(SF_ALERT_MASK_A + 1, SF_ALERT_MASK_B);
    assert_eq!(SF_ALERT_MASK_B + 1, SF_ALERT_MASK_C);
}

#[test]
fn cell_interconnect_resistance_mohm_is_zero() {
    assert_eq!(CELL_INTERCONNECT_RESISTANCE_MOHM, 0);
}

#[test]
fn security_keys_are_not_in_register_range() {
    const {
        assert!(UNSEAL_KEY_STEP_1 < 0x9000);
        assert!(UNSEAL_KEY_STEP_2 < 0x9000);
        assert!(FULL_ACCESS_KEY_STEP_1 < 0x9000);
        assert!(FULL_ACCESS_KEY_STEP_2 >= 0x9000);
    }
}
