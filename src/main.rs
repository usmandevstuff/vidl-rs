use std::process::Command;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "vidl",
    version = "0.1",
    about = "a cli app to download videos from websites supported by yt-dlp"
)]
struct Cli {
    #[arg(help = "URL of the video to download")]
    url: String,

    #[arg(
        short,
        long,
        help = "path to save the downloaded video",
        default_value = "current"
    )]
    output: String,

    #[arg(short, long, help="audio only[ignores video download]", action = clap::ArgAction::SetTrue)]
    audio_only: bool,

    #[arg(
        short,
        long,
        help = "list available formats",
    )]
    list_formats: bool,

    #[arg(
        short,
        long,
        help = "[mp4: default, webm: for higher bitrate]",
        default_value = "mp4"
    )]
    ext: String,

    #[arg(
        short,
        long,
        help = "video quality[eg: 1080, 720, 480, 360, 240, 144]",
        default_value = "best"
    )]
    quality: String,
}

fn main() {
    let cli = Cli::parse();
    let url = cli.url;
    let output = cli.output;
    let audio_only = cli.audio_only;
    let list_formats = cli.list_formats;
    let ext = cli.ext;
    let quality = cli.quality;

    let mut cmd = Command::new("yt-dlp");
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    if url != "none" {
        cmd.arg(url);
    } else {
        println!("URL is required");
        return;
    }

    if audio_only {
        cmd.arg("-f").arg("bestaudio").arg("-x");
    } else if list_formats {
        cmd.arg("-F");
    } else if ext == "mp4" {
        let format = match quality.as_str() {
            "1080" => "bestvideo[height<=1080][ext=mp4]+bestaudio",
            "720" => "bestvideo[height<=720][ext=mp4]+bestaudio",
            "480" => "bestvideo[height<=480][ext=mp4]+bestaudio",
            "360" => "bestvideo[height<=360][ext=mp4]+bestaudio",
            "240" => "bestvideo[height<=240][ext=mp4]+bestaudio",
            "144" => "bestvideo[height<=144][ext=mp4]+bestaudio",
            _ => "bestvideo[ext=mp4]+bestaudio",
        };
        cmd.arg("-f").arg(format);
    } else if ext == "webm" {
        let format = match quality.as_str() {
            "1080" => "bestvideo[height<=1080][ext=webm]+bestaudio",
            "720" => "bestvideo[height<=720][ext=webm]+bestaudio",
            "480" => "bestvideo[height<=480][ext=webm]+bestaudio",
            "360" => "bestvideo[height<=360][ext=webm]+bestaudio",
            "240" => "bestvideo[height<=240][ext=webm]+bestaudio",
            "144" => "bestvideo[height<=144][ext=webm]+bestaudio",
            _ => "bestvideo[ext=webm]+bestaudio",
        };
        cmd.arg("-f").arg(format);
    } else {
        println!("Invalid extension");
        return;
    }

    if output != "current" {
        cmd.arg("-o").arg(output);
    } else {
        cmd.arg("-o").arg("%(title)s.%(ext)s");
    }

    println!("executing yt-dlp command: {:?}", cmd);

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("executed yt-dlp successfully");
            } else {
                println!("yt-dlp failed with status: {}", status);
            }
        }
        Err(e) => {
            eprintln!("Error executing yt-dlp: {}", e);
        }
    }
}
