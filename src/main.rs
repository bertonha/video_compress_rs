use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

const OUTPUT_TRANSCODE_EXTENSION: &str = "compressed.mp4";
const OUTPUT_THUMBNAIL_EXTENSION: &str = "thumbnail.jpg";

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
    #[arg(long = "thumbnail")]
    thumbnail: bool,
}

fn thanscode_video(in_filename: &Path, out_filename: &Path, use_gpu: bool) {
    let mut ffmpeg = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-i")
        .arg(in_filename)
        .arg("-c:v")
        .arg(if use_gpu { "h264_nvenc" } else { "libx264" })
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

fn create_thumbnail(in_filename: &Path, out_filename: &Path) {
    let mut ffmpeg = Command::new("ffmpeg")
        .arg("-i")
        .arg(in_filename)
        .arg("-vf")
        .arg("thumbnail,scale=1280:720")
        .arg("-frames:v")
        .arg("1")
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
        .filter(|e| {
            e.path()
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase()
                == args.extension
        })
        .filter(|e| {
            !e.file_name()
                .to_string_lossy()
                .ends_with(OUTPUT_TRANSCODE_EXTENSION)
        });

    for entry in walker {
        let in_filename = entry.path();

        if args.thumbnail {
            let mut out_thumbnail_filename = PathBuf::from(in_filename);
            out_thumbnail_filename.set_extension(OUTPUT_THUMBNAIL_EXTENSION);

            if !out_thumbnail_filename.exists() {
                println!("Creating thumbnail: {}", in_filename.display());
                create_thumbnail(in_filename, out_thumbnail_filename.as_path());
            }
        }

        let mut out_transcoded_filename = PathBuf::from(in_filename);
        out_transcoded_filename.set_extension(OUTPUT_TRANSCODE_EXTENSION);

        if out_transcoded_filename.exists() {
            println!("Skipping: {}", in_filename.display());
            continue;
        }

        println!("Processing: {}", in_filename.display());
        thanscode_video(in_filename, out_transcoded_filename.as_path(), args.use_gpu);

        if args.delete {
            fs::remove_file(in_filename).expect("failed to delete file");
        }
    }
}
