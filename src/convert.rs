use image::ImageReader;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::{
    ffi::OsStr, fs::create_dir_all, path::PathBuf, sync::atomic::{AtomicUsize, Ordering::Relaxed}
};
use walkdir::{DirEntry, WalkDir};

use crate::Res;

pub fn dir(path: PathBuf) -> Res<()> {
    let output_dir = path.join("output");
    create_dir_all(&output_dir)?;

    let existing_files = WalkDir::new(&output_dir).into_iter().filter_map(Result::ok).filter_map(|e| {
        let name = ends_with(e.file_name(), ".jpg");
        if name.0 {
            Some(name.1.replace(".jpg", ".png"))
        } else {
            None
        }
    }).collect::<Vec<_>>();

    let files = WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|file| {
            let name = ends_with(file.file_name(), "png");
            name.0 && !existing_files.contains(&name.1)
        }).collect::<Vec<_>>();
    
    let total = files.len();

    let count = AtomicUsize::new(0);
    let errors = AtomicUsize::new(0);

    files.par_iter().for_each(|file| {
        print!("\rConverting: {}/{total}, Errors: {}", count.load(Relaxed), errors.load(Relaxed));
        if convert_to_jpg(file, output_dir.to_str().unwrap()).is_err() {
            errors.fetch_add(1, Relaxed);
            count.fetch_add(1, Relaxed);
        } else {
            count.fetch_add(1, Relaxed);
        }
    });

    println!("\rConverting: {}/{total}, Errors: {}", count.load(Relaxed), errors.load(Relaxed));
    Ok(())
}

#[inline]
fn ends_with(file_name: &OsStr, with: &str) -> (bool, String) {
    let out = file_name.to_str().unwrap().to_string();
    (out.ends_with(with), out)
}

fn convert_to_jpg(file: &DirEntry, output_dir: &str) -> Res<()> {
    let new_name = file
        .file_name()
        .to_str()
        .unwrap()
        .replace(".png", ".jpg")
        .replace(".PNG", ".jpg");
    let image = ImageReader::open(file.path())?.decode()?;
    image.save_with_format(PathBuf::from(format!("{output_dir}\\{new_name}")), image::ImageFormat::Jpeg)?;

    Ok(())
}
