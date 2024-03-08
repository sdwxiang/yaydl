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
    const URL: &'static str = "https://www.youtube.com/youtubei/v1/player?key=AIzaSyA8eiZmM1FaDVjRy-df2KTyQ_vz_yYM39w";

    pub fn new() -> Self {
        let http_client = blocking::Client::new();

        Self{
            http_client
        }
    }

    /// test id TgoYoc8oBFw
    pub fn fetch_url(&self, video_id: &str) -> Result<Vec<VideoFormat>, Box<dyn Error>> {
        let query = json!(
            {
                "videoId": video_id, 
                "context":
                {
                    "client":
                    {
                        "clientName":"ANDROID",
                        "clientVersion":"17.31.35"
                    }
                },
                "params":"CgIQBg=="
            }
        );
        let query_str = query.to_string();
        let request = self.http_client.post(Self::URL).body(query_str);
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
        // println!("{video_id}, {:?}", r);
    }
}