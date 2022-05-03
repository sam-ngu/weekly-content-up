use walkdir::WalkDir;

fn is_hidden_folder(folder_name: &String) -> bool{
  folder_name.starts_with(".")
}

pub fn scan(dir_path: &String) -> Result<(), std::io::Error> {
    let directories = WalkDir::new(dir_path).follow_links(true).into_iter();


    for entry in WalkDir::new(dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();
        let modified_at = entry.metadata()?.modified()?;

        let metadata = entry.metadata().expect("Cant get metadata");

        if( metadata.is_dir() && !is_hidden_folder(&filename.to_string()) ){
          println!("{:?}", filename);
        }

    }

    Ok(())
}
