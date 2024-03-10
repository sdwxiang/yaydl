use std::process;

use yaydl::youtube::{ Youtube, fetch_id_from_url };
use argh::FromArgs;

/// youtube dl command
#[derive(FromArgs)]
struct CmdParam {
    /// youtube url
    #[argh(option, short='u')]
    url: Option<String>,
    /// youture ids
    #[argh(positional, greedy)]
    ids: Vec<String>,
}

fn main() {
    let args: CmdParam = argh::from_env();
    let mut video_id: String = "".into();
    if args.ids.len() == 0 {
        if let Some(url) = args.url {
            match fetch_id_from_url(url.as_str()) {
                Ok(id) => {
                    video_id = id;
                },
                Err(e) => {
                    println!("fetch id from url failed: {:?}", e);
                },
            }
        }
    } else {
        video_id = args.ids[0].clone();
    }

    if video_id.len() == 0 {
        println!("no video id found, please check input");
        process::exit(1);
    }

    let dl = Youtube::new();

    let videos = match dl.fetch_url(video_id.as_str()) {
        Ok(v) => v,
        Err(e) => {
            println!("fetch url error\n{:?}", e);
            process::exit(1);
        },
    };
    
    let mut index = 1;
    for video in videos {
        println!("{index}) {}x{}, len:{:?}, duration: {}\n{}", 
        video.width, 
        video.height, 
        video.content_length, 
        video.approx_duration_ms,
        video.url
        );
        index = index + 1;
    }
}
