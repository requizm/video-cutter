mod input;

use input::{parse_time, Clip, Clips};

use crate::input::Input;

use std::{fmt::format, process::Command};

fn main() {
    let input: Input = Input::new();

    for video in input.videos.iter() {
        ffmpeg(
            video.clone(),
            input.output_path.clone(),
            input.resolution.clone(),
            input.fps,
            input.video_format.clone(),
            input.preset.clone(),
        );
    }
}

fn ffmpeg(
    video: Clips,
    output_folder: String,
    resolution: Option<String>,
    fps: Option<usize>,
    video_format: Option<String>,
    preset: Option<String>,
) {
    // Get video extension
    let extension = video_format.unwrap_or_else(|| {
        let binding = video.video_path.clone();
        let binding = binding.split('.').collect::<Vec<&str>>();
        binding.last().unwrap().to_string()
    });

    let output_paths: Vec<(Clip, String)> = video
        .clips
        .iter()
        .map(|clip| {
            (
                clip.clone(),
                format!("{}/{}.{}", output_folder, clip.name, extension),
            )
        })
        .collect();

    let not_exists_clips: Vec<(Clip, String)> = output_paths
        .iter()
        .filter(|(_, path)| {
            !std::path::Path::new(&path).exists() && !std::path::Path::new(&path).exists()
        })
        .map(|(clip, path)| (clip.clone(), path.clone()))
        .collect();

    if not_exists_clips.len() > 0 {
        for (clip, path) in not_exists_clips.clone() {
            // Without reencoding
            {
                let mut command = Command::new("ffmpeg");
                let mut args = command
                    .arg("-ss")
                    .arg(clip.start)
                    .arg("-to")
                    .arg(clip.end)
                    .arg("-i")
                    .arg(video.video_path.clone())
                    .arg("-c")
                    .arg("copy")
                    .arg(format!("{}/{}_full.{}", output_folder, clip.name, extension));
                if let Some(ref resolution) = resolution {
                    args = args.arg("-s").arg(resolution);
                }

                if let Some(fps) = fps {
                    args = args.arg("-filter:v").arg(format!("fps={}", fps));
                }

                if let Some(ref preset) = preset {
                    args = args.arg("-preset").arg(preset);
                }

                let mut res = args.spawn().expect("Failed to start ffmpeg");
                res.wait().expect("Failed to wait for ffmpeg");
            }

            // Optimize previous output
            {
                let mut command = Command::new("ffmpeg");
                let mut args = command
                    .arg("-i")
                    .arg(format!("{}/{}_full.{}", output_folder, clip.name, extension))
                    .arg("-c:v")
                    .arg("libx264")
                    .arg("-c:a")
                    .arg("aac")
                    .arg("-b:a")
                    .arg("128k")
                    .arg("-b:v")
                    .arg("5M")
                    .arg(format!("{}/{}.{}", output_folder, clip.name, extension));
                if let Some(ref resolution) = resolution {
                    args = args.arg("-s").arg(resolution);
                }

                if let Some(fps) = fps {
                    args = args.arg("-filter:v").arg(format!("fps={}", fps));
                }

                if let Some(ref preset) = preset {
                    args = args.arg("-preset").arg(preset);
                }

                let mut res = args.spawn().expect("Failed to start ffmpeg");
                res.wait().expect("Failed to wait for ffmpeg");
            }

            // Remove previous output
            {
                let opt_output = format!("{}/{}_full.{}", output_folder, clip.name, extension);
                let path = std::path::Path::new(&opt_output);
                if path.exists() {
                    let _ = std::fs::remove_file(path);
                }
            }
        }
    }
}
