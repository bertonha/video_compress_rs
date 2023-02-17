use std::fs;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

const OUTPUT_EXTENSION: &str = "compressed.mp4";

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    input: PathBuf,
    #[arg(long = "extension", default_value = "mp4")]
    extension: String,
    #[arg(long = "delete")]
    delete: bool,
    #[arg(long = "use-gpu")]
    use_gpu: bool,
}

fn call_ffmpeg(in_filename: &Path, out_filename: &Path, use_gpu: bool) {
    let mut ffmpeg = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-i")
        .arg(in_filename)
        .arg("-c:v")
        .arg(if use_gpu {"h264_nvenc"} else {"libx264"})
        .arg("-preset")
        .arg("slow")
        .arg("-crf")
        .arg("24")
        .arg("-profile:v")
        .arg("high")
        .arg("-c:a")
        .arg("aac")
        .arg("-b:a")
        .arg("128k")
        .arg(out_filename)
        .spawn()
        .expect("failed to execute ffmpeg");

    ffmpeg.wait().expect("failed to wait on ffmpeg");
}

fn main() {
    let args = Args::parse();

    let walker = WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension() == Some(args.extension.as_ref()))
        .filter(|e| !e.file_name().to_string_lossy().ends_with(OUTPUT_EXTENSION));

    for entry in walker {
        let in_filename = entry.path();
        let mut out_filename = PathBuf::from(in_filename);
        out_filename.set_extension(OUTPUT_EXTENSION);

        if out_filename.exists() {
            println!("Skipping: {}", in_filename.display());
            continue;
        }

        println!("Processing: {}", in_filename.display());
        call_ffmpeg(in_filename, out_filename.as_path(), args.use_gpu);

        if args.delete {
            fs::remove_file(in_filename).expect("failed to delete file");
        }
    }
}
