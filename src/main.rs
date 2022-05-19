mod utils;
mod uploader;
mod config;

use config::app_config::AppConfig;
use dotenvy::dotenv;
use inquire::{Text, validator::StringValidator, Select, Confirm};
use utils::scandir;
use uploader::{content_uploader, gitlab};


fn main() {
    dotenv().ok();
    
    let app_config = AppConfig::new();

    let dest = String::from(app_config.class_repo_path);
    
    

    // content_uploader::add_solved_to(&week_num, &dest);
    


    // which week to upload?
    let numeric_string_validator: StringValidator = &|input| {
        if input.parse::<f64>().is_ok(){
            Ok(())
        }else{
            Err("Only numerical week is allowed!".to_string())
        }
    };
    let week_num = Text::new("Which week do you want to upload?")
        .with_validator(numeric_string_validator)
        .prompt()
        .expect("can't get input");


    // Upload solved or unsolved?
    let options = vec!["solved", "unsolved"];
    let upload_type = Select::new("Upload solved or unsolved?", options)
        .prompt().expect("cannot ask selection");

    let push_now = Confirm::new("Push to gitlab?").prompt().expect("cannot ask gitlab");


    println!("{}", week_num);
    

    if upload_type == "solved"{
        content_uploader::add_solved_to(&week_num, &dest);
    }else {
        content_uploader::copy_week_content(&week_num, &dest);
        
        let week_name = content_uploader::get_week_title(&week_num);

        let dest_week_folder = format!("{}/{}", dest, week_name);
        content_uploader::remove_solved_from(&dest_week_folder);

        content_uploader::add_homework_to(&week_num, &dest);
    }

    let commit_msg = format!("added week {} {}", week_num, upload_type);

    if push_now {
        gitlab::commit(&commit_msg);  
        gitlab::push();
    }
}
