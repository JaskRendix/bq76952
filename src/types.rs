//! BQ76952 Types, Enums, and Status Bitfields

/// Device Security State representation based on battery status bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityState {
    Sealed = 0,
    Unsealed = 1,
    FullAccess = 2,
    Reserved = 3,
}

impl From<u8> for SecurityState {
    fn from(val: u8) -> Self {
        match val & 0x03 {
            0 => Self::Sealed,
            1 => Self::Unsealed,
            2 => Self::FullAccess,
            _ => Self::Reserved,
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
pub struct Protection {
    pub sc_dchg: bool,
    pub oc2_dchg: bool,
    pub oc1_dchg: bool,
    pub oc_chg: bool,
    pub cell_ov: bool,
    pub cell_uv: bool,
}

impl From<u8> for Protection {
    fn from(val: u8) -> Self {
        Self {
            sc_dchg: (val & 0x01) != 0,
            oc2_dchg: (val & 0x02) != 0,
            oc1_dchg: (val & 0x04) != 0,
            oc_chg: (val & 0x08) != 0,
            cell_ov: (val & 0x10) != 0,
            cell_uv: (val & 0x20) != 0,
        }
    }
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
            ocd3: (val & 0x01) != 0,
            scdl: (val & 0x02) != 0,
            ocdl: (val & 0x04) != 0,
            covl: (val & 0x08) != 0,
            ptos: (val & 0x10) != 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyStatusC {
    pub ocd3: bool,
    pub scdl: bool,
    pub ocdl: bool,
    pub covl: bool,
    pub ptos: bool,
}

impl From<u16> for SafetyStatusC {
    fn from(val: u16) -> Self {
        Self {
            ocd3: (val & 0x0001) != 0,
            scdl: (val & 0x0002) != 0,
            ocdl: (val & 0x0004) != 0,
            covl: (val & 0x0008) != 0,
            ptos: (val & 0x0010) != 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemperatureProtection {
    pub overtemp_fet: bool,
    pub overtemp_internal: bool,
    pub overtemp_dchg: bool,
    pub overtemp_chg: bool,
    pub undertemp_internal: bool,
    pub undertemp_dchg: bool,
    pub undertemp_chg: bool,
}

impl From<u8> for TemperatureProtection {
    fn from(val: u8) -> Self {
        Self {
            overtemp_fet: (val & 0x01) != 0,
            overtemp_internal: (val & 0x02) != 0,
            overtemp_dchg: (val & 0x04) != 0,
            overtemp_chg: (val & 0x08) != 0,
            undertemp_internal: (val & 0x10) != 0,
            undertemp_dchg: (val & 0x20) != 0,
            undertemp_chg: (val & 0x40) != 0,
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
    pub full_reset_occured: bool,
    pub sleep_en_allowed: bool,
    pub precharge_mode: bool,
    pub config_update_mode: bool,
}

impl From<u16> for BatteryStatus {
    fn from(val: u16) -> Self {
        Self {
            sleep_mode: (val & 0x0001) != 0,
            shutdown_pending: (val & 0x0004) != 0,
            permanent_fault: (val & 0x0008) != 0,
            safety_fault: (val & 0x0010) != 0,
            fuse_pin: (val & 0x0020) != 0,
            security_state: SecurityState::from(((val >> 6) & 0x0003) as u8),
            wr_to_otp_blocked: (val & 0x0100) != 0,
            wr_to_otp_pending: (val & 0x0200) != 0,
            open_wire_check: (val & 0x0400) != 0,
            wd_was_triggered: (val & 0x0800) != 0,
            full_reset_occured: (val & 0x1000) != 0,
            sleep_en_allowed: (val & 0x2000) != 0,
            precharge_mode: (val & 0x4000) != 0,
            config_update_mode: (val & 0x8000) != 0,
        }
    }
}

/// Safety Status A flags (CUV, COV, OCC, OCD1, OCD2, SCD, etc.)
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
            cuv: (val & 0x0001) != 0,
            cov: (val & 0x0002) != 0,
            occ: (val & 0x0004) != 0,
            ocd1: (val & 0x0008) != 0,
            ocd2: (val & 0x0010) != 0,
            scd: (val & 0x0020) != 0,
        }
    }
}

/// Alarm Status flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlarmStatus {
    pub safety_alert: bool,
    pub safety_status: bool,
    pub pf: bool,
    pub full_scan: bool,
}

impl From<u16> for AlarmStatus {
    fn from(val: u16) -> Self {
        Self {
            safety_alert: (val & 0x0001) != 0,
            safety_status: (val & 0x0002) != 0,
            pf: (val & 0x0010) != 0,
            full_scan: (val & 0x0020) != 0,
        }
    }
}

/// Safety Status B flags (OTF, OVT, UTINT, UTOT, OCD3, SCDL, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyStatusB {
    pub otf: bool,
    pub oot: bool,
    pub utint: bool,
    pub ut_dchg: bool,
    pub ut_chg: bool,
    pub ocd3: bool,
}

impl From<u16> for SafetyStatusB {
    fn from(val: u16) -> Self {
        Self {
            otf: (val & 0x0080) != 0, // Bit 7: OTF (FET Overtemperature)
            oot: (val & 0x0040) != 0, // Bit 6: OOT / OVT
            utint: (val & 0x0020) != 0,
            ut_dchg: (val & 0x0010) != 0,
            ut_chg: (val & 0x0008) != 0,
            ocd3: (val & 0x0001) != 0,
        }
    }
}

/// Permanent Fault Status flags (Thermal Runaway, Permanent Overcurrent, Fuse Blow, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermanentFaults {
    pub sotf: bool, // Safety Over Temperature FET
    pub sopt: bool, // Safety Over Power / Temperature
    pub fuse: bool, // Fuse blown / triggered
    pub tov: bool,  // Permanent Cell Overvoltage
    pub suv: bool,  // Permanent Cell Undervoltage
    pub soc: bool,  // Permanent Overcurrent
}

impl From<u16> for PermanentFaults {
    fn from(val: u16) -> Self {
        Self {
            sotf: (val & 0x0001) != 0,
            sopt: (val & 0x0002) != 0,
            fuse: (val & 0x0004) != 0,
            tov: (val & 0x0008) != 0,
            suv: (val & 0x0010) != 0,
            soc: (val & 0x0020) != 0,
        }
    }
}
