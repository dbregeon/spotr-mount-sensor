extern crate serde_derive;
extern crate spotr_sensing;
extern crate toml;

use nix::sys::statvfs::statvfs;
use serde_derive::Deserialize;
use std::result::Result;

use spotr_sensing::{Sensor, SensorOutput};

#[derive(Deserialize, Clone)]
struct MountSensorConfig {
    mount_points: Vec<String>,
}

#[no_mangle]
pub fn initialize(config: &str) -> *mut dyn Sensor {
    let sensor_config: MountSensorConfig = toml::from_str(config).unwrap();
    Box::into_raw(Box::new(MountSensor {
        mount_points: sensor_config.mount_points,
    }))
}

struct MountSensor {
    mount_points: Vec<String>,
}

impl MountSensor {
    fn statvfs(&self) -> Vec<SensorOutput> {
        self.mount_points
            .iter()
            .flat_map(|m| {
                statvfs(m.as_str())
                    .map(|s| (m, s))
                    .map(|(m, s)| SensorOutput::MountPoint {
                        name: m.to_string(),
                        size: s.blocks() * s.fragment_size(),
                        free: s.blocks_free() * s.fragment_size(),
                    })
            })
            .collect()
    }
}

impl Sensor for MountSensor {
    fn sample(&mut self) -> Result<Vec<SensorOutput>, std::io::Error> {
        Ok(self.statvfs())
    }
}
