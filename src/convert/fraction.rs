
#[allow(unused)]
pub fn fractions(line: String) -> String {

    let line_list = line.split("}$").collect::<Vec<&str>>();
    if line_list.len() == 1 {
        return line;
    }

    let mut output = String::new();
    for item in line_list.iter() {
        let item_list = item.split("$\\frac{").collect::<Vec<&str>>();
        if item_list.len() == 1 {
            output += item_list[0];
            continue;
        }

        let pre = item_list[0];
        let arguments = item_list[1].split("}{").collect::<Vec<&str>>();

        output += format!("{}<sup>{}</sup>&frasl;<sub>{}</sub>", pre, arguments[0], arguments[1]).as_str();

    }

    output
}