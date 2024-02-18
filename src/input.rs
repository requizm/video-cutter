use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Clip {
    pub name: String,
    pub start: String, // HH:MM:SS
    pub end: String,   // HH:MM:SS
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Clips {
    pub video_path: String,
    pub clips: Vec<Clip>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Input {
    pub videos: Vec<Clips>,
    pub output_folder: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>, // widthxheight

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>, // mp4, mkv, avi, etc

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>, // ultrafast, superfast, veryfast, faster, fast, medium, slow, slower, veryslow
}

impl Input {
    pub fn new() -> Self {
        let input_str: String =
            std::fs::read_to_string("input.json").expect("input.json not found");
        let input: Input = serde_json::from_str(&input_str).expect("Error parsing input.json");

        // Check if all times are valid (HH:MM:SS)
        for video in input.videos.iter() {
            for clip in video.clips.iter() {
                if clip.start.len() != 8 || clip.end.len() != 8 {
                    panic!("Invalid time length in input.json");
                }
                let hours: usize = clip.start[..2].parse().unwrap();
                let minutes: usize = clip.start[3..5].parse().unwrap();
                let seconds: usize = clip.start[6..8].parse().unwrap();
                if hours > 23 || minutes > 59 || seconds > 59 {
                    panic!("Invalid time in input.json");
                }
            }
        }

        // Check if all video paths exist
        for video in input.videos.iter() {
            std::fs::metadata(&video.video_path).expect("Video path not found");
        }

        // Check if output folder exists
        std::fs::metadata(&input.output_folder).expect("Output folder not found");

        // Check if resolution is valid
        if let Some(resolution) = &input.resolution {
            let resolution: Vec<&str> = resolution.split('x').collect();
            if resolution.len() != 2 {
                panic!("Invalid resolution in input.json");
            }
            let _: usize = resolution[0].parse().unwrap();
            let _: usize = resolution[1].parse().unwrap();
        }

        // Check if preset is valid
        if let Some(preset) = &input.preset {
            if preset != "ultrafast"
                && preset != "superfast"
                && preset != "veryfast"
                && preset != "faster"
                && preset != "fast"
                && preset != "medium"
                && preset != "slow"
                && preset != "slower"
                && preset != "veryslow"
            {
                panic!("Invalid preset in input.json");
            }
        }

        return input;
    }
}
