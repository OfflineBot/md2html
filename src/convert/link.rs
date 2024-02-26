
fn find_file(name: String, all_files: Vec<String>) -> String {
    for i in all_files.iter() {
        if i.contains(&name) {
            return i.to_owned();
        }
    }
    name
}

pub fn link(line: String, all_files: Vec<String>) -> String {
    if !line.contains("]]") { return line; }

    let split = line.split("]]").collect::<Vec<&str>>();
    println!("line: {}", line);

    let mut output = String::new();
    for item in split.iter() {
        let new_split = item.split("[[").collect::<Vec<&str>>();
        let names = new_split[1].split("|").collect::<Vec<&str>>();

        let location = names[0];
        let location_new = find_file(names[0].to_string(), all_files.clone());
        let location_split = location_new.split("/").collect::<Vec<&str>>();
        let location_path_points = location_split.len();

        let mut from_main = String::from("./");
        for _ in 0..location_path_points {
            from_main += "../";
        }

        let name = match names.len() > 1 { true => names[1].to_string(), false => names[0].to_string()};

        output += format!("{}<a href=\"{}{}\">{}</a>", new_split[0], from_main, location, name).as_str();
    }

    line
}
