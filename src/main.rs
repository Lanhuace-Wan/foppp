use std::env;
use std::fs;
use directories::UserDirs;
fn main() {
    use env::args;
    let args:Vec<String> = args().collect(); //Get arguments
    if args.len() <= 3 {eprintln!("Insufficient arguments, need at least 3 arguments");} //Check arguments
    let user_dirs = UserDirs::new().unwrap(); //Get user directories
    let target_path = args.last().unwrap(); //Get target path
    let files = fs::read_dir(target_path).unwrap();
    for file in files { //Loop through files
        let item = file.unwrap();
        //println!("{:#?}", item.file_name());
        let path = item.path();
        if path.is_file() { //Pass if file
            if let Some(path_ext) = path.extension() && let Some(path_str) = path_ext.to_str() { //Get extension string
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
                    let target_dir_option = match path_str {
                        "jpg" | "png" | "jpeg" | "gif" | "bmp" | "tiff" | "webp" | "ico" | "svg" => user_dirs.picture_dir(),
                        "mp3" | "wav" | "flac" | "ogg" | "aiff" | "wma" | "m4a" => user_dirs.audio_dir(),
                        "mp4" | "avi" | "mov" | "wmv" | "flv" | "mkv" | "webm" | "ogv" | "mpg" | "mpeg" | "m4v" => user_dirs.video_dir(),
                        _ => None,
                    };
                    if let Some(target_dir) = target_dir_option {
                        let destination = target_dir.join(path.file_name().unwrap());
                        fs::rename(item.path(), destination);
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
