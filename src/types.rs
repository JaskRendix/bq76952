//! BQ76952 Types, Enums, and Status Bitfields

/// Device Security State representation based on battery status bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityState {
    Unknown = 0,
    FullAccess = 1,
    Unsealed = 2,
    Sealed = 3,
}

impl From<u8> for SecurityState {
    fn from(val: u8) -> Self {
        match val & 0x03 {
            1 => Self::FullAccess,
            2 => Self::Unsealed,
            3 => Self::Sealed,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Thermistor {
    Ts1,
    Ts2,
    Ts3,
    Hdq,
    Dchg,
    Ddsg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fet {
    Chg,
    Dch,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetState {
    Off,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScdThreshold {
    Scd10,
    Scd20,
    Scd40,
    Scd60,
    Scd80,
    Scd100,
    Scd125,
    Scd150,
    Scd175,
    Scd200,
    Scd250,
    Scd300,
    Scd350,
    Scd400,
    Scd450,
    Scd500,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyAlertC {
    pub ocd3: bool,
    pub scdl: bool,
    pub ocdl: bool,
    pub covl: bool,
    pub ptos: bool,
}

impl From<u8> for SafetyAlertC {
    fn from(val: u8) -> Self {
        Self {
            ocd3: (val & 0x80) != 0,
            scdl: (val & 0x40) != 0,
            ocdl: (val & 0x20) != 0,
            covl: (val & 0x10) != 0,
            ptos: (val & 0x08) != 0,
        }
    }
}

/// Safety Status C flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyStatusC {
    pub ocd3: bool,
    pub scdl: bool,
    pub ocdl: bool,
    pub covl: bool,
    pub pto: bool,
    pub hwdf: bool,
}

impl From<u16> for SafetyStatusC {
    fn from(val: u16) -> Self {
        Self {
            ocd3: (val & 0x0080) != 0,
            scdl: (val & 0x0040) != 0,
            ocdl: (val & 0x0020) != 0,
            covl: (val & 0x0010) != 0,
            pto: (val & 0x0004) != 0,
            hwdf: (val & 0x0002) != 0,
        }
    }
}

/// Safety Alert A flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyAlertA {
    pub scd: bool,
    pub ocd2: bool,
    pub ocd1: bool,
    pub occ: bool,
    pub cov: bool,
    pub cuv: bool,
}

impl From<u8> for SafetyAlertA {
    fn from(val: u8) -> Self {
        Self {
            scd: (val & 0x80) != 0,
            ocd2: (val & 0x40) != 0,
            ocd1: (val & 0x20) != 0,
            occ: (val & 0x10) != 0,
            cov: (val & 0x08) != 0,
            cuv: (val & 0x04) != 0,
        }
    }
}

/// Safety Alert B flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyAlertB {
    pub otf: bool,
    pub otint: bool,
    pub otd: bool,
    pub otc: bool,
    pub utint: bool,
    pub utd: bool,
    pub utc: bool,
}

impl From<u8> for SafetyAlertB {
    fn from(val: u8) -> Self {
        Self {
            otf: (val & 0x80) != 0,
            otint: (val & 0x40) != 0,
            otd: (val & 0x20) != 0,
            otc: (val & 0x10) != 0,
            utint: (val & 0x04) != 0,
            utd: (val & 0x02) != 0,
            utc: (val & 0x01) != 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryStatus {
    pub sleep_mode: bool,
    pub shutdown_pending: bool,
    pub permanent_fault: bool,
    pub safety_fault: bool,
    pub fuse_pin: bool,
    pub security_state: SecurityState,
    pub wr_to_otp_blocked: bool,
    pub wr_to_otp_pending: bool,
    pub open_wire_check: bool,
    pub wd_was_triggered: bool,
    pub full_reset_occurred: bool,
    pub sleep_en_allowed: bool,
    pub precharge_mode: bool,
    pub config_update_mode: bool,
}

impl From<u16> for BatteryStatus {
    fn from(val: u16) -> Self {
        Self {
            sleep_mode: (val & 0x8000) != 0,
            shutdown_pending: (val & 0x2000) != 0,
            permanent_fault: (val & 0x1000) != 0,
            safety_fault: (val & 0x0800) != 0,
            fuse_pin: (val & 0x0400) != 0,

            security_state: SecurityState::from(((val >> 8) & 0x03) as u8),

            wr_to_otp_blocked: (val & 0x0080) != 0,
            wr_to_otp_pending: (val & 0x0040) != 0,
            open_wire_check: (val & 0x0020) != 0,
            wd_was_triggered: (val & 0x0010) != 0,
            full_reset_occurred: (val & 0x0008) != 0,
            sleep_en_allowed: (val & 0x0004) != 0,
            precharge_mode: (val & 0x0002) != 0,
            config_update_mode: (val & 0x0001) != 0,
        }
    }
}

/// Safety Status A flags.etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyStatusA {
    pub cuv: bool,
    pub cov: bool,
    pub occ: bool,
    pub ocd1: bool,
    pub ocd2: bool,
    pub scd: bool,
}

impl From<u16> for SafetyStatusA {
    fn from(val: u16) -> Self {
        Self {
            cuv: (val & 0x0004) != 0,
            cov: (val & 0x0008) != 0,
            occ: (val & 0x0010) != 0,
            ocd1: (val & 0x0020) != 0,
            ocd2: (val & 0x0040) != 0,
            scd: (val & 0x0080) != 0,
        }
    }
}

/// Alarm Status register flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlarmStatus {
    pub ssbc: bool,      // Safety Status B or Safety Status C contains an active fault
    pub ssa: bool,       // Safety Status A contains an active fault
    pub pf: bool,        // Permanent Fail fault has triggered
    pub full_scan: bool, // Full voltage scan completed
}

impl From<u16> for AlarmStatus {
    fn from(val: u16) -> Self {
        Self {
            ssbc: (val & 0x8000) != 0,      // Bit 15: Safety Status B/C
            ssa: (val & 0x4000) != 0,       // Bit 14: Safety Status A
            pf: (val & 0x2000) != 0,        // Bit 13: Permanent Fail
            full_scan: (val & 0x0080) != 0, // Bit 7: Full voltage scan complete
        }
    }
}

/// Safety Status B flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyStatusB {
    pub otf: bool,
    pub otint: bool,
    pub otd: bool,
    pub otc: bool,
    pub utint: bool,
    pub utd: bool,
    pub utc: bool,
}

impl From<u16> for SafetyStatusB {
    fn from(val: u16) -> Self {
        Self {
            otf: (val & 0x0080) != 0,
            otint: (val & 0x0040) != 0,
            otd: (val & 0x0020) != 0,
            otc: (val & 0x0010) != 0,
            utint: (val & 0x0004) != 0,
            utd: (val & 0x0002) != 0,
            utc: (val & 0x0001) != 0,
        }
    }
}

/// Permanent Fault Status A flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermanentFaults {
    /// Safety Cell Undervoltage Permanent Fail
    pub suv: bool,

    /// Safety Cell Overvoltage Permanent Fail
    pub sov: bool,

    /// Safety Overcurrent in Charge Permanent Fail
    pub socc: bool,

    /// Safety Overcurrent in Discharge Permanent Fail
    pub socd: bool,

    /// Safety Overtemperature Permanent Fail
    pub sot: bool,

    /// Safety Overtemperature FET Permanent Fail
    pub sotf: bool,

    /// Copper Deposition Permanent Fail
    pub cudep: bool,
}

impl From<u16> for PermanentFaults {
    fn from(val: u16) -> Self {
        Self {
            suv: (val & 0x0001) != 0,
            sov: (val & 0x0002) != 0,
            socc: (val & 0x0004) != 0,
            socd: (val & 0x0008) != 0,
            sot: (val & 0x0010) != 0,
            sotf: (val & 0x0040) != 0,
            cudep: (val & 0x0080) != 0,
        }
    }
}

/// FET Status register flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FetStatus {
    pub alrt_pin: bool,
    pub ddsg_pin: bool,
    pub dchg_pin: bool,
    pub pdsg_fet: bool,
    pub dsg_fet: bool,
    pub pchg_fet: bool,
    pub chg_fet: bool,
}

impl From<u16> for FetStatus {
    fn from(val: u16) -> Self {
        Self {
            alrt_pin: (val & 0x0040) != 0,
            ddsg_pin: (val & 0x0020) != 0,
            dchg_pin: (val & 0x0010) != 0,
            pdsg_fet: (val & 0x0008) != 0,
            dsg_fet: (val & 0x0004) != 0,
            pchg_fet: (val & 0x0002) != 0,
            chg_fet: (val & 0x0001) != 0,
        }
    }
}
