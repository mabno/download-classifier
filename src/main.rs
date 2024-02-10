use std::thread::sleep;
use std::time::Duration;

use std::{collections::HashMap, path::Path};
use std::fs;


fn get_classification() -> HashMap<String, String>{
    let mut classification: HashMap<String, String> = HashMap::new();

    let mut contents = fs::read_to_string("./classification.txt")
        .expect("Something went wrong reading the file");
    contents.retain(|c| !c.is_whitespace());
    let folders = contents.split(">");
    for folder in folders {
        let mut extensions = folder.split("-");
        let folder_name = extensions.nth(0);
        for extension in extensions {
            classification.insert(extension.to_string(), folder_name.unwrap().to_string());
        }
    }
    classification
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    let polling_interval = std::env::args()
        .nth(2)
        .expect("Argument 2 needs to be a number of seconds")
        .parse::<u64>()
        .expect("Argument 2 needs to be a number of seconds");

    loop {
        watch(&path);
    
        sleep(Duration::from_secs(polling_interval));
    }
}



fn watch(path: &str) {
    let classification = get_classification();
    fs::read_dir(&path).unwrap()
        .for_each(|entry| {
            let entry = entry.unwrap();
            if  entry.file_type().unwrap().is_dir() {
                return;
            }
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();

            if (filename.ends_with(".tmp") || filename.ends_with("crdownload")) {
                return;
            }
            
            let source_folder = path.parent().unwrap();
            let extension = filename.split(".").last().unwrap();
            match classification.get(extension) {
                Some(folder) => {
                    println!("Moving '{}' to '{}' category", filename, folder);
                    let dest_folder = format!("{}/{}", &source_folder.to_str().unwrap(), folder);
                    fs::create_dir_all(&dest_folder).unwrap();
                    fs::rename(entry.path(), format!("{}/{}", dest_folder, filename)).unwrap();
                },
                None => {
                    println!("Moving '{}' to 'other' category, because its extension is not classified", filename);
                    let dest_folder = format!("{}/{}", &source_folder.to_str().unwrap(), "other");
                    fs::create_dir_all(&dest_folder).unwrap();
                    fs::rename(entry.path(), format!("{}/{}", dest_folder, filename)).unwrap();
                }
            }
        });
}