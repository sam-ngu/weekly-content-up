mod utils;
mod uploader;
mod config;

use dotenvy::dotenv;
use utils::scandir;
use uploader::files_filter;


fn main() {
    dotenv().ok();
    
    files_filter::copy_week_content(String::from("1"));
    let week_num = String::from("1");
    let dest = String::from("/home/sam/Development/trilogy/WAUS-VIRT-FSF-PT-05-2022-U-LOLC");
    files_filter::remove_solved_from(&week_num,&dest);
    // files_filter::add_solved_to(&week_num, &dest);

    // let dir_path = String::from(".");
    // let result = scandir::scan(&dir_path);



    // TODO: parse args

    // expected args
    // 1. week num
    // 2. solved or unsolved

    // check if wanted solved or unsolved

    // if unsolved
    // copy all unsolved to gitlab repo

    // and push

    // if solved
    // copy all solved to gitlab repo

    // and push


    
}
