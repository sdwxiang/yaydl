use std::{error::Error, io::stdin};

pub mod youtube;

/// read user input usize number
pub fn read_input_index() -> Result<usize, Box<dyn Error>> {
    let mut select_index_input = String::new();
    stdin().read_line(&mut select_index_input)?;

    let select_index = select_index_input.trim_end();

    Ok(select_index.parse::<usize>()?)
}