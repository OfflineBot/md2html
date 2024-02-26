
mod convert;
use convert::convert;

mod settings;
pub use settings::Settings;

/// ## md_to_html
/// ### params:
/// - md_body:      file content
/// - title:        title + header for file
/// - paths:        locations of important files (used for links)
/// - settings:     md edit settings. uses default if not given
#[allow(unused)]
pub fn md_to_html(md_body: String, title: String, relative_target_location: String, settings: Option<Settings>, all_possible_files: Vec<String>) -> String {
    let out_settings = match settings {
        Some(settings) => settings,
        None => Settings::default(),
    };
    convert(md_body, title, relative_target_location, out_settings, all_possible_files)
}