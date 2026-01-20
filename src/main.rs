use std::env;
use std::fs;
use directories::UserDirs;
fn main() {
    use env::args;
    let args:Vec<String> = args().collect(); //Get arguments
    if args.len() <= 2 {
        eprintln!("Insufficient arguments, need at least 3 arguments");
        return;} //Check arguments
    let user_dirs = UserDirs::new().unwrap(); //Get user directories
    let target_path = args.last().unwrap(); //Get target path
    let files = fs::read_dir(target_path).unwrap(); //Get actual files info
    for file in files { //Loop through files
        let item = file.unwrap();
        //println!("{:#?}", item.file_name());
        let path = item.path();
        let file_name = path.file_stem().unwrap();
        if path.is_file() { //Pass if file
            if let Some(path_ext) = path.extension() && let Some(path_ext_str) = path_ext.to_str() { //Get extension string
                    //println!("{}", path_str);
                    // if path_str == "jpg" || path_str == "png" || path_str == "jpeg" || path_str == "gif" || path_str == "bmp" || path_str == "tiff" || path_str == "webp" || path_str == "ico" || path_str == "svg"{
                    //     println!("Image");
                    // }
                    // else if path_str == "mp4" || path_str == "avi" || path_str == "mov" || path_str == "wmv" || path_str == "flv" || path_str == "mkv" || path_str == "webm" || path_str == "ogv" || path_str == "mpg" || path_str == "mpeg" || path_str == "m4v"{
                    //     println!("Video");
                    // }
                    // else if path_str == "mp3" || path_str == "wav" || path_str == "flac" || path_str == "ogg" || path_str == "aiff" || path_str == "wma" || path_str == "m4a"{
                    //     println!("Audio");
                    // }
                    // else{
                    //     println!("Other");
                    // }
                    let target_dir_option = match path_ext_str {
                        "jpg" | "png" | "jpeg" | "gif" | "bmp" | "tiff" | "webp" | "ico" | "svg" => user_dirs.picture_dir(),
                        "mp3" | "wav" | "flac" | "ogg" | "aiff" | "wma" | "m4a" => user_dirs.audio_dir(),
                        "mp4" | "avi" | "mov" | "wmv" | "flv" | "mkv" | "webm" | "ogv" | "mpg" | "mpeg" | "m4v" => user_dirs.video_dir(),
                        _ => None,
                    };
                    // Prepare to move file
                    if let Some(target_dir) = target_dir_option {
                        let destination = target_dir.join(path.file_name().unwrap());
                        // Check if file exists
                        let mut name_check = destination.clone();
                        let mut counter = 1;
                        while name_check.exists() {
                            let name_append = format!("{}_{}.{}", file_name.to_str().unwrap(), counter, path_ext_str);
                            // dbg!(&name_append);
                            counter += 1;
                            name_check = target_dir.join(name_append);
                        }
                        // dbg!(&name_check);
                        fs::rename(path, name_check); //TODO:catch err
                    };
            }
        } else if item.file_name().to_str().unwrap().starts_with(".") {
            log::debug!("Skipping hidden file {}", item.file_name().to_str().unwrap()) //.file check
        } else {
            log::debug!("{} not a file, skipping", item.file_name().to_str().unwrap()); //Directory check
        }
    }
    // println!("{:#?}", args);
    // println!("{}", args.len());
}
