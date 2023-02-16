use clap::Parser;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

const OUTPUT_EXTENSION: &str = "compressed.mp4";

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "delete")]
    delete: bool,

    #[arg(short = 'i', long = "input")]
    input: PathBuf,

    #[arg(long = "extension", default_value = "mp4")]
    extension: String,
}

fn call_ffmpeg(in_filename: PathBuf) {
    let mut out_filename = in_filename.clone();
    out_filename.set_extension(OUTPUT_EXTENSION);
    Command::new("ffmpeg")
        .args([
            "-i",
            in_filename.to_str().unwrap(),
            "-c:v",
            "libx265",
            "-crf",
            "28",
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            out_filename.to_str().unwrap(),
        ])
        .spawn()
        .expect("failed to execute process");
}

fn main() {
    let args = Args::parse();

    let walker = WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension() == Some(args.extension.as_ref()));

    for entry in walker {
        println!("Processing: {}", entry.path().display());
        call_ffmpeg(entry.into_path());
    }
}
