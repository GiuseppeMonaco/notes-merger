#![allow(unused)]

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, RgbaImage};
use regex::{CaptureNames, Regex};
use std::{
    collections::HashMap,
    env::{self, current_dir, Args},
    fs,
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

use samsung_notes_merger::{
    get_folder, get_note_name, get_notes, merge_images, open_image, open_images, visit_dirs,
};

pub fn main() {
    let folder = get_folder().unwrap_or_else(|| {
        current_dir().ok().unwrap_or_else(|| {
            println!("Devi specificare una cartella valida come argomento del programma.");
            exit(-1);
        })
    });

    match folder.try_exists() {
        Ok(exist) => {
            if !exist {
                println!("Il percorso specificato non esiste.");
                exit(-1);
            }
        }
        Err(_) => {
            println!("Non riesco a verificare se la cartella esiste.");
            exit(-1);
        }
    }

    if !folder.is_dir() {
        println!("Il percorso specificato non è una directory.");
        exit(-1);
    }

    let img_paths = match visit_dirs(folder) {
        Ok(a) => a,
        Err(_) => {
            println!("Non sono riuscito a leggere le cartelle.");
            exit(-1);
        }
    };

    let notes = get_notes(img_paths);

    for note in notes {
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
        merge_images(&images).save(note.0);
    }
}
