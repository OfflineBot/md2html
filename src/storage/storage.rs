

#[allow(unused)]
pub struct PathStorage {
    pub main_path: String,
    pub target: String,
}

impl PathStorage {
    #[allow(unused)]
    pub fn new(main_path: String, target: String) -> Self {
        PathStorage { main_path, target }
    }

    #[allow(unused)]
    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }

    #[allow(unused)]
    pub fn set_main_path(&mut self, main_path: String) {
        self.main_path = main_path;
    }
}