use std::{error::Error, fmt::Debug};

use reqwest::blocking;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct PlayerResponse {
    #[serde(rename="streamingData")]
    streaming_data: StreamingData
}

#[derive(Debug, Deserialize)]
struct StreamingData {
    formats: Vec<VideoFormat>
}

#[derive(Debug, Deserialize)]
pub struct VideoFormat {
    pub url: String,
    pub height: u32,
    pub width: u32,
    // #[serde(default)]
    #[serde(rename="contentLength")]
    pub content_length: Option<String>,
    #[serde(rename="approxDurationMs")]
    pub approx_duration_ms: String
}

pub struct Youtube {
    http_client: blocking::Client
}

impl Youtube {
    const POST: &'static Post = &ANDROID_POST;
    pub fn new() -> Self {
        let http_client = blocking::Client::new();

        Self{
            http_client
        }
    }

    /// test id TgoYoc8oBFw
    /// https://www.youtube.com/watch?v=vzf_VGhZPI4
    pub fn fetch_url(&self, video_id: &str) -> Result<Vec<VideoFormat>, Box<dyn Error>> {
        let query = json!(
            {
                "videoId": video_id, 
                "context":
                {
                    "client":
                    {
                        "clientName":Self::POST.client_name,
                        "clientVersion":Self::POST.client_version
                    }
                },
                "params":"CgIQBg=="
            }
        );
        let query_str = query.to_string();
        let request = self.http_client.post(Self::POST.url).body(query_str);
        let response = match request.send() {
            Ok(r) => r,
            Err(e) => {
                return Err(e.into());
            }
        };

        let response_text = match response.text() {
            Ok(t) => t,
            Err(e) => {
                return Err(e.into());
            },
        };

        match serde_json::from_str::<PlayerResponse>(response_text.as_str()) {
            Ok(player_response) => Ok(player_response.streaming_data.formats),
            Err(e) => {
                println!("\nparse response txt failed\n\n{}", response_text);
                Err(e.into())
            },
        }
    }
}

/// 2024.3.10 replace android with web. because android didn't work. change version, also work!!!
/// 2024.3.10
struct Post {
    url: &'static str,
    /// youtube client name: android, web
    client_name: &'static str,
    /// youtube client version
    client_version: &'static str
}

#[allow(dead_code)]
const ANDROID_POST: Post = Post {
    url: "https://www.youtube.com/youtubei/v1/player?key=AIzaSyA8eiZmM1FaDVjRy-df2KTyQ_vz_yYM39w",
    client_name: "ANDROID",
    // 2024.3.10 change 17.31.35 to 19.09.37, because old version didn't work.
    client_version: "19.09.37"
};

#[allow(dead_code)]
const WEB_POST: Post = Post {
    url: "https://www.youtube.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8",
    client_name: "WEB",
    client_version: "2.20220801.00.00"
};