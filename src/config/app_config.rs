use std::default::Default;


pub struct AppConfig{

  // pub class_content_path: String,

  // the src material path
  pub src_path: String,

  // dest to class content
  pub class_repo_path: String,

}

// impl Default for AppConfig {
//     fn default() -> Self {
//         Self {
//           class_content_path: String::from("01-Class-Content"),
//         }
//     }
// }

impl AppConfig {
    pub fn new()-> Self {
      Self {

          src_path: String::from("/home/sam/Development/trilogy/bootcampsrc-uwa"),
          
          class_repo_path: String::from("/home/sam/Development/trilogy/WAUS-VIRT-FSF-PT-05-2022-U-LOLC"),
        //  class_content_path: String::from("01-Class-Content"),

      }
    }
}

