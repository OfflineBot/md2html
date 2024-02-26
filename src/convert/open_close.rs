
#[allow(unused)]
fn replace_item(mut line: String, from: &str, to: &str) -> String {

    line += from;
    let mut output = String::new();

    let split = line.split(from).collect::<Vec<&str>>();
    let mut counter = 0;
    for i in split.iter() {
        if counter % 2 == 0 {
            output += format!("{}<{}>", i, to).as_str();
            counter += 1;
        } else {
            output += format!("{}</{}>", i, to).as_str();
            counter += 1;
        }
    }
    let replacement = format!("<{}></{}>", &to, &to);
    output = output.replace(replacement.as_str(), "");
    output

}

#[allow(unused)]
pub fn open_close(line: String) -> String {

    let list = vec![
        ["**", "bold"],
        ["__", "bold"],
    ];

    let mut output = line.clone();

    for i in list.iter() {
        if output.contains(i[0]) {
            output = replace_item(line.clone(), i[0], i[1]);
        }
    }     

    output
}