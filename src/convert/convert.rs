use super::{fractions, html, open_close};

pub fn convert(body: String) -> String {
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