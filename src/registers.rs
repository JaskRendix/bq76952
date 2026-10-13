//! BQ76952 Register Addresses, Subcommands, and Constants

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

// Cell UV Protection
pub const CUV_THRESHOLD: u16 = 0x9275;
pub const CUV_DELAY: u16 = 0x9276;

// Cell OV Protection
pub const COV_THRESHOLD: u16 = 0x9278;
pub const COV_DELAY: u16 = 0x9279;

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

// ======= Command-Only Subcommands =======
pub const SUBCMD_RESET: u16 = 0x0012;
pub const SUBCMD_SHUTDOWN: u16 = 0x0014;

// Configuration Update
pub const SUBCMD_ENTER_CFGUPDATE: u16 = 0x0090;
pub const SUBCMD_EXIT_CFGUPDATE: u16 = 0x0092;

// FET Control
pub const SUBCMD_DSG_OFF: u16 = 0x0093;
pub const SUBCMD_CHG_OFF: u16 = 0x0094;
pub const SUBCMD_ALL_FETS_OFF: u16 = 0x0095;
pub const SUBCMD_ALL_FETS_ON: u16 = 0x0096;

// Power Modes
pub const SUBCMD_SLEEP_ENABLE: u16 = 0x0099;
pub const SUBCMD_SLEEP_DISABLE: u16 = 0x009A;

// ======= Direct Commands =======

// Status
pub const CMD_CONTROL_STATUS: u8 = 0x00;
pub const CMD_DEVICE_NUMBER: u8 = 0x01;

pub const CMD_SAFETY_ALERT_A: u8 = 0x02;
pub const CMD_SAFETY_STATUS_A: u8 = 0x03;

pub const CMD_SAFETY_ALERT_B: u8 = 0x04;
pub const CMD_SAFETY_STATUS_B: u8 = 0x05;

pub const CMD_SAFETY_ALERT_C: u8 = 0x06;
pub const CMD_SAFETY_STATUS_C: u8 = 0x07;

pub const CMD_PF_ALERT_A: u8 = 0x0A;
pub const CMD_PF_STATUS_A: u8 = 0x0B;

pub const CMD_PF_ALERT_B: u8 = 0x0C;
pub const CMD_PF_STATUS_B: u8 = 0x0D;

pub const CMD_PF_ALERT_C: u8 = 0x0E;
pub const CMD_PF_STATUS_C: u8 = 0x0F;

pub const CMD_PF_ALERT_D: u8 = 0x10;
pub const CMD_PF_STATUS_D: u8 = 0x11;

pub const CMD_BATTERY_STATUS: u8 = 0x12;

// Voltage
pub const CMD_CELL1_VOLTAGE: u8 = 0x14;
pub const CMD_STACK_VOLTAGE: u8 = 0x34;
pub const CMD_PACK_PIN_VOLTAGE: u8 = 0x36;
pub const CMD_LD_PIN_VOLTAGE: u8 = 0x38;

pub const CMD_MIN_CELL_VOLTAGE: u8 = 0x58;
pub const CMD_MAX_CELL_VOLTAGE: u8 = 0x5A;

// Current
pub const CMD_CC2_CURRENT: u8 = 0x3A;

// Alarm
pub const CMD_ALARM_STATUS: u8 = 0x62;
pub const CMD_ALARM_RAW_STATUS: u8 = 0x64;
pub const CMD_ALARM_ENABLE: u8 = 0x66;

// Temperature
pub const CMD_INT_TEMPERATURE: u8 = 0x68;
pub const CMD_CFETOFF_TEMPERATURE: u8 = 0x6A;
pub const CMD_DFETOFF_TEMPERATURE: u8 = 0x6C;
pub const CMD_ALERT_TEMPERATURE: u8 = 0x6E;

pub const CMD_TS1_TEMPERATURE: u8 = 0x70;
pub const CMD_TS2_TEMPERATURE: u8 = 0x72;
pub const CMD_TS3_TEMPERATURE: u8 = 0x74;

pub const CMD_HDQ_TEMPERATURE: u8 = 0x76;
pub const CMD_DCHG_TEMPERATURE: u8 = 0x78;
pub const CMD_DDSG_TEMPERATURE: u8 = 0x7A;

// FET
pub const CMD_FET_STATUS: u8 = 0x7F;
