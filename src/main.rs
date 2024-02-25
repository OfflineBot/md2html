
mod convert;
use convert::{
    fractions,
    html, open_close,
};

use std::io::{Result, Write};
use std::{fs, io::Read};

fn write(body: String) {
    let mut file = std::fs::File::create("./index.html").unwrap();
    file.write_all(body.as_bytes()).unwrap();
}

fn convert(body: String) -> String {
    let mut output = String::new();

    for line in body.lines() {
        let frac = fractions(line.to_string());
        let html = html(frac);
        let open_close = open_close(html);

        // to new line
        output += open_close.as_str();
        output += "\n";
    }

    output
}

fn main() -> Result<()> {
    let mut test_file = String::new();
    let mut f = fs::File::open("./test.md")?;
    f.read_to_string(&mut test_file)?;

    let output = convert(test_file);

    write(output);
    Ok(())
}
