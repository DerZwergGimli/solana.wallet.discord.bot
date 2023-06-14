use std::fs::File;
use std::io::{Read, Write};


use log::{error, info, warn};

use crate::configuration::Configuration;

pub fn read_config(path: String) -> Configuration {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn write_config(path: String, config: Configuration) {
    if config.update_config {
        let mut file = File::create(path.clone()).unwrap();
        match write!(file, "{}", serde_json::to_string_pretty(&config).unwrap()) {
            Ok(_) => {
                info!("Updated config file: {}", path)
            }
            Err(_) => {
                error!("Unable to write file: {}", path)
            }
        }
        info!("mapped_tx has been written to file")
    } else {
        warn!("config_update is disabled")
    }
}



