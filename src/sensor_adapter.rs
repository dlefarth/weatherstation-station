use crate::measurement::Measurement;
use chrono::{Utc};

extern crate bmp085;
extern crate i2cdev;

use bmp085::*;
use i2cdev::linux::*;
use i2cdev::sensors::{Barometer, Thermometer};

pub fn measure() -> Measurement {
    
    let i2c_dev = LinuxI2CDevice::new("/dev/i2c-1", BMP085_I2C_ADDR).unwrap();
    let mut s = BMP085BarometerThermometer::new(i2c_dev, SamplingMode::Standard).unwrap();

    Measurement {
        temperature: s.temperature_celsius().unwrap(),
        pressure: s.pressure_kpa().unwrap(),
        timestamp: Utc::now()
    }
}