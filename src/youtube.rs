use std::{error::Error, fmt::{Debug, Display} };

use reqwest::blocking;
use serde::Deserialize;
use serde_json::json;
use url::Url;

#[derive(Debug, Deserialize)]
struct PlayerResponse {
    #[serde(rename="streamingData")]
    streaming_data: StreamingData,
    #[serde(rename="videoDetails")]
    video_details: VideoDetail,
}

#[derive(Debug, Deserialize)]
struct VideoDetail {
    title: String
}

#[derive(Debug, Deserialize)]
struct StreamingData {
    formats: Vec<VideoFormat>
}

#[derive(Debug, Deserialize)]
struct VideoFormat {
    url: String,
    height: u32,
    width: u32,
    // #[serde(default)]
    #[serde(rename="contentLength")]
    content_length: Option<String>,
    #[serde(rename="approxDurationMs")]
    approx_duration_ms: String
}

pub struct YoutubeVideo {
    title: String,
    formats: Vec<VideoFormat>
}

impl YoutubeVideo {
    pub fn title(&self) -> &str{
        &self.title
    }

    pub fn formats_count(&self) -> usize {
        self.formats.len()
    }

    pub fn format_url(&self, index: usize) -> Option<&str> {
        if index < self.formats.len() {
            Some(&self.formats[index].url)
        } else {
            None
        }
    }
}

impl Display for YoutubeVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "title: {}\n", self.title)?;
        write!(f, "\nvideo formats list:\n")?;
        let mut index = 1;
        for video in &self.formats {
            write!(f, "{index}) {}x{}, len:{:?}, duration: {}\n", 
                video.width, 
                video.height, 
                video.content_length, 
                video.approx_duration_ms
            )?;
            index = index + 1;
        }
        Ok(())
    }
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

    /// request video information
    pub fn fetch_url(&self, video_id: &str) -> Result<YoutubeVideo, Box<dyn Error>> {
        let query = json!(
            {
                "videoId": video_id, 
                "context":
                {
                    "client":
                    {
                        "clientName": Self::POST.client_name,
                        "clientVersion": Self::POST.client_version
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

        // println!("\n{}\n", response_text);

        match serde_json::from_str::<PlayerResponse>(response_text.as_str()) {
            Ok(player_response) => {
                Ok(
                    YoutubeVideo {
                        formats: player_response.streaming_data.formats,
                        title: player_response.video_details.title
                    }
                )
            },
            Err(e) => {
                println!("\nparse response txt failed\n\n{}", response_text);
                Err(e.into())
            },
        }
    }
}

/// fetch video id from url
pub fn fetch_id_from_url(url: &str) -> Result<String, Box<dyn Error>> {
    let u = Url::parse(url)?;
    let host = u.host_str().unwrap_or("");
    let path = u.path();
    let query = u.query_pairs();

    if !(host.contains("youtube.com") || host.contains("youtu.be")) {
        return Err("not youtube url".into());
    }

    let mut has_si_param = false;
    for (key, val) in query {
        if key == "v" {
            return Ok(val.to_string());
        } else if key == "si" {
           has_si_param = true;
        }
    }

    if has_si_param {
        if let Some(id) = path.find("/") {
            return Ok(path[id + 1 ..].to_string());
        }
    }
    
    Err("no video id found".into())
}

/// youtube rquest paramemters
/// 
/// 2024.3.10 replace android with web. because android didn't work. change version, also work!!!
/// 
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

#[cfg(test)]
mod tests {
    #[test]
    /// ```
    /// https://www.youtube.com/watch?v=sgNS56c1K30 377MB
    /// https://youtu.be/sgNS56c1K30?si=PQhW2mDPsaAvvYuj 377MB
    /// https://youtu.be/duIfgZkp9Lc?si=4_k4A6BSbt1-JjpU 20MB
    /// ```
    fn fetch_id_from_url_test() {
        let id = super::fetch_id_from_url("https://www.youtube.com/watch?v=sgNS56c1K30").unwrap();
        assert_eq!(id.as_str(), "sgNS56c1K30");
        let id = super::fetch_id_from_url("https://youtu.be/sgNS56c1K30?si=PQhW2mDPsaAvvYuj").unwrap();
        assert!(id.as_str() == "sgNS56c1K30");
        let id = super::fetch_id_from_url("https://youtu.be?si=PQhW2mDPsaAvvYuj").unwrap();
        assert!(id.as_str() == "");
    }
}