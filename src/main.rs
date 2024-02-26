
mod convert;
use convert::convert;

mod files;
use files::write_file;

mod input;
mod storage;

use std::{
    fs, 
    io::{Read, Result}
};


fn main() -> Result<()> {
    let mut test_file = String::new();
    let mut f = fs::File::open("./test.md")?;
    f.read_to_string(&mut test_file)?;

    let output = convert(test_file);

    write_file("./index.html", output)?;
    Ok(())
}
