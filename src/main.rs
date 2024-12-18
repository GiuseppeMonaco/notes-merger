use clap::Parser;
use indicatif::{ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle};
use std::{fs, path::PathBuf, process::exit};

use samsung_notes_merger::{get_notes, merge_images, open_images, visit_dirs};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "\x1b[1m\x1b[4mSamsung Note Merger\x1b[0m\nA tool for merging images exported from Samsung Notes app "
)]
struct Args {
    /// Directory da cui prendere le note
    #[arg(short, long, default_value_t = String::from("./"))]
    input_folder: String,

    /// Directory dove salvare le note mergate
    #[arg(short, long, default_value_t = String::from("./out/"))]
    output_folder: String,

    /// Mostra una preview delle note senza salvarle
    #[arg(short, long, default_value_t = false)]
    dry: bool,
}

pub fn main() {
    let args = Args::parse();

    let input_folder = PathBuf::from(args.input_folder);

    let output_folder = PathBuf::from(&args.output_folder);

    // Controllo validità input folder

    match input_folder.try_exists() {
        Ok(exist) => {
            if !exist {
                println!("La directory specificata come input non esiste.");
                exit(-1);
            }
        }
        Err(_) => {
            println!("Non riesco a verificare se la directory esiste.");
            exit(-1);
        }
    }

    if !input_folder.is_dir() {
        println!("Il percorso specificato come input non è una directory.");
        exit(-1);
    }

    // Controllo validità output folder

    if !output_folder.is_dir() {
        match output_folder.try_exists() {
            Ok(exist) => {
                if exist {
                    println!(
                        "Il percorso specificato come output esiste già e non è una directory."
                    );
                    exit(-1);
                } else {
                    fs::create_dir(output_folder).unwrap_or_else(|_| {
                        println!("Non posso creare la directory specificata come output");
                        exit(-1);
                    })
                }
            }
            Err(_) => {
                println!("Non riesco a verificare se la directory esiste.");
                exit(-1);
            }
        }
    }

    let img_paths = match visit_dirs(input_folder) {
        Ok(a) => a,
        Err(_) => {
            println!("Non sono riuscito a leggere le cartelle.");
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
                    "Non sono riuscito ad aprire l'immagine {:?}. Salto la nota {}",
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
                println!("Impossibile salvare la nota {}: {:?}", note.0, err)
            }
        };
    }
    println!("Done!");
}
