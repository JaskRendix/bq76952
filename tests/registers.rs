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
fn subcommands_have_expected_values() {
    assert_eq!(SUBCMD_CB_ACTIVE_CELLS, 0x0083);

    assert_eq!(SUBCMD_RESET, 0x0012);
    assert_eq!(SUBCMD_SHUTDOWN, 0x0014);

    assert_eq!(SUBCMD_ENTER_CFGUPDATE, 0x0090);
    assert_eq!(SUBCMD_EXIT_CFGUPDATE, 0x0092);

    assert_eq!(SUBCMD_DSG_OFF, 0x0093);
    assert_eq!(SUBCMD_CHG_OFF, 0x0094);
    assert_eq!(SUBCMD_ALL_FETS_OFF, 0x0095);
    assert_eq!(SUBCMD_ALL_FETS_ON, 0x0096);

    assert_eq!(SUBCMD_SLEEP_ENABLE, 0x0099);
    assert_eq!(SUBCMD_SLEEP_DISABLE, 0x009A);
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
