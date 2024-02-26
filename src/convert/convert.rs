
use super::{fractions, html, html_template, open_close};

#[allow(unused)]
// convert file to html
pub fn convert(body: String, title: String, current_relative_path: String) -> String {
    let mut output = String::new();

    for line in body.lines() {
        let frac = fractions(line.to_string());
        let html = html(frac);
        let open_close = open_close(html);

        // to new line
        output += open_close.as_str();
        output += "\n";
    }

    let back_route_points = current_relative_path.split("/").collect::<Vec<&str>>().len() - 1;
    let mut back_route = String::new();
    back_route += "./";
    for _ in 0..back_route_points {
        back_route += "../";
    }

    output = html_template(back_route, title, output);
    output
}

