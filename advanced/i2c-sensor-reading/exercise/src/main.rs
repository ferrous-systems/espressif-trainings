use anyhow;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::*;
// use imc42670p::{IMC42670P, SlaveAddr}; // not in solution?? unnecesary??

use shtcx::{self, PowerMode};

// use shared_bus;  // not in solution?? unnecesary??

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

    // is this too much of the solution already???
    // let i2c = Master::<I2C0, _, _>::new(
    //     ... ,
    //     ... ,
    //     <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    // )?;

    // TO DO: ^should we explain about baudrate? -> yes, but in workshop / handbook


    // Instanciate the driver for the sensor and pass it i2c
    // let mut sht = ... ;

    // print device_id

 

    loop {
        // inside the loop, 
        // start to measure
        // delay 100ms (using FreeRtos)
        // keep measuring
        // print results (temperature in degrees celsius, humidity as percent )
        // delay 500ms


        // sht.start_measurement(PowerMode::NormalMode).unwrap();
        // FreeRtos.delay_ms(100u32); // FreeRtos is an operating system, thats been explained. say a bit more??
        // let measurement = sht.get_measurement_result().unwrap(); 
        
        // print temperature (in degrees celsius) humidity and
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

