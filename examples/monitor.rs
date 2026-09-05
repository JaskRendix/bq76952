use bq76952::Bq76952;
use linux_embedded_hal::I2cdev;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing BQ76952 over /dev/i2c-1...");

    // Open the Linux I2C bus (adjust bus number as needed, e.g., 1)
    let i2c = I2cdev::new("/dev/i2c-1")?;

    // BQ76952 default I2C address is typically 0x08 or 0x10 depending on configuration
    let mut bms = Bq76952::new(i2c, 0x10);

    // Check connection
    if bms.is_connected() {
        println!("Successfully connected to BQ76952!");
    } else {
        println!("Warning: Device did not acknowledge connection check.");
    }

    println!("Starting telemetry polling loop (Press Ctrl+C to exit)...");

    loop {
        match bms.get_stack_voltage() {
            Ok(stack_v) => println!("Stack Voltage: {} mV", stack_v),
            Err(e) => eprintln!("Failed to read stack voltage: {:?}", e),
        }

        match bms.get_current() {
            Ok(current) => println!("Current: {} mA", current),
            Err(e) => eprintln!("Failed to read current: {:?}", e),
        }

        match bms.get_min_cell_voltage() {
            Ok(min_v) => println!("Min Cell Voltage: {} mV", min_v),
            Err(e) => eprintln!("Failed to read min cell voltage: {:?}", e),
        }

        match bms.get_max_cell_voltage() {
            Ok(max_v) => println!("Max Cell Voltage: {} mV", max_v),
            Err(e) => eprintln!("Failed to read max cell voltage: {:?}", e),
        }

        match bms.get_safety_status_a() {
            Ok(status) => {
                if status.cov || status.cuv || status.occ || status.scd {
                    println!(
                        "FAULTS DETECTED -> OVP: {}, UVP: {}, OCC: {}, SCD: {}",
                        status.cov, status.cuv, status.occ, status.scd
                    );
                }
            }
            Err(e) => eprintln!("Failed to read safety status: {:?}", e),
        }

        println!("--------------------------------------------------");
        thread::sleep(Duration::from_secs(1));
    }
}
