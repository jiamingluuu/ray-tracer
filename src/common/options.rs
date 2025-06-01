use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Options {
    #[arg(short, long, value_parser = parse_resolution)]
    resolution: (usize, usize),

    #[arg(short = 'r', long, default_value_t = 5)]
    recursion_depth: usize,

    #[arg(short = 'o', long, default_value_t = String::from("output.ppm"))]
    filename: String,

    #[arg(short = 'f', long = "focal")]
    focal_length: f32,
}

fn parse_resolution(s: &str) -> Result<(usize, usize), String> {
    let (w, h) = s.split_once("x").ok_or("Expected <width>x<height>")?;
    let width = w
        .parse::<usize>()
        .map_err(|_| "<width> must be an unsigned int")?;
    let height = h
        .parse::<usize>()
        .map_err(|_| "<height> must be an unsigned int")?;

    Ok((width, height))
}
