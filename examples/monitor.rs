use bq76952::{Bq76952, Thermistor};
use linux_embedded_hal::I2cdev;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing BQ76952...");

    let i2c = I2cdev::new("/dev/i2c-1")?;

    // Adjust address to match your hardware configuration.
    let mut bms = Bq76952::new(i2c, 0x10);

    if !bms.is_connected() {
        eprintln!("Unable to communicate with BQ76952");
        return Ok(());
    }

    println!("Connected to BQ76952");
    println!("Starting telemetry monitoring...\n");

    loop {
        println!("==================================================");

        // Basic telemetry
        match bms.get_stack_voltage() {
            Ok(v) => println!("Stack Voltage     : {} mV", v),
            Err(e) => eprintln!("Stack Voltage     : {:?}", e),
        }

        match bms.get_current() {
            Ok(i) => println!("Pack Current      : {} mA", i),
            Err(e) => eprintln!("Pack Current      : {:?}", e),
        }

        match bms.get_min_cell_voltage() {
            Ok(v) => println!("Min Cell Voltage  : {} mV", v),
            Err(e) => eprintln!("Min Cell Voltage  : {:?}", e),
        }

        match bms.get_max_cell_voltage() {
            Ok(v) => println!("Max Cell Voltage  : {} mV", v),
            Err(e) => eprintln!("Max Cell Voltage  : {:?}", e),
        }

        // Temperatures
        match bms.get_internal_temperature_celsius() {
            Ok(t) => println!("Internal Temp     : {:.1} °C", t),
            Err(e) => eprintln!("Internal Temp     : {:?}", e),
        }

        match bms.get_temperature_celsius(Thermistor::Ts1) {
            Ok(t) => println!("TS1 Temperature   : {:.1} °C", t),
            Err(e) => eprintln!("TS1 Temperature   : {:?}", e),
        }

        // Battery status
        match bms.get_battery_status() {
            Ok(status) => {
                println!("Security State    : {:?}", status.security_state);
                println!("Sleep Mode        : {}", status.sleep_mode);
                println!("Permanent Fault   : {}", status.permanent_fault);
                println!("Safety Fault      : {}", status.safety_fault);
            }
            Err(e) => eprintln!("Battery Status    : {:?}", e),
        }

        // Alarm status
        match bms.get_alarm_status() {
            Ok(alarm) => {
                if alarm.ssa || alarm.ssbc || alarm.pf {
                    println!(
                        "Alarm Status      : SSA={}, SSBC={}, PF={}",
                        alarm.ssa, alarm.ssbc, alarm.pf
                    );
                }
            }
            Err(e) => eprintln!("Alarm Status      : {:?}", e),
        }

        // Safety alerts
        match bms.get_safety_alert_a() {
            Ok(alert) => {
                if alert.cuv || alert.cov || alert.occ || alert.ocd1 || alert.ocd2 || alert.scd {
                    println!(
                        "Safety Alert A    : CUV={}, COV={}, OCC={}, OCD1={}, OCD2={}, SCD={}",
                        alert.cuv, alert.cov, alert.occ, alert.ocd1, alert.ocd2, alert.scd
                    );
                }
            }
            Err(e) => eprintln!("Safety Alert A    : {:?}", e),
        }

        match bms.get_safety_alert_b() {
            Ok(alert) => {
                if alert.otf
                    || alert.otint
                    || alert.otd
                    || alert.otc
                    || alert.utint
                    || alert.utd
                    || alert.utc
                {
                    println!(
                        "Safety Alert B    : OTF={}, OTINT={}, OTD={}, OTC={}, UTINT={}, UTD={}, UTC={}",
                        alert.otf,
                        alert.otint,
                        alert.otd,
                        alert.otc,
                        alert.utint,
                        alert.utd,
                        alert.utc
                    );
                }
            }
            Err(e) => eprintln!("Safety Alert B    : {:?}", e),
        }

        // Safety status
        match bms.get_safety_status_a() {
            Ok(status) => {
                if status.cuv
                    || status.cov
                    || status.occ
                    || status.ocd1
                    || status.ocd2
                    || status.scd
                {
                    println!(
                        "Safety Status A   : CUV={}, COV={}, OCC={}, OCD1={}, OCD2={}, SCD={}",
                        status.cuv, status.cov, status.occ, status.ocd1, status.ocd2, status.scd
                    );
                }
            }
            Err(e) => eprintln!("Safety Status A   : {:?}", e),
        }

        match bms.get_safety_status_b() {
            Ok(status) => {
                if status.otf
                    || status.otint
                    || status.otd
                    || status.otc
                    || status.utint
                    || status.utd
                    || status.utc
                {
                    println!(
                        "Safety Status B   : OTF={}, OTINT={}, OTD={}, OTC={}, UTINT={}, UTD={}, UTC={}",
                        status.otf,
                        status.otint,
                        status.otd,
                        status.otc,
                        status.utint,
                        status.utd,
                        status.utc
                    );
                }
            }
            Err(e) => eprintln!("Safety Status B   : {:?}", e),
        }

        match bms.get_safety_status_c() {
            Ok(status) => {
                if status.ocd3
                    || status.scdl
                    || status.ocdl
                    || status.covl
                    || status.pto
                    || status.hwdf
                {
                    println!(
                        "Safety Status C   : OCD3={}, SCDL={}, OCDL={}, COVL={}, PTO={}, HWDF={}",
                        status.ocd3, status.scdl, status.ocdl, status.covl, status.pto, status.hwdf
                    );
                }
            }
            Err(e) => eprintln!("Safety Status C   : {:?}", e),
        }

        // Permanent faults
        match bms.get_permanent_faults() {
            Ok(pf) => {
                if pf.suv || pf.sov || pf.socc || pf.socd || pf.sot || pf.sotf || pf.cudep {
                    println!(
                        "Permanent Faults  : SUV={}, SOV={}, SOCC={}, SOCD={}, SOT={}, SOTF={}, CUDEP={}",
                        pf.suv, pf.sov, pf.socc, pf.socd, pf.sot, pf.sotf, pf.cudep
                    );
                }
            }
            Err(e) => eprintln!("Permanent Faults  : {:?}", e),
        }

        // FET status
        match bms.get_fet_status() {
            Ok(fet) => {
                println!(
                    "FET Status        : CHG={} PCHG={} DSG={} PDSG={}",
                    fet.chg_fet, fet.pchg_fet, fet.dsg_fet, fet.pdsg_fet
                );

                println!(
                    "Pins              : ALERT={} DCHG={} DDSG={}",
                    fet.alrt_pin, fet.dchg_pin, fet.ddsg_pin
                );
            }
            Err(e) => eprintln!("FET Status        : {:?}", e),
        }

        println!();
        thread::sleep(Duration::from_secs(1));
    }
}
