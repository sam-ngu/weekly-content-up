use std::default::Default;


pub struct AppConfig{

  class_content_path: String,


}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
          class_content_path: String::from("01-Class-Content"),
        }
    }
}

