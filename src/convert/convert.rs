
use super::{fractions, html, html_template, open_close, link};
use crate::Settings;

#[allow(unused)]
// convert file to html
pub fn convert(body: String, title: String, current_relative_path: String, main_file: String, settings: Settings, all_possible_file_links: Vec<String>) -> String {
    println!("settings not implemented yet");
    let mut output = String::new();

    for line in body.lines() {
        let mut out_line = line.to_string();
        out_line = link(out_line, all_possible_file_links.clone(), main_file.clone(), current_relative_path.clone());
        out_line = fractions(out_line);
        out_line = html(out_line);
        out_line = open_close(out_line);

        // to new line
        output += out_line.as_str();
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

