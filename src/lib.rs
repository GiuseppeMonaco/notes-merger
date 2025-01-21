use std::{
    collections::HashMap,
    env, fs, io,
    path::{Path, PathBuf},
};

use image::{
    DynamicImage, GenericImageView, ImageBuffer, ImageError, ImageReader, Rgba, RgbaImage,
};
use regex::Regex;

pub fn get_folder() -> Option<PathBuf> {
    let mut args = env::args();
    args.next();
    Some(PathBuf::from(args.next()?))
}

pub fn merge_images(imgs: &Vec<DynamicImage>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    for img in imgs {
        height = height + img.height();

        if width < img.width() {
            width = img.width();
        }
    }

    let mut merged_img: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

    let mut current_height: u32 = 0;
    for img in imgs {
        for y in 0..img.height() {
            for x in 0..img.width() {
                merged_img.put_pixel(x, y + current_height, img.get_pixel(x, y));
            }
        }
        current_height = current_height + img.height();
    }

    return merged_img;
}

pub fn open_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, ImageError> {
    ImageReader::open(path)?.decode()
}

pub fn open_images(paths: &Vec<PathBuf>) -> Result<Vec<DynamicImage>, &PathBuf> {
    let mut ret = Vec::<DynamicImage>::new();
    for path in paths {
        let img = match open_image(&path) {
            Ok(img) => img,
            Err(_) => return Err(path),
        };
        ret.push(img);
    }
    Ok(ret)
}

pub fn visit_dirs<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    let mut list: Vec<PathBuf> = vec![];
    visit_dirs_rec(dir, &mut list)?;
    Ok(list)
}

fn visit_dirs_rec<P: AsRef<Path>>(dir: P, list: &mut Vec<PathBuf>) -> io::Result<()> {
    let dir = dir.as_ref();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs_rec(&path, list)?;
            } else {
                list.push(path);
            }
        }
    }
    Ok(())
}

fn get_note_name(path: &PathBuf) -> Option<String> {
    let note_name = path.to_str()?.to_string();

    let re = Regex::new(r".*[\/\\](.*?)_.*(?:\.png|\.jpeg|\.jpg)").unwrap();
    let mut note_name = re
        .captures(&note_name)?
        .get(1)
        .map(|m| m.as_str().to_string())?;
    note_name.push_str(".png");
    Some(note_name)
}

#[cfg(test)]
mod get_note_name_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test1() {
        assert_eq!(
            "Note 1.png",
            get_note_name(
                &PathBuf::from_str("/some/folder/nesting/Note 1_240923_105036.jpg").unwrap()
            )
            .unwrap()
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            "Note 2.png",
            get_note_name(&PathBuf::from_str("./Note 2_123.png").unwrap()).unwrap()
        )
    }

    #[test]
    fn test3() -> Result<(), String> {
        match get_note_name(&PathBuf::from_str("/Note 1.jpeg").unwrap()) {
            Some(value) => Err(String::from(format!(
                "Function shouldn't return nothing, '{}' returned instead",
                value
            ))),
            None => Ok(()),
        }
    }
}

pub fn get_notes(
    img_paths: Vec<PathBuf>,
) -> Result<HashMap<String, Vec<PathBuf>>, HashMap<String, Vec<PathBuf>>> {
    let mut notes = HashMap::<String, Vec<PathBuf>>::new();
    let mut some_not_file = false;

    for path in img_paths {
        let name = match get_note_name(&path) {
            Some(name) => name,
            None => {
                some_not_file = true;
                continue;
            }
        };

        if !notes.contains_key(&name) {
            notes.insert(name.clone(), vec![]);
        }

        notes.get_mut(&name).unwrap().push(path);
    }

    for path in notes.iter_mut() {
        path.1.sort();
    }
    if some_not_file {
        Err(notes)
    } else {
        Ok(notes)
    }
}

// Generate a confirmation
pub fn confirmation_prompt() -> bool {
    let mut string = String::new();
    match io::stdin().read_line(&mut string) {
        Ok(_) => (),
        Err(_) => return false,
    };
    string = string.trim().to_lowercase();
    match string.as_str() {
        "y" => true,
        "yes" => true,
        _ => false,
    }
}

pub fn get_printable_notes_list(notes: HashMap<String, Vec<PathBuf>>) -> String {
    let mut ret = String::from("\x1b[1mNotes after merging:\x1b[0m\n");
    for note in notes {
        ret.push_str("    \x1b[4m");
        ret.push_str(note.0.as_str());
        ret.push_str("\x1b[0m\n");
        let mut c: u32 = 0;
        for path in note.1 {
            c = c + 1;
            ret.push_str("        ");
            ret.push_str(c.to_string().as_str());
            ret.push_str(") \"");
            ret.push_str(path.to_str().unwrap_or(""));
            ret.push_str("\"\n");
        }
    }
    ret
}
