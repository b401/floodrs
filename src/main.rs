mod floodrs;

#[macro_use(value_t)]
extern crate clap;

use clap::{App, Arg};
use std::path::Path;

fn main() -> Result<(), image::ImageError> {
    let arg = App::new("floodrs")
        .version("0.1")
        .about("Pixelflut rs client")
        .author("buuky")
        .arg(
            Arg::with_name("FILE")
                .short("f")
                .help("Path to Image")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("IP")
                .short("ip")
                .help("Server IP")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("PORT")
                .short("p")
                .help("Server PORT")
                .required(false)
                .index(3),
        )
        .arg(
            Arg::with_name("xOffset")
                .short("x")
                .help("X Offset")
                .required(false)
                .index(4),
        )
        .arg(
            Arg::with_name("yOffset")
                .short("y")
                .help("y Offset")
                .required(false)
                .index(5),
        )
        .arg(
            Arg::with_name("repeat")
                .short("r")
                .help("redraw infinitely")
                .required(false)
                .min_values(0)
                .index(6),
        )
        .get_matches();

    let pixelflut_infos = floodrs::Config::new(
        arg.value_of("IP").unwrap_or("127.0.0.1"),
        value_t!(arg.value_of("PORT"), u16).unwrap_or(1234u16),
        floodrs::open_image(Path::new(arg.value_of("FILE").unwrap()))?,
        value_t!(arg.value_of("xOffset"), u32).unwrap_or(0u32),
        value_t!(arg.value_of("yOffset"), u32).unwrap_or(0u32),
        arg.is_present("repeat"),
    );

    floodrs::run(pixelflut_infos)?;
    Ok(())
}
