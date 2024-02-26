use std::io::{Result, Write};


pub fn write_file(full_location: &str, body: String) -> Result<()> {

    let mut file = std::fs::File::create(full_location)?;
    file.write_all(body.as_bytes())?;

    Ok(())
}