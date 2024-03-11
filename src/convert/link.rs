
fn find_file(name: String, all_files: Vec<String>, main_path: String) -> String {
    for i in all_files.iter() {
        if i.contains(&name) {
            let found = i.replace(&main_path, "").replace(".md", ".html");
            return found;
        }
    }
    name
}

#[allow(unused)]
pub fn link(line: String, all_files: Vec<String>, main_path: String, current_locations: String) -> String {
    if !line.contains("]]") { return line; }

    let split = line.split("]]").collect::<Vec<&str>>();
    let mut output = String::new();
    for item in split.iter() {

        let new_split = item.split("[[").collect::<Vec<&str>>();
        if new_split.len() == 1 {
            output += new_split[0];
            continue;
        }
        let names = new_split[1].split("|").collect::<Vec<&str>>();

        let mut location_new = find_file(names[0].to_string(), all_files.clone(), main_path.clone());
        location_new = location_new.replace(&main_path, "").replace("//", "/");
        let location_split = current_locations.split("/").collect::<Vec<&str>>();
        let location_path_points = location_split.len();

        let mut from_main = String::from(".");
        for _ in 0..location_path_points-2 {
            from_main += "/..";
        }

        let name = match names.len() > 1 { true => names[1].to_string(), false => names[0].to_string()};

        output += format!("{}<a href=\"{}/{}\">{}</a>", new_split[0], from_main, location_new, name).replace("//", "/").as_str();
    }
    output
}
