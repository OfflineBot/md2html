

#[derive(Debug, Clone)]
pub struct Settings {
    pub html_change: Vec<[String; 2]>,
    pub replace_by: Vec<[String; 2]>,
    pub start_end_case: Vec<[String; 3]>,
    pub start_end_second: Vec<[String; 3]>,
}

impl Settings {
    #[allow(unused)]
    pub fn empty() -> Self {
        Settings { html_change: Vec::new(), replace_by: Vec::new(), start_end_case: Vec::new(), start_end_second: Vec::new() }
    } 

    #[allow(unused)]
    pub fn default() -> Self {
        let mut settings = Settings::empty();

        settings.add_html_change(["-".to_string(), "li".to_string()]);
        settings.add_html_change(["+".to_string(), "li".to_string()]);
        settings.add_html_change(["#".to_string(), "h1".to_string()]);
        settings.add_html_change(["##".to_string(), "h2".to_string()]);
        settings.add_html_change(["###".to_string(), "h3".to_string()]);
        settings.add_html_change(["####".to_string(), "h4".to_string()]);
        settings.add_html_change(["#####".to_string(), "h5".to_string()]);
        settings.add_html_change(["######".to_string(), "h6".to_string()]);

        settings.add_start_end(["**".to_string(), "<bold>".to_string(), "</bold>".to_string()]);
        settings.add_start_end(["__".to_string(), "<bold>".to_string(), "</bold>".to_string()]);
        settings.add_start_end_second(["$".to_string(), "\\(".to_string(), "\\)".to_string()]);


        //settings.add_replacement(["$\\Delta$".to_string(), "∆".to_string()]);
        //settings.add_replacement(["\\Delta".to_string(), "∆".to_string()]);
        //settings.add_replacement(["$\\Sigma$".to_string(), "∑".to_string()]);
        //settings.add_replacement(["\\Sigma".to_string(), "∑".to_string()]);
        //settings.add_replacement(["\\[".to_string(), "[".to_string()]);

        settings
    }

    #[allow(unused)]
    pub fn add_html_change(&mut self, data: [String; 2]) {
        self.html_change.push(data);
    }

    #[allow(unused)]
    pub fn add_replacement(&mut self, data: [String; 2]) {
        self.replace_by.push(data);
    }

    #[allow(unused)]
    pub fn add_start_end(&mut self, data: [String; 3]) {
        self.start_end_case.push(data);
    }

    #[allow(unused)]
    pub fn add_start_end_second(&mut self, data: [String; 3]) {
        self.start_end_second.push(data);
    }
}
