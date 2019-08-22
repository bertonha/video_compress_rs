use glob::glob_with;
use glob::MatchOptions;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

const OUTPUT_SUFFIX: &str = "_compressed.mp4";

#[derive(Debug, StructOpt)]
#[structopt(name = "video_compress", about = "Compress videos recursively.")]
struct Opt {
    #[structopt(long = "delete")]
    delete: bool,

    input: String,

    #[structopt(long = "format", default_value = ".mp4")]
    format: String,
}

fn call_ffmpeg(in_filename: PathBuf) {
    let mut out_filename = in_filename.clone();
    out_filename.set_extension(OUTPUT_SUFFIX);
    Command::new("ffmpeg")
        .args(&[
            "-i",
            &in_filename.to_str().unwrap(),
            "-c:v",
            "libx265",
            "-crf",
            "28",
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            &out_filename.to_str().unwrap(),
        ])
        .spawn()
        .expect("failed to execute process");
}

fn main() {
    let args = Opt::from_args();

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let path_to_search = format!(
        "{initial_path}/**/*{format}",
        initial_path = args.input,
        format = args.format
    );

    for entry in glob_with(&path_to_search, options).unwrap() {
        if let Ok(path) = entry {
            call_ffmpeg(path);
            if args.delete {
                // remove file
            }
        }
    }
}
