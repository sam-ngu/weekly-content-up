mod utils;
mod uploader;
mod config;

use dotenvy::dotenv;
use utils::scandir;
use uploader::{content_uploader, gitlab};


fn main() {
    dotenv().ok();
    
    let week_num = String::from("1");
    let dest = String::from("/home/sam/Development/trilogy/WAUS-VIRT-FSF-PT-05-2022-U-LOLC");
    
    // content_uploader::copy_week_content(week_num, &dest);
    // content_uploader::remove_solved_from(&dest);
    // files_filter::add_solved_to(&week_num, &dest);


    // content_uploader::add_homework_to(&week_num, &dest);
    

    gitlab::commit("testing2");
    gitlab::push();



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
