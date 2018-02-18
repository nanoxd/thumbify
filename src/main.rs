extern crate image;
#[macro_use]
extern crate quicli;
use quicli::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::File;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
    #[structopt(default_value = "*.jpg")]
    pattern: String,
    #[structopt(long = "max-size", short = "s", default_value = "300")]
    size: u32,
    #[structopt(long = "output", short = "o", default_value = "thumbnails")]
    thumb_dir: String,
}

fn make_thumbnail(original: &Path, thumb_dir: &str, longest_edge: u32) -> Result<()> {
    let img = image::open(&original)?;
    let thumbnail = img.resize(longest_edge, longest_edge, image::FilterType::Nearest);

    let thumb_name = original
        .file_name()
        .ok_or_else(|| format_err!("Could not read file name of {:?}", original))?;

    let thumb_path = PathBuf::from(thumb_dir)
        .join(thumb_name)
        .with_extension("jpg");

    let mut output_file = File::create(thumb_path)?;
    thumbnail.save(&mut output_file, image::JPEG)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
    main!(|args: Cli, log_level: verbosity| {
        let files = glob(&args.pattern)?;
        create_dir(&args.thumb_dir)?;
        info!(
            "Saving {} thumbnails into {:?}...",
            files.len(),
            args.thumb_dir
        );
    });
}
