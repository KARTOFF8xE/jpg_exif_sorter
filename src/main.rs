extern crate exif;

use core::panic;
use std::{fs::{{self}, File}};
use std::io::{{self}, BufReader};
use exif::{In, Reader, Tag};
use image::{ImageError, io::Reader as ImageReader};

struct Picture {
    path: String,
    date: String,
    timestamp: Option<i64>,
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
        let new_path = format!("{}JPEG", new_path.as_str());
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

        Picture { path: i.to_string(), date: "Date".to_string(),
                is_picture: i.contains(".jpeg") || i.contains(".png") || i.contains(".jpg"), timestamp: None}
    }

    fn printer(v: Vec<Self>) {
        println!("The EXIF-Data of the Pictures told me:");
        for p in v {
            if p.is_picture {
                println!("Path: {}\n\tDate: {}\n\tTimeStamp: {}", p.path, p.date,
                    match p.timestamp {
                        Some(i) => i.to_string(),
                        None => "No TimeStamp".to_string(),
                    }
                );
            }
        }
    }
}

fn get_pictures() -> Vec<Picture> { //holt alle Bilder und wandelt Sie in .png um
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

fn sort_pictures(pictures: &mut Vec<Picture>) {
    for picture in pictures.iter_mut() {
        //-----------
        println!("{:?}", picture.path);

        //-----------

        let file = File::open(str::replace(picture.path.as_str(), "JPEG", "jpg")).expect("Unable to open File for Sorting");
        let exif = Reader::new().read_from_container(&mut BufReader::new(&file)).expect("Unable to start Reader");

        let tag = Tag::DateTime;
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            picture.date = field.display_value().with_unit(&exif).to_string();
            let mut tmp  = picture.date.replace("-", "");
            tmp = tmp.replace(":", "");
            tmp = tmp.replace(" ", "");
            
            picture.timestamp = match tmp.trim().parse::<i64>() {
                    Ok(i) => Some(i),
                    Err(_) => None,
            }
        }
    }

    pictures.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

}

fn rename_pictures(pictures: &mut Vec<Picture>) {
    println!("<NAME>-index with NAME: ");
    let mut call: String = String::new();
    io::stdin()
        .read_line(&mut call)
        .expect("Unable to read line");
    call = call.trim().replace(" ", "");

    let mut ending: String = String::from("");
    let tmp = &pictures[0].path.chars();
    for c in tmp.clone() {
        if c == '.' { ending.clear(); }
        ending.push(c);
    }


    let mut counter: u16   = 1;
    for picture in pictures {
        let new_path = format!("{}-{}{}", call, counter, ending);
        fs::rename(&picture.path, &new_path).expect("Unable to rename Files");
        picture.path = new_path;
        counter += 1;
    }
}

fn main() {
    let mut v :Vec<Picture> = get_pictures();
    sort_pictures(&mut v);
    rename_pictures(&mut v);
    Picture::printer(v);
}