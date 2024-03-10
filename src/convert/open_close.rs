use crate::Settings;


#[allow(unused)]
fn replace_item(mut line: String, key: &str, from: &str, to: &str) -> String {

    line += key;
    let mut output = String::new();

    let split = line.split(key).collect::<Vec<&str>>();
    let mut counter = 0;
    for i in split.iter() {
        if counter % 2 == 0 {
            output += format!("{}{}", i, from).as_str();
            counter += 1;
        } else {
            output += format!("{}{}", i, to).as_str();
            counter += 1;
        }
    }
    let replacement = format!("{}{}", &from, &to);
    output = output.replace(replacement.as_str(), "");
    output

}

#[allow(unused)]
pub fn open_close(line: String, settings: &Settings) -> String {

    let list = settings.start_end_case.clone(); 

    let mut output = line.clone();

    for i in list.iter() {
        if output.contains(&i[0]) {
            output = replace_item(line.clone(), &i[0], &i[1], &i[2]);
        }
    }     

    output
}