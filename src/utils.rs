use std::fs::File;
use std::io::{BufReader, Read};
use std::env;
use std::path::Path;

pub fn read_bytes_from_env_file() -> Result<Vec<u8>, String> {
    let file_path = env::args().skip(1).next().ok_or(String::from("filename not provided"))?;

    let file_path = Path::new(&file_path);

    let file = File::open(file_path).map_err(|err| err.to_string())?;
    let mut reader = BufReader::new(file);
    let mut data = Vec::new();

    reader.read_to_end(&mut data).map_err(|err| err.to_string())?;

    Ok(data)
}
