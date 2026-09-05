use bq76952::{
    AlarmStatus, BatteryStatus, Fet, FetState, PermanentFaults, Protection, SafetyAlertC,
    SafetyStatusA, SafetyStatusB, SafetyStatusC, ScdThreshold, SecurityState,
    TemperatureProtection, Thermistor,
};

#[test]
fn protection_all_bits_clear() {
    let p = Protection::from(0x00);
    assert!(!p.sc_dchg);
    assert!(!p.oc2_dchg);
    assert!(!p.oc1_dchg);
    assert!(!p.oc_chg);
    assert!(!p.cell_ov);
    assert!(!p.cell_uv);
}

#[test]
fn protection_all_bits_set() {
    let p = Protection::from(0xFF);
    assert!(p.sc_dchg);
    assert!(p.oc2_dchg);
    assert!(p.oc1_dchg);
    assert!(p.oc_chg);
    assert!(p.cell_ov);
    assert!(p.cell_uv);
}

#[test]
fn protection_specific_bits() {
    // Matches: sc_dchg (bit 0), oc2_dchg (bit 1), oc_chg (bit 3), cell_uv (bit 5)
    let p = Protection::from(0b0010_1011);
    assert!(p.sc_dchg);
    assert!(p.oc2_dchg);
    assert!(!p.oc1_dchg);
    assert!(p.oc_chg);
    assert!(!p.cell_ov);
    assert!(p.cell_uv);
}

#[test]
fn safety_alert_all_clear() {
    let s = SafetyAlertC::from(0x00);
    assert!(!s.ocd3);
    assert!(!s.scdl);
    assert!(!s.ocdl);
    assert!(!s.covl);
    assert!(!s.ptos);
}

#[test]
fn safety_alert_all_set() {
    let s = SafetyAlertC::from(0xFF);
    assert!(s.ocd3);
    assert!(s.scdl);
    assert!(s.ocdl);
    assert!(s.covl);
    assert!(s.ptos);
}

#[test]
fn safety_alert_specific_bits() {
    let s = SafetyAlertC::from(0b0001_1011);
    assert!(s.ocd3);
    assert!(s.scdl);
    assert!(!s.ocdl);
    assert!(s.covl);
    assert!(s.ptos);
}

#[test]
fn safety_status_c_parsing() {
    let s = SafetyStatusC::from(0x001F);
    assert!(s.ocd3);
    assert!(s.scdl);
    assert!(s.ocdl);
    assert!(s.covl);
    assert!(s.ptos);
}

#[test]
fn safety_status_b_parsing() {
    // Bit 7: OTF, Bit 6: OOT, Bit 5: UTINT, Bit 4: UT_DCHG, Bit 3: UT_CHG, Bit 0: OCD3
    let s = SafetyStatusB::from(0x00FB);
    assert!(s.otf);
    assert!(s.oot);
    assert!(s.utint);
    assert!(s.ut_dchg);
    assert!(s.ut_chg);
    assert!(s.ocd3);
}

#[test]
fn temp_protection_all_clear() {
    let t = TemperatureProtection::from(0x00);
    assert!(!t.overtemp_fet);
    assert!(!t.overtemp_internal);
    assert!(!t.overtemp_dchg);
    assert!(!t.overtemp_chg);
    assert!(!t.undertemp_internal);
    assert!(!t.undertemp_dchg);
    assert!(!t.undertemp_chg);
}

#[test]
fn temp_protection_all_set() {
    let t = TemperatureProtection::from(0xFF);
    assert!(t.overtemp_fet);
    assert!(t.overtemp_internal);
    assert!(t.overtemp_dchg);
    assert!(t.overtemp_chg);
    assert!(t.undertemp_internal);
    assert!(t.undertemp_dchg);
    assert!(t.undertemp_chg);
}

#[test]
fn temp_protection_specific_bits() {
    // Matches: overtemp_fet (bit 0), overtemp_internal (bit 1), overtemp_chg (bit 3), undertemp_dchg (bit 5)
    let t = TemperatureProtection::from(0b0010_1011);
    assert!(t.overtemp_fet);
    assert!(t.overtemp_internal);
    assert!(!t.overtemp_dchg);
    assert!(t.overtemp_chg);
    assert!(!t.undertemp_internal);
    assert!(t.undertemp_dchg);
    assert!(!t.undertemp_chg);
}

#[test]
fn battery_status_all_clear() {
    let b = BatteryStatus::from(0x0000);
    assert!(!b.sleep_mode);
    assert!(!b.shutdown_pending);
    assert!(!b.permanent_fault);
    assert!(!b.safety_fault);
    assert!(!b.fuse_pin);
    assert_eq!(b.security_state, SecurityState::Sealed);
    assert!(!b.wr_to_otp_blocked);
    assert!(!b.wr_to_otp_pending);
    assert!(!b.open_wire_check);
    assert!(!b.wd_was_triggered);
    assert!(!b.full_reset_occured);
    assert!(!b.sleep_en_allowed);
    assert!(!b.precharge_mode);
    assert!(!b.config_update_mode);
}

#[test]
fn battery_status_all_set() {
    let b = BatteryStatus::from(0xFFFF);
    assert!(b.sleep_mode);
    assert!(b.shutdown_pending);
    assert!(b.permanent_fault);
    assert!(b.safety_fault);
    assert!(b.fuse_pin);
    assert_eq!(b.security_state, SecurityState::Reserved);
    assert!(b.wr_to_otp_blocked);
    assert!(b.wr_to_otp_pending);
    assert!(b.open_wire_check);
    assert!(b.wd_was_triggered);
    assert!(b.full_reset_occured);
    assert!(b.sleep_en_allowed);
    assert!(b.precharge_mode);
    assert!(b.config_update_mode);
}

#[test]
fn battery_status_security_state_values() {
    let states = [
        SecurityState::Sealed,
        SecurityState::Unsealed,
        SecurityState::FullAccess,
        SecurityState::Reserved,
    ];
    for (i, &expected_state) in states.iter().enumerate() {
        let raw = (i as u16) << 6;
        let b = BatteryStatus::from(raw);
        assert_eq!(b.security_state, expected_state);
    }
}

#[test]
fn battery_status_specific_bits() {
    let raw: u16 = 0x0001 // sleep_mode
        | 0x0008 // permanent_fault
        | 0x0010 // safety_fault
        | 0x0020 // fuse_pin
        | (0b10 << 6) // security_state = FullAccess
        | 0x0200 // wr_to_otp_pending
        | 0x0800 // wd_was_triggered
        | 0x2000 // sleep_en_allowed
        | 0x8000; // config_update_mode

    let b = BatteryStatus::from(raw);

    assert!(b.sleep_mode);
    assert!(!b.shutdown_pending);
    assert!(b.permanent_fault);
    assert!(b.safety_fault);
    assert!(b.fuse_pin);

    assert_eq!(b.security_state, SecurityState::FullAccess);

    assert!(!b.wr_to_otp_blocked);
    assert!(b.wr_to_otp_pending);
    assert!(!b.open_wire_check);
    assert!(b.wd_was_triggered);
    assert!(!b.full_reset_occured);
    assert!(b.sleep_en_allowed);
    assert!(!b.precharge_mode);
    assert!(b.config_update_mode);
}

#[test]
fn thermistor_enum_values() {
    assert_eq!(Thermistor::Ts1 as u8, 0);
    assert_eq!(Thermistor::Ts2 as u8, 1);
    assert_eq!(Thermistor::Ts3 as u8, 2);
    assert_eq!(Thermistor::Hdq as u8, 3);
    assert_eq!(Thermistor::Dchg as u8, 4);
    assert_eq!(Thermistor::Ddsg as u8, 5);
}

#[test]
fn fet_enum_values() {
    assert_eq!(Fet::Chg as u8, 0);
    assert_eq!(Fet::Dch as u8, 1);
    assert_eq!(Fet::All as u8, 2);
}

#[test]
fn fet_state_enum_values() {
    assert_eq!(FetState::Off as u8, 0);
    assert_eq!(FetState::On as u8, 1);
}

#[test]
fn scd_threshold_enum_values() {
    assert_eq!(ScdThreshold::Scd10 as u8, 0);
    assert_eq!(ScdThreshold::Scd500 as u8, 15);
}

#[test]
fn safety_status_a_all_clear() {
    let s = SafetyStatusA::from(0x0000);
    assert!(!s.cuv);
    assert!(!s.cov);
    assert!(!s.occ);
    assert!(!s.ocd1);
    assert!(!s.ocd2);
    assert!(!s.scd);
}

#[test]
fn safety_status_a_all_set() {
    let s = SafetyStatusA::from(0x003F);
    assert!(s.cuv);
    assert!(s.cov);
    assert!(s.occ);
    assert!(s.ocd1);
    assert!(s.ocd2);
    assert!(s.scd);
}

#[test]
fn safety_status_a_specific_bits() {
    // cuv + occ + ocd2
    let s = SafetyStatusA::from(0b0001_0101);
    assert!(s.cuv);
    assert!(!s.cov);
    assert!(s.occ);
    assert!(!s.ocd1);
    assert!(s.ocd2);
    assert!(!s.scd);
}

#[test]
fn alarm_status_all_clear() {
    let a = AlarmStatus::from(0x0000);
    assert!(!a.safety_alert);
    assert!(!a.safety_status);
    assert!(!a.pf);
    assert!(!a.full_scan);
}

