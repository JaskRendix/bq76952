//! # BQ76952 Battery Monitor Driver
//!
//! A `no_std`, `embedded-hal` 1.0 compatible Rust driver for the Texas Instruments BQ76952
//! stack monitor and protector.

#![no_std]

mod driver;
mod error;
pub mod registers;
mod types;

pub use driver::Bq76952;
pub use error::Error;
pub use types::{
    AlarmStatus, BatteryStatus, Fet, FetState, PermanentFaults, Protection, SafetyAlertC,
    SafetyStatusA, SafetyStatusB, SafetyStatusC, ScdThreshold, SecurityState,
    TemperatureProtection, Thermistor,
};
