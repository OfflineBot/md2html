
fn find_file(name: String, all_files: Vec<String>) -> String {
    for i in all_files.iter() {
        if i.contains(&name) {
            return i.to_owned();
        }
    }
    name
}

pub fn link(line: String, all_files: Vec<String>, main_path: String) -> String {
    if !line.contains("]]") { return line; }

    let split = line.split("]]").collect::<Vec<&str>>();
    let mut output = String::new();
    for item in split.iter() {

        let new_split = item.split("[[").collect::<Vec<&str>>();
        if new_split.len() == 1 {continue;}
        let names = new_split[1].split("|").collect::<Vec<&str>>();

        let location = names[0];
        let mut location_new = find_file(names[0].to_string(), all_files.clone());
        location_new = location_new.replace(&main_path, "");
        let location_split = location_new.split("/").collect::<Vec<&str>>();
        let location_path_points = location_split.len() - 2;

        let mut from_main = String::from("./");
        for _ in 0..location_path_points {
            from_main += "../";
        }

        let name = match names.len() > 1 { true => names[1].to_string(), false => names[0].to_string()};

        output += format!("{}<a href=\"{}{}\">{}</a>", new_split[0], from_main, location, name).as_str();
    }
    println!("link_line: {}", output);
    output
}
