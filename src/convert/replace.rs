use crate::Settings;


pub fn replace_by(line: String, settings: &Settings) -> String {
    let replacements = settings.replace_by.clone();

    let mut output = line.clone();

    for [key, item] in replacements.iter() {
        output = output.replace(key, &item);
    }
    
    output
}