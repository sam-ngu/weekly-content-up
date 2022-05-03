mod utils;
use dotenvy::dotenv;
use utils::scandir;


fn main() {
    dotenv().ok();

    let dir_path = String::from(".");
    let result = scandir::scan(&dir_path);


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


    println!("Hello, world!");
}
