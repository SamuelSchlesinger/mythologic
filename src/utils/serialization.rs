use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

/// Save data to a JSON file
pub fn save_to_json<T: Serialize>(data: &T, path: &Path) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Load data from a JSON file
pub fn load_from_json<T: for<'de> Deserialize<'de>>(path: &Path) -> std::io::Result<T> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data = serde_json::from_str(&contents)?;
    Ok(data)
}
