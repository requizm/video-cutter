mod input;

use input::Clip;

use crate::input::Input;

use std::process::Command;

fn main() {
    let input: Input = Input::new();

    for video in input.videos.iter() {
        for clip in video.clips.iter() {
            ffmpeg(
                clip.clone(),
                video.video_path.clone(),
                input.output_folder.clone(),
                input.resolution.clone(),
                input.fps,
                input.video_format.clone(),
                input.preset.clone(),
            );
        }
    }
}

fn ffmpeg(
    clip: Clip,
    video_path: String,
    output_folder: String,
    resolution: Option<String>,
    fps: Option<usize>,
    video_format: Option<String>,
    preset: Option<String>,
) {
    // Get video extension
    let extension = video_format.unwrap_or_else(|| {
        let binding = video_path.clone();
        let binding = binding.split('.').collect::<Vec<&str>>();
        binding.last().unwrap().to_string()
    });

    // Check if video already exists
    let output_path = format!("{}/{}.{}", output_folder, clip.name, extension);
    if std::path::Path::new(&output_path).exists() {
        println!("Video {} already exists", output_path);
        return;
    }

    let mut command = Command::new("ffmpeg");
    let mut args = command
        .arg("-i")
        .arg(video_path)
        .arg("-ss")
        .arg(clip.start)
        .arg("-to")
        .arg(clip.end);
        // .arg("-c:a")
        // .arg("copy")
        // .arg("-c:v")
        // .arg("libx264")

    if let Some(resolution) = resolution {
        args = args.arg("-s").arg(resolution);
    }

    if let Some(fps) = fps {
        args = args.arg("-filter:v").arg(format!("fps={}", fps));
    }

    if let Some(preset) = preset {
        args = args.arg("-preset").arg(preset);
    }

    args = args.arg(output_path);

    println!("Running: {:?}", args);

    let mut res = args.spawn().expect("Failed to start ffmpeg");
    res.wait().expect("Failed to wait for ffmpeg");
}
