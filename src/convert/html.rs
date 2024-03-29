use crate::Settings;


#[allow(unused)]
pub fn html(line: String, settings: &Settings) -> String {

    if line.trim().is_empty() {
            return "<br>".to_string();
    }
    let options = settings.html_change.clone();

    let mut data = line.split_whitespace().collect::<Vec<&str>>();

    let mut replaced_item = false;

    let first_val = data[0];
    data.remove(0);
    let mut combined_data = data.join(" ");
    let mut output = "".to_string();
    for i in options.iter() {
        if i[0] == first_val {
            replaced_item = true;
            output += format!("<{}>", i[1]).as_str();
            output += combined_data.as_str();
            output += format!(" </{}>", i[1]).as_str();
        }
    }

    if !replaced_item {
        return format!("<p>{}</p>", line);
    }
    
    output
}