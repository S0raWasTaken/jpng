#![warn(clippy::pedantic)]

use std::{
    env::current_dir, error::Error, path::PathBuf
};

use clap::{crate_version, App, Arg, Command};
use convert::dir;
use rayon::yield_now;

type Res<T> = std::result::Result<T, Box<dyn Error>>;

mod convert;

fn main() -> Res<()> {
    let args = cli().get_matches();
    let folder = args.get_one::<String>("img_or_folder");

    let path = if let Some(folder) = folder {
        PathBuf::from(folder)
    } else {
        current_dir()?
    };

    path.try_exists()?;
    dir(path)?;

    println!("Done!");
    loop {yield_now();}
}
fn cli() -> App<'static> {
    Command::new("JPng")
        .about("Converte Imagens pra JPG")
        .author("S0ra, s0ra@duck.com, https://github.com/S0raWasTaken")
        .version(crate_version!())
        .arg(
            Arg::new("img_or_folder")
                .index(1)
                .help("o caminho para a imagem ou pasta")
        )
}
