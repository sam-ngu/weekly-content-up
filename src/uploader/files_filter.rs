use crate::config::app::AppConfig;
use fs_extra::{self, dir::CopyOptions};
use std::{
    fs::{self, create_dir_all},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

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
    let week_num = get_week_num(week_num.to_owned());
    let class_content_path = get_class_content_path();

    // find the weekfolder
    let dirs = fs::read_dir(class_content_path).expect("can't find direct");

    let filtered: Vec<String> = dirs
        .into_iter()
        .filter_map(|dir| {
            let entry = dir.unwrap();
            let name = entry.file_name().into_string().unwrap();

            if name.starts_with(&week_num) {
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

fn get_week_title(week_num: &String) -> String {
    let week_folder_path = get_week_folder_path(week_num);
    get_week_name_from_path(&week_folder_path)
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

fn for_each_solved_folder_from(folder: &String, iteratee: impl Fn(DirEntry)) {
    let folder_path = Path::new(folder);
    WalkDir::new(folder_path)
        .follow_links(true)
        .into_iter()
        .for_each(|res| {
            let entry = res.expect("folder not found?");
            if entry.path().is_dir() && entry.path().ends_with("Solved") {
                iteratee(entry);
            }
        });
}
fn get_week_num(week_num: String) -> String {
    if week_num.len() == 1 {
        return format!("0{}", week_num);
    }
    week_num
}
pub fn copy_week_content(week_num: String) {
    let week_num = get_week_num(week_num);

    let week_folder_path = get_week_folder_path(&week_num);

    let week_title = get_week_name_from_path(&week_folder_path);

    let src_activities = format!("{}/01-Activities", String::from(week_folder_path));
    let dest_activities = format!("{}/{}", String::from(target_folder()), week_title);

    // copy the activities
    println!("{}", src_activities);
    println!("{}", dest_activities);

    copy_files(&src_activities, &dest_activities);
}

pub fn remove_solved_from(week_num: &String, folder: &String) {
    let week_folder = get_week_title(week_num);
    println!("week num: {}", week_num);
    println!("week folder: {}", week_folder);

    for_each_solved_folder_from(folder, |entry| {
        let result = fs::remove_dir_all(entry.path());
        println!("Deleting {:?}", entry.path());
        result.expect("Cannot delete folder!");
    });
}

pub fn add_solved_to(week_num: &String, dest_folder: &String) {
    // dest_folder is "/home/sam/Development/trilogy/WAUS-VIRT-FSF-PT-05-2022-U-LOLC"

    let week_num = get_week_num(week_num.to_owned());

    let src_week_folder = get_week_folder_path(&week_num.to_owned());
    let week_title = get_week_name_from_path(&src_week_folder);

    let dest_folder_path = Path::new(dest_folder);

    for_each_solved_folder_from(&src_week_folder, |entry| {
        let solved_path = String::from(entry.path().to_string_lossy());
        println!("{:?}", solved_path);

        let splitted: Vec<&str> = solved_path.split("/").collect();
        let activity_name = splitted[splitted.len() - 2];
        

        let copy_to = format!("{}/{}/01-Activities/{}", dest_folder, week_title, activity_name);
        println!("copy to: {}", copy_to);
        let from = String::from(entry.path().to_string_lossy());
        // let copy_to_path = Path::new();
        copy_files(&from, &copy_to);
    });
}
