use directories::UserDirs;
use std::fs;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "fops")]
#[command(before_help = include_str!("./ascii.txt"))] 
#[command(version = "0.9.3")]
#[command(about = "Shitcoded tool that can put shitty files into shitty user folders automatically", long_about = None)]
struct Cli {
    #[arg(default_value = ".")] 
    target_path: PathBuf, 
    #[arg(short = 'd', long)]
    dry_run: bool,
    #[arg(short = 'v', long)]
    verbose: bool,
    #[arg(short = 'r', long)]
    recursive: bool,
}
fn main() {
    env_logger::init(); //Initialize logger
    let args = Cli::parse();
    let user_dirs = UserDirs::new().expect("Unable to find user directories"); //Get user directories
    let target_path = args.target_path; //Get target path
    let files = match fs::read_dir(target_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    }; //Get actual files info
    for file in files {
        //Loop through files
        let item = match file {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return;
            }
        };
    
        //println!("{:#?}", item.file_name());
        let path = item.path();
        let file_name = path.file_stem().expect("Failed to get file name, illegak file name?");
        if path.is_file() {
            //Pass if file
            if let Some(path_ext) = path.extension()
                && let Some(path_ext_str) = path_ext.to_str()
            {
                //Get extension string
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
                    "jpg" | "png" | "jpeg" | "gif" | "bmp" | "tiff" | "webp" | "ico" | "svg" => {
                        user_dirs.picture_dir()
                    }
                    "mp3" | "wav" | "flac" | "ogg" | "aiff" | "wma" | "m4a" => {
                        user_dirs.audio_dir()
                    }
                    "mp4" | "avi" | "mov" | "wmv" | "flv" | "mkv" | "webm" | "ogv" | "mpg"
                    | "mpeg" | "m4v" => user_dirs.video_dir(),
                    _ => None,
                };
                // Prepare to move file
                if let Some(target_dir) = target_dir_option {
                    let destination = target_dir.join(path.file_name().unwrap());
                    // Check if file exists
                    let mut name_check = destination.clone();
                    let mut counter = 1;
                    while name_check.exists() {
                        let name_append = format!(
                            "{}_{}.{}",
                            file_name.to_string_lossy(),
                            counter,
                            path_ext_str
                        );
                        // dbg!(&name_append);
                        counter += 1;
                        name_check = target_dir.join(name_append);
                    }
                    // dbg!(&name_check);
                    match fs::rename(&path, &name_check) {
                        Ok(_) => {
                            log::info!("Moved {} to {}", path.display(), name_check.display());
                        }
                        Err(e) => {
                            log::error!("Error moving {}: {}", path.display(), e);
                        }
                    }
                };
            }
        } else if file_name.to_string_lossy().starts_with(".") {
            log::debug!(
                "Skipping hidden file {}",
                item.file_name().to_string_lossy()
            ) //.file check
        } else {
            log::debug!(
                "{} is not a file, skipping",
                item.file_name().to_string_lossy()
            ); //Directory check
        }
    }
    // println!("{:#?}", args);
    // println!("{}", args.len());
}
