
mod convert;
use convert::convert;
mod locations;

/// ## md_to_html
/// ### params:
/// - md_body:      file content
/// - title:        title + header for file
/// - paths:        locations of important files (used for links)
/// - css_template: optional custom css
#[allow(unused)]
pub fn md_to_html(md_body: String, title: String, relative_target_location: String) -> String {
    convert(md_body, title, relative_target_location) 
}
