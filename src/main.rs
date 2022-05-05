mod utils;
mod uploader;
mod config;

use dotenvy::dotenv;
use utils::scandir;
use uploader::files_filter;


fn main() {
    dotenv().ok();
    
    // files_filter::get_unsolved(String::from("1"));
    let dest = String::from("/home/sam/Development/trilogy/WAUS-VIRT-FSF-PT-05-2022-U-LOLC/01-HTML-Git-CSS");
    files_filter::remove_solved_from(&dest);

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
