use crate::config::app::AppConfig;
use fs_extra::{self, dir::CopyOptions};
use std::{
    fs::{self, create_dir_all},
    path::Path,
};
use walkdir::{WalkDir, DirEntry};

fn src_folder() -> String {
    String::from("/home/sam/Development/trilogy/bootcampsrc-uwa")
}

fn target_folder() -> String {
    String::from("/home/sam/Development/trilogy/WAUS-VIRT-FSF-PT-05-2022-U-LOLC")
}

fn get_class_content_path() -> String {
    format!("{}/01-Class-Content", src_folder())
}

fn get_week_folder_path(week_num: &String) -> String {
    let class_content_path = get_class_content_path();

    // find the weekfolder
    let dirs = fs::read_dir(class_content_path).expect("can't find direct");

    let filtered: Vec<String> = dirs
        .into_iter()
        .filter_map(|dir| {
            let entry = dir.unwrap();
            let name = entry.file_name().into_string().unwrap();

            if name.starts_with(week_num) {
                Some(format!("{}", entry.path().to_string_lossy()))
            } else {
                None
            }
        })
        .collect();

    filtered.first().expect("Not found").to_owned()
}

fn get_week_name_from_path(week_folder_path: &String) -> String {
    let splitted = week_folder_path.split("/").collect::<Vec<&str>>();
    splitted
        .last()
        .expect("Cannot split folder name")
        .to_string()
}

fn copy_files(from: &String, to: &String) -> bool {
    let from_path = Path::new(&from);
    let to_path = Path::new(&to);
    if !to_path.exists() {
        create_dir_all(to_path).expect("failed to create directories");
    }
    let base_options = CopyOptions::new();
    let copy_options = CopyOptions {
        overwrite: true,
        ..base_options
    };
    let result = fs_extra::dir::copy(from_path, to_path, &copy_options);

    result.expect("cannot copy");
    true
}

pub fn copy_week_content(mut week_num: String) {
    if week_num.len() == 1 {
        week_num = format!("0{}", week_num);
    }

    let week_folder_path = get_week_folder_path(&week_num);

    let week_title = get_week_name_from_path(&week_folder_path);

    let src_activities = format!("{}/01-Activities", String::from(week_folder_path));
    let dest_activities = format!("{}/{}", String::from(target_folder()), week_title);

    // copy the activities
    println!("{}", src_activities);
    println!("{}", dest_activities);

    // copy_files(&src_activities, &dest_activities);

}

pub fn remove_solved_from(folder: &String){
  let folder_path = Path::new(folder);

  
  WalkDir::new(folder_path)
    .follow_links(true)
    .into_iter()
    .for_each(|res|{
      let entry = res.expect("folder not found?");
      if entry.path().is_dir() && entry.path().ends_with("Solved") {

        let result = fs::remove_dir_all(entry.path());
        println!("Deleting {:?}", entry.path());
        result.expect("Cannot delete folder!");
      
      }
    });

  
}
