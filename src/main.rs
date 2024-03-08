use std::process;

use yaydl::youtube::Youtube;
use argh::FromArgs;

/// youtube dl command
#[derive(FromArgs)]
struct CmdParam {
    /// youture ids
    #[argh(positional, greedy)]
    ids: Vec<String>,
}

fn main() {
    let args: CmdParam = argh::from_env();
    if args.ids.len() == 0 {
        println!("please input youtube vido ids");
        process::exit(1);
    }

    let dl = Youtube::new();

    let videos = match dl.fetch_url(args.ids[0].as_str()) {
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
