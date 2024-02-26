use std::env;

use crate::storage::PathStorage;

#[allow(unused)]
fn info() {

}

#[allow(unused)]
fn create() {

}

#[allow(unused)]
fn setup() {

}


#[allow(unused)]
pub fn handle_args() -> Option<PathStorage> {

    let mut main_path = String::new();
    let mut target = "output".to_string();

    let mut path_storage = PathStorage::new(main_path, target);

    let argc = env::args().len();
    let argv: Vec<String> = env::args().collect();

    match argc {
        0 => println!("What happened? 0 args given."),
        1 => {
            info();
            return None;
        },
        _ => {
            for arg in argv.iter() {
                let values = arg.split("=").collect::<Vec<&str>>();
                match values[0] {
                    "create" => create(),
                    "setup" => setup(),
                    "target" => path_storage.set_target(values[1].to_string()),
                    _ => println!("Found invalid argument: {}", arg),
                };
            }
        } 
    }


    Some(path_storage)
}