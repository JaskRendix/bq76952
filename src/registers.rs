//! BQ76952 Register Addresses, Subcommands, and Constants

#![allow(dead_code)]

// ======= BQ76952 Direct Commands / Regulators / Alerts =======
pub const REG0_CONFIG: u16 = 0x9237;
pub const REG12_CONTROL: u16 = 0x9236;
pub const ALERT_PIN_CONFIG: u16 = 0x92FC;
pub const DEFAULT_ALARM_MASK_CONFIG: u16 = 0x926D;
pub const DA_CONFIGURATION: u16 = 0x9303;
pub const SHUTDOWN_STACK_VOLTAGE: u16 = 0x9241;

// ======= VCell & Protection Configurations =======
pub const VCELL_MODE: u16 = 0x9304;
pub const PROTECTION_CONFIGURATION: u16 = 0x925F;
pub const ENABLE_PROTECTIONS_A: u16 = 0x9261;
pub const ENABLE_PROTECTIONS_B: u16 = 0x9262;
pub const ENABLE_PROTECTIONS_C: u16 = 0x9263;

pub const CHG_FET_PROTECTION_A: u16 = 0x9265;
pub const CHG_FET_PROTECTION_B: u16 = 0x9266;
pub const CHG_FET_PROTECTION_C: u16 = 0x9267;

pub const DSG_FET_PROTECTION_A: u16 = 0x9269;
pub const DSG_FET_PROTECTION_B: u16 = 0x926A;
pub const DSG_FET_PROTECTION_C: u16 = 0x926B;

pub const SF_ALERT_MASK_A: u16 = 0x926F;
pub const SF_ALERT_MASK_B: u16 = 0x9270;
pub const SF_ALERT_MASK_C: u16 = 0x9271;

pub const SCD_THRESHOLD_CONFIG: u16 = 0x9286;
pub const SCD_DELAY_CONFIG: u16 = 0x9287;
pub const FET_OPTIONS: u16 = 0x9308;
pub const FET_PREDISCHARGE_TIMEOUT: u16 = 0x930E;
pub const FET_PREDISCHARGE_STOP_DELTA: u16 = 0x930F;

pub const CC3_SAMPLES: u16 = 0x9307;
pub const TS1_CONFIG: u16 = 0x92FD;
pub const TS2_CONFIG: u16 = 0x92FE;
pub const TS3_CONFIG: u16 = 0x92FF;

pub const CELL_INTERCONNECT_RESISTANCE: u16 = 0x9315;
pub const CELL_INTERCONNECT_RESISTANCE_MOHM: u16 = 0;

// ======= Subcommands =======
pub const SUBCMD_CB_ACTIVE_CELLS: u16 = 0x0083;

// ======= Security Keys =======
pub const UNSEAL_KEY_STEP_1: u16 = 0x0414;
pub const UNSEAL_KEY_STEP_2: u16 = 0x3672;
pub const FULL_ACCESS_KEY_STEP_1: u16 = 0x1234;
pub const FULL_ACCESS_KEY_STEP_2: u16 = 0xABCD;

// ======= Direct Command / Response Registers =======
pub const CMD_DIR_SUBCMD_LOW: u8 = 0x3E;
pub const CMD_DIR_RESP_CHKSUM: u8 = 0x60;
