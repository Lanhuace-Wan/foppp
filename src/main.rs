use directories::UserDirs;
use std::path::Path;
use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;
use std::fs;
use std::io;
use log::LevelFilter;

#[derive(Parser)]
#[command(name = "fop")]
#[command(before_help = include_str!("./ascii.txt"))] 
#[command(version = "0.9.5")]
#[command(about = "Shitcoded tool that can put shitty files into shitty user folders automatically, made with pure hate and guide from Gemini", long_about = None)]
struct Cli {
    #[arg(default_value = ".")] 
    target_path: PathBuf, 
    #[arg(short = 'd', long)]
    dry_run: bool,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short = 'r', long)]
    recursive: bool,
}
fn main() {
    let args = Cli::parse();
    let log_level = match args.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug
    }; //Set log level
    env_logger::Builder::new().filter_level(log_level).format_timestamp(None).init();
    let user_dirs = UserDirs::new().expect("Unable to find user directories"); //Get user directories
    let target_path = args.target_path; //Get target path
    let depth = if args.recursive {usize::MAX} else {1}; //Get depth
    let walker = WalkDir::new(&target_path)//Initialize logger
        .min_depth(1)
        .max_depth(depth)
        .into_iter(); //Get actual files info and do recursive search
    for entry in walker {
        //Loop through files
        let entry = match entry {
            Ok(f) => f,
            Err(e) => {
                log::error!("Error reading file: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let file_name = path.file_stem().expect("Failed to get file name, illegal name?");
        if file_name.to_string_lossy().starts_with(".") {
            log::debug!(
                "Skipping hidden file {}",
                entry.file_name().to_string_lossy()
            ) //Hidden file check
        } else if path.is_file() {
            //Pass if file, into one file stage
            if let Some(path_ext_str) = path.extension().and_then(|path_ext| path_ext.to_str()) {
                //Get extension string
                let target_dir_option = extension_check(path_ext_str, &user_dirs);
                // Prepare to move file
                if let Some(target_dir) = target_dir_option {
                    let destination = target_dir.join(path.file_name().unwrap());
                    if path.parent() == destination.parent() {
                    log::debug!("Wait target is already in destination path I will just kill myself");
                    continue;
                    }
                    // Check if file exists
                    let mut name_check = destination.clone();
                    let mut counter = 1;
                    while name_check.exists() { //Renaming duplicate file
                        let name_append = format!( //Glue name together
                            "{}_{}.{}",
                            file_name.to_string_lossy(),
                            counter,
                            path_ext_str
                        );
                        counter += 1;
                        name_check = target_dir.join(name_append);
                    }
                    if args.dry_run {
                        println!("Dryrun result: Moved {} to {}", path.display(), name_check.display());
                    } else {
                        match move_file(path, &name_check) { //Moving file
                            Ok(_) => {
                                log::info!("Moved {} to {}", path.display(), name_check.display());
                            }
                            Err(e) => {
                                log::error!("Error moving {}: {}", path.display(), e);
                            }
                        }
                    }
                };
            }
        } else {
            log::debug!(
                "{} is not a file, skipping",
                entry.file_name().to_string_lossy()
            ); //Directory check
        }
    }
    // println!("{:#?}", args);
    // println!("{}", args.len());
}

fn extension_check (ext_str: &str, user_dirs: &UserDirs) -> Option<PathBuf> {
    let extens = ext_str.to_lowercase();
    match extens.as_str() {
        "jpg" | "png" | "jpeg" | "gif" | "bmp" | "tiff" | "webp" | "svg" => user_dirs.picture_dir().map(|p| p.to_path_buf()),
        "mp3" | "wav" | "flac" | "ogg" | "aiff" | "wma" | "m4a" => user_dirs.audio_dir().map(|p| p.to_path_buf()),
        "mp4" | "avi" | "mov" | "wmv" | "flv" | "mkv" | "webm" | "ogv" | "mpg" | "mpeg" | "m4v" => user_dirs.video_dir().map(|p| p.to_path_buf()),
        "txt" | "docx" | "doc" | "pdf" | "rtf" | "csv" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp" | "key" => user_dirs.document_dir().map(|p| p.to_path_buf()),
        "ttf" | "otf" | "woff" | "woff2" | "eot" => user_dirs.font_dir().map(|p| p.to_path_buf()),
        _ => None,
    }
}

fn move_file(source: &Path, destination: &Path) -> io::Result<()> {
    match fs::rename(source, destination) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("Moving error: {}", e);
            fs::copy(source, destination)?;
            log::debug!("Trying copy mode from {} to {}", source.to_string_lossy(), destination.to_string_lossy());
            trash::delete(source).map_err(|e| {io::Error::other(e.to_string())})

        }
    }
}