use std::process;

use yaydl_lib::{ 
    youtube::{ Youtube, fetch_id_from_url }, 
    read_input_index,
    open_url
};
use argh::FromArgs;

/// youtube dl command
#[derive(FromArgs)]
struct CmdParam {
    /// youtube url
    // #[argh(option, short='u')]
    // url: Option<String>,
    /// youture ids
    #[argh(positional, greedy)]
    ids: Vec<String>,
}

fn main() {
    let args: CmdParam = argh::from_env();
    let mut video_id: String = "".into();
    if args.ids.len() == 0 || args.ids[0].contains("youtu") {
        match fetch_id_from_url(args.ids[0].as_str()) {
            Ok(id) => {
                video_id = id;
            },
            Err(e) => {
                println!("fetch id from url failed: {:?}", e);
            },
        }
    } else {
        video_id = args.ids[0].clone();
    }

    println!("input video_id: ({})", video_id);

    if video_id.len() == 0 {
        println!("no video id found, please check input");
        process::exit(1);
    }

    let dl = Youtube::new();

    let youtube_video = match dl.fetch_url(video_id.as_str()) {
        Ok(v) => v,
        Err(e) => {
            println!("fetch url error\n{:?}", e);
            process::exit(1);
        },
    };
    
    println!("{youtube_video}");

    println!("please input the index, which format you want:\n");
    match read_input_index() {
        Ok(index) => {
            if let Some(url) = youtube_video.format_url(index - 1) {
                println!("{url}");
                open_url(url);
            } else {
                println!("{} not in [1-{}]", index, youtube_video.formats_count());
            }
        },
        Err(e) => {
            println!("parse input index error.({})", e);
        },
    }
}
