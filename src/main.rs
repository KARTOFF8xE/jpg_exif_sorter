use core::panic;
use std::{fs::{self}};
use image::{ImageError, io::Reader as ImageReader};

struct Picture {
    path: String,
    date: String,
    time: String,
    is_picture: bool,
}

impl Picture {

    fn convert(path: &str) -> Result<String, ImageError> {
        let mut new_path = path.chars().clone();
        print!("Converting {} ", path);
        let img = ImageReader::open(path)?.decode()?;
        
        for c in path.chars().rev() {
            if c == '.' { break; }
            new_path.next_back();
        };
        let new_path = format!("{}png", new_path.as_str());
        img.save(new_path.as_str()).expect("Couldn't save the Image");
        println!("-> {}", new_path);
        Ok(new_path.as_str().to_string())
    }

    fn new(input: Option<&str>) -> Picture {
        let i;
        i = match input {
            Some(i) => i,
            None => panic!("found Invalid Name"),
        };

        Picture { path: i.to_string(), date: "Date".to_string(), time: "Time".to_string(),
                is_picture: i.contains(".jpeg") || i.contains(".png") || i.contains(".jpg") }
    }

    fn printer(v: Vec<Self>) {
        println!("The EXIF-Data of the Pictures told me:");
        for p in v {
            if p.is_picture { println!("Path: {}\n\tDate: {}\n\tTime: {}", p.path, p.date, p.time); }
        }
    }
}

fn get_pictures() -> Vec<Picture> {
    let mut files: Vec<Picture> = Vec::new();
    let paths = fs::read_dir("./").expect("Couldn't read Paths");

    println!("I can see the following Data: ");
    for path in paths {
        println!("Name: {}", path.as_ref().unwrap().path().display());
        files.push(
            match path {
                Ok(pic) => Picture::new(pic.file_name().to_str()),
                Err(_) => panic!("Found invalid path"),
            }
        );
    }

    let mut pictures: Vec<Picture> = Vec::new();
    for mut p in files {
        if p.is_picture {
            let i = match Picture::convert(&p.path) {
                Ok(i) => i,
                Err(_) => panic!("Not able to convert")
            };
            p.path = i;

            pictures.push(p);
        }
    }

    pictures
}

fn main() {
    let v :Vec<Picture> = get_pictures();
    Picture::printer(v);
}
