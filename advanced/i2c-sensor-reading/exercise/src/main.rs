use anyhow;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
use imc42670p::{IMC42670P, SlaveAddr};

use shtcx::{self, PowerMode};

use shared_bus;

// goals of this exercise:
// instantiate i2c peripheral
// implement one sensor, print sensor values
// implement second sensor on same bus to solve an ownership problem

fn main() -> anyhow::Result<()>  {
    link_patches();

    // instantiate i2c peripheral
    // let peripherals = ...;

    // Define two pins, one as SDA and one as SCL.
    // let sda = ...;
    // let scl = ...;

    // Use them to instantiate an Instance of the I²C peripheral.
    // let i2c = Master::<I2C0, _, _>::new(
    //     ... ,
    //     ... ,
    //     <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    // )?;

    // TO DO: ^should we explain about baudrate?

    // let mut sht = shtcx::shtc3(i2c);
    // let device_id = sht.device_identifier().unwrap();
 

    // println!("Device ID: {}", device_id);

    loop {
        // sht.start_measurement(PowerMode::NormalMode).unwrap();
        // FreeRtos.delay_ms(100u32);
        // let measurement = sht.get_measurement_result().unwrap(); 
        

        // println!(
        //     "TEMP: {}\n
        //     HUM: {:?}\n
        //     \n 
        //     ",
        //     measurement.temperature.as_degrees_celsius(), measurement.humidity.as_percent(),
        // );

        FreeRtos.delay_ms(500u32);
    }
}

