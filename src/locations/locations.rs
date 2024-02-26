
/// Locations to important paths
pub struct Locations {
    pub absolut_main: String,
    pub relative_index: String,
    pub target: String, 
}

impl Locations {
    pub fn new(absolut_path: String, relative_path_to_index: String, target_folder_name: String) -> Locations {
        Locations {absolut_main: absolut_path, relative_index: relative_path_to_index, target: target_folder_name}
    }
}