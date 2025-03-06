extern crate serde_json;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Cartridge {
    code: String,
    sprites: Vec<u8>,
}

pub fn save_cartridge(cart: &Cartridge, filename: &str) {
    let json = serde_json::to_string(cart).unwrap();
    fs::write(filename, json).unwrap();
}
pub fn load_cartridge(filename: &str) -> Cartridge {
    let data = fs::read_to_string(&filename).unwrap();
    serde_json::from_str(&data).unwrap()
}