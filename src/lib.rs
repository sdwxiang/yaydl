use core::time;
use std::{error::Error, io::stdin, process::Command, thread};

pub mod youtube;

/// read user input usize number
pub fn read_input_index() -> Result<usize, Box<dyn Error>> {
    let mut select_index_input = String::new();
    stdin().read_line(&mut select_index_input)?;

    let select_index = select_index_input.trim_end();

    Ok(select_index.parse::<usize>()?)
}

/// oepn url in browser
pub fn open_url(url: &str) -> bool {
    let url_str = url.replace("&", "^&");
    if let Ok(mut child) = Command::new("cmd.exe")
        .arg("/C").arg("start").arg("")
        .arg(url_str).spawn() {
        // On windows need to allow time for browser to start
        thread::sleep(time::Duration::new(3, 0));
        
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}