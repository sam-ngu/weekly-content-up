use walkdir::WalkDir;
use std::fs;
use crate::config::app::AppConfig;



fn src_folder() -> String{
  String::from("/home/sam/Development/trilogy/bootcampsrc-uwa")
}

fn get_class_content_path()-> String{
  format!("{}/01-Class-Content", src_folder())
}

fn get_week_folder_path(week_num: &String) -> String{

  let class_content_path = get_class_content_path();

  // find the weekfolder
  let dirs = fs::read_dir(class_content_path).expect("can't find direct");

  let filtered: Vec<String> = dirs.into_iter().filter_map(|dir|{

    let entry = dir.unwrap();
    let name = entry.file_name().into_string().unwrap();

    if name.starts_with(week_num) {
      Some(format!("{}", entry.path().to_string_lossy()))
    }else {
      None
    }

  }).collect();

  filtered.first().expect("Not found").to_owned()
  
}


pub fn get_unsolved(mut week_num: String){

  if week_num.len() == 1 {
    week_num = format!("0{}", week_num);
  }
  

  println!("{:?}", week_num);

  let class_content_path = "01-Class-Content";


  let week_folder_path = get_week_folder_path(&week_num);

  println!("{}", week_folder_path);

  let activities_path = format!("{}/01-Activities", String::from(week_folder_path));

  // copy the activities 

  // WalkDir::new(root)


}