#[test]
fn alarm_status_all_set() {
    let a = AlarmStatus::from(0x0033);
    assert!(a.safety_alert);
    assert!(a.safety_status);
    assert!(a.pf);
    assert!(a.full_scan);
}

#[test]
fn alarm_status_specific_bits() {
    let a = AlarmStatus::from(0b0000_0001); // Only safety_alert set
    assert!(a.safety_alert);
    assert!(!a.safety_status);
    assert!(!a.pf);
    assert!(!a.full_scan);
}

#[test]
fn permanent_faults_all_clear() {
    let pf = PermanentFaults::from(0x0000);
    assert!(!pf.sotf);
    assert!(!pf.sopt);
    assert!(!pf.fuse);
    assert!(!pf.tov);
    assert!(!pf.suv);
    assert!(!pf.soc);
}

#[test]
fn permanent_faults_all_set() {
    let pf = PermanentFaults::from(0x003F);
    assert!(pf.sotf);
    assert!(pf.sopt);
    assert!(pf.fuse);
    assert!(pf.tov);
    assert!(pf.suv);
    assert!(pf.soc);
}

#[test]
fn permanent_faults_specific_bits() {
    let pf = PermanentFaults::from(0b0000_0101); // sets sotf and fuse, soc is clear
    assert!(pf.sotf);
    assert!(!pf.sopt);
    assert!(pf.fuse);
    assert!(!pf.tov);
    assert!(!pf.suv);
    assert!(!pf.soc);
}

#[test]
fn security_state_direct_mapping() {
    assert_eq!(SecurityState::from(0), SecurityState::Sealed);
    assert_eq!(SecurityState::from(1), SecurityState::Unsealed);
    assert_eq!(SecurityState::from(2), SecurityState::FullAccess);
    assert_eq!(SecurityState::from(3), SecurityState::Reserved);
    assert_eq!(SecurityState::from(0xFF), SecurityState::Reserved); // masked
}

#[test]
fn thermistor_enum_pattern_matching() {
    fn id(t: Thermistor) -> &'static str {
        match t {
            Thermistor::Ts1 => "TS1",
            Thermistor::Ts2 => "TS2",
            Thermistor::Ts3 => "TS3",
            Thermistor::Hdq => "HDQ",
            Thermistor::Dchg => "DCHG",
            Thermistor::Ddsg => "DDSG",
        }
    }

    assert_eq!(id(Thermistor::Ts1), "TS1");
    assert_eq!(id(Thermistor::Ddsg), "DDSG");
}

#[test]
fn fet_enum_pattern_matching() {
    fn is_charge(f: Fet) -> bool {
        matches!(f, Fet::Chg)
    }

    assert!(is_charge(Fet::Chg));
    assert!(!is_charge(Fet::Dch));
}

#[test]
fn scd_threshold_monotonic_values() {
    let vals = [
        ScdThreshold::Scd10 as u8,
        ScdThreshold::Scd20 as u8,
        ScdThreshold::Scd40 as u8,
        ScdThreshold::Scd60 as u8,
        ScdThreshold::Scd80 as u8,
        ScdThreshold::Scd100 as u8,
        ScdThreshold::Scd125 as u8,
        ScdThreshold::Scd150 as u8,
        ScdThreshold::Scd175 as u8,
        ScdThreshold::Scd200 as u8,
        ScdThreshold::Scd250 as u8,
        ScdThreshold::Scd300 as u8,
        ScdThreshold::Scd350 as u8,
        ScdThreshold::Scd400 as u8,
        ScdThreshold::Scd450 as u8,
        ScdThreshold::Scd500 as u8,
    ];

    for (i, &v) in vals.iter().enumerate() {
        assert_eq!(v, i as u8);
    }
}

#[test]
fn battery_status_random_values() {
    for raw in [0x1234, 0xAAAA, 0x0F0F, 0xF00F] {
        let b = BatteryStatus::from(raw);

        // SecurityState must always be in 0..=3
        assert!(matches!(
            b.security_state,
            SecurityState::Sealed
                | SecurityState::Unsealed
                | SecurityState::FullAccess
                | SecurityState::Reserved
        ));
    }
}

#[test]
fn safety_alert_vs_status_c_consistency() {
    let raw8: u8 = 0b0001_1011;
    let raw16: u16 = raw8 as u16;

    let a = SafetyAlertC::from(raw8);
    let s = SafetyStatusC::from(raw16);

    assert_eq!(a.ocd3, s.ocd3);
    assert_eq!(a.scdl, s.scdl);
    assert_eq!(a.ocdl, s.ocdl);
    assert_eq!(a.covl, s.covl);
    assert_eq!(a.ptos, s.ptos);
}
