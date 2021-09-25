extern crate exif;
extern crate pbr;

use core::panic;
use std::{fs::{{self}, File}};
use std::io::{{self}, BufReader};
use exif::{In, Reader, Tag};
use pbr::ProgressBar;
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
        //print!("Converting {} ", path);
        let img = ImageReader::open(path)?.decode()?;
        
        for c in path.chars().rev() {
            if c == '.' { break; }
            new_path.next_back();
        };
        let new_path = format!("{}JPEG", new_path.as_str());
        img.save(new_path.as_str()).expect("Couldn't save the Image");
        Ok(new_path.as_str().to_string())
    }

    fn new(input: Option<&str>) -> Picture {
        let i;
        i = match input {
            Some(i) => i,
            None => panic!("found Invalid Name"),
        };

        Picture { path: i.to_string(), date: "Date".to_string(),
                is_picture: i.contains(".jpg"), timestamp: None}
    }

    fn printer(v: Vec<Self>) {
        println!("The EXIF-Data of the Pictures told me:");
        for p in v {
            if p.is_picture {
                println!("Path: {}\n\tDate: {}\n\t\tTimeStamp: {}", p.path, p.date,
                    match p.timestamp {
                        Some(i) => i.to_string(),
                        None => "No TimeStamp".to_string(),
                    }
                );
            }
        }
    }
}

fn get_pictures() -> Vec<Picture> { //holt alle Bilder und wandelt Sie in .JPEG um
    let mut files: Vec<Picture> = Vec::new();
    let paths = fs::read_dir("./").expect("Couldn't read Paths");

    println!("I can see the following Files: ");
    for path in paths {
        println!("Name: {}", path.as_ref().unwrap().path().display());
        files.push(
            match path {
                Ok(pic) => Picture::new(pic.file_name().to_str()),
                Err(_) => panic!("Found invalid path"),
            }
        );
    }

    println!("Converting *.jpg -> *.JPEG ... start");
    let mut pictures: Vec<Picture> = Vec::new();
    let mut pb = ProgressBar::new(files.len() as u64);
    pb.format("╢▌▌░╟");

    for mut p in files {
        pb.inc();
        if p.is_picture {
                let i = match Picture::convert(&p.path) {
                    Ok(i) => i,
                    Err(_) => panic!("Unable to convert")
                };
            p.path = i;

            pictures.push(p);
        }
    }

    pb.finish_println("Converting ... finished\n");
    pictures
}

fn sort_pictures(pictures: &mut Vec<Picture>) {
    for picture in pictures.iter_mut() {

        let file = File::open(str::replace(picture.path
                        .as_str(), "JPEG", "jpg"))
                        .expect("Unable to open File for Sorting");
        let exif = Reader::new()
                        .read_from_container(&mut BufReader::new(&file))
                        .expect("Unable to start Reader");

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

    print!("Sorting ... ");
    pictures.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    println!("finished");
}

fn rename_pictures(pictures: &mut Vec<Picture>) {
    println!("<NAME>_index with NAME: ");
    let mut call: String = String::new();
    io::stdin()
        .read_line(&mut call)
        .expect("Unable to read line");
    call = call.trim().replace(" ", "");

    let mut ending: String = String::new();
    let tmp = &pictures[0].path.chars();
    for c in tmp.clone() {
        if c == '.' { ending.clear(); }
        ending.push(c);
    }

    let mut counter: u32 = 10000;
    let mut pb = ProgressBar::new(pictures.len() as u64);
    pb.format("╢▌▌░╟");
    println!("Renaming ... start");
    for picture in pictures {
        counter += 1;
        let c = counter.to_string();
        let mut c = c.chars();
        c.next();
        let c = c.as_str();

        fs::create_dir_all(format!("./{}", call)).expect("Unable to create Folder");
        let new_path = format!("{}/{}_{}{}", call, call, c, ending);
        
        fs::rename(&picture.path
                .replace(".JPEG", ".jpg"), 
            &new_path
                .replace(".JPEG", ".jpg"))
            .expect("Unable to rename Files");
        picture.path = new_path;
        pb.inc();
    }
    pb.finish_println("Renaming ... finished\n");
}

fn main() {
    println!("Hi, I order your .jpg-Files :)");
    let mut v :Vec<Picture> = get_pictures();
    sort_pictures(&mut v);
    rename_pictures(&mut v);
    //Picture::printer(v);
}