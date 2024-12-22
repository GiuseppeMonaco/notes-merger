use clap::Parser;
use indicatif::{ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle};
use std::{fs, path::PathBuf, process::exit};

use samsung_notes_merger::{get_notes, merge_images, open_images, visit_dirs};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "\x1b[1m\x1b[4mSamsung Note Merger\x1b[0m\nA tool for merging images exported from Samsung Notes app"
)]
struct Args {
    /// Directory to take notes from
    #[arg(short, long, default_value_t = String::from("./"))]
    input_folder: String,

    /// Directory where merged notes are saved
    #[arg(short, long, default_value_t = String::from("./out/"))]
    output_folder: String,

    /// Show a preview of notes without saving them
    #[arg(short, long, default_value_t = false)]
    dry: bool,
}

pub fn main() {
    let args = Args::parse();

    let input_folder = PathBuf::from(args.input_folder);

    let output_folder = PathBuf::from(&args.output_folder);

    // Check if input folder is valid

    match input_folder.try_exists() {
        Ok(exist) => {
            if !exist {
                println!("The directory specified as input does not exist.");
                exit(-1);
            }
        }
        Err(_) => {
            println!("Cannot check if the directory specified as input exists.");
            exit(-1);
        }
    }

    if !input_folder.is_dir() {
        println!("The path specified as input is not a directory.");
        exit(-1);
    }

    // Check if output folder is valid

    if !output_folder.is_dir() {
        match output_folder.try_exists() {
            Ok(exist) => {
                if exist {
                    println!("The path specified as output already exists and is not a directory.");
                    exit(-1);
                } else {
                    fs::create_dir(output_folder).unwrap_or_else(|_| {
                        println!("Cannot create directory specified as output");
                        exit(-1);
                    })
                }
            }
            Err(_) => {
                println!("Cannot verify if the directory exist.");
                exit(-1);
            }
        }
    }

    let img_paths = match visit_dirs(input_folder) {
        Ok(a) => a,
        Err(_) => {
            println!("Cannot read the folder.");
            exit(-1);
        }
    };

    let notes = get_notes(img_paths);

    if args.dry {
        dbg!(notes);
        exit(0);
    }

    let merge_progressbar = ProgressBar::new(notes.len() as u64)
        .with_prefix("Merging notes")
        .with_style(
            ProgressStyle::with_template("{prefix} [{bar:40}] {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("=> "),
        )
        .with_finish(ProgressFinish::Abandon);

    for note in notes.iter().progress_with(merge_progressbar) {
        let images = match open_images(note.1) {
            Ok(images) => images,
            Err(path) => {
                println!(
                    "Could not open the image: {:?}. Skipping note: {}",
                    path, note.0
                );
                continue;
            }
        };

        let mut output_folder = PathBuf::from(&args.output_folder);
        output_folder.push(&note.0);

        match merge_images(&images).save(output_folder) {
            Ok(_) => {}
            Err(err) => {
                println!("Unable to save note {}: {:?}", note.0, err)
            }
        };
    }
    println!("Done!");
}
