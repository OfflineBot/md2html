
#[allow(unused)]
pub fn html(line: String) -> String {

    let options = vec![
        ["#", "h1"],
        ["##", "h2"],
        ["###", "h3"],
        ["####", "h4"],
        ["#####", "h5"],
        ["######", "h6"],
        ["-", "li"],
        ["+", "li"],
    ];

    let mut output = line.clone();
    let data = line.split_whitespace().collect::<Vec<&str>>();
    if data.len() <= 1 {
        if output.trim().is_empty() {
            return "<br>".to_string();
        }
        return format!("<p>{} </p>", output);
    }

    for i in options.iter() {
        if i[0] == data[0] {
            output = output.replace(i[0], format!("<{}>", i[1]).as_str());
            output += format!(" </{}>", i[1]).as_str();
        }
    }
    
    output
}