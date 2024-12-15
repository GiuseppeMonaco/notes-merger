#![allow(unused)]

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, RgbaImage};
use regex::Regex;
use std::{env, fs, path::PathBuf, str::FromStr};

use samsung_notes_merger::{get_note_name, merge_images, open_image, visit_dirs};

fn main_bak() {
    let paths: Vec<_> = std::fs::read_dir("imgs/in")
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();

    dbg!(&paths);

    for path in paths {
        let note_name = match get_note_name(&path) {
            Some(ret) => ret,
            None => continue,
        };

        if match fs::exists(PathBuf::from_str(&note_name).unwrap()) {
            Ok(bool) => bool,
            Err(_) => panic!("Impossibile verificare se il file esiste già."),
        } {
            println!("{} esiste già... skip", note_name);
            continue;
        }

        if !path.is_dir() {
            ImageReader::open(path)
                .unwrap()
                .decode()
                .unwrap()
                .save(note_name)
                .unwrap();
            continue;
        };

        let mut img_paths: Vec<String> = std::fs::read_dir(path)
            .unwrap()
            .map(|res| res.unwrap().path().to_str().unwrap().to_string())
            .collect();

        img_paths.sort();

        dbg!(&img_paths);
    }
}

fn esempio_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    println!("{}", {
        match args.get(1) {
            Some(arg) => arg,
            None => "Primo argomento non definito!",
        }
    });
    println!("{}", {
        match args.get(2) {
            Some(arg) => arg,
            None => "Secondo argomento non definito!",
        }
    });
    args
}

pub fn main_main() {
    loop {
        let img1 = match open_image("benches/assets/image_merge/img1.jpg") {
            Ok(img) => img,
            Err(err) => {println!("Impossibile aprire l'immagine: {:?}", err); continue},
        };
        let img2 = match open_image("benches/assets/image_merge/img2.jpg") {
            Ok(img) => img,
            Err(err) => {println!("Impossibile aprire l'immagine: {:?}", err); continue},
        };
        let imgs = vec![img1, img2];
        merge_images(&imgs).save("imgs/ciao.png");
    }
}

pub fn main() {
    dbg!(visit_dirs("imgs"));
}