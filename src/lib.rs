
extern crate spotr_sensing;
extern crate toml;
extern crate serde_derive;

use nix::sys::statvfs;
use serde_derive::Deserialize;
use std::result::Result;

use spotr_sensing::{Sensor, SensorOutput};

#[derive(Deserialize, Clone)]
struct MountSensorConfig {
    mount_points: Vec<String>
}

#[no_mangle]
pub fn initialize(config: &str) -> *mut dyn Sensor {
    let sensor_config: MountSensorConfig = toml::from_str(config).unwrap();
    Box::into_raw(Box::new(MountSensor { mount_points : sensor_config.mount_points}))
}

struct MountSensor {
    mount_points: Vec<String>
}

impl MountSensor {
    fn statvfs(&self) -> Vec<SensorOutput> {
        vec!()
    }
}

impl Sensor for MountSensor {
    fn sample(&self) -> Result<Vec<SensorOutput>, std::io::Error> {
         Ok(self.statvfs())
    }
}
