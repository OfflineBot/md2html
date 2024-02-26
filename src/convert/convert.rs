
use super::{fractions, html, html_template, open_close, link};
use crate::Settings;

#[allow(unused)]
// convert file to html
pub fn convert(body: String, title: String, current_relative_path: String, main_file: String, settings: Settings, all_possible_file_links: Vec<String>) -> String {
    println!("settings not implemented yet");
    let mut output = String::new();

    for line in body.lines() {
        let line = link(line.to_string(), all_possible_file_links.clone(), main_file.clone());
        let frac = fractions(line);
        let html = html(frac);
        let open_close = open_close(html);

        // to new line
        output += open_close.as_str();
        output += "\n";
    }

    let back_route_points = current_relative_path.split("/").collect::<Vec<&str>>().len() - 2;
    let mut back_route = String::new();
    back_route += "./";
    for _ in 0..back_route_points {
        back_route += "../";
    }

    output = html_template(back_route, title, output);
    output
}

