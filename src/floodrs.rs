extern crate image;

use image::gif::Decoder;
use image::open;
use image::AnimationDecoder;
use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::time::Instant;

pub struct Config {
    pub server: SocketAddrV4,
    pub image: ImageEnum,
    pub offset: (u32, u32),
    pub rpt: bool,
}

pub enum ImageEnum {
    GifImage(image::ImageResult<Vec<image::Frame>>),
    StaticImage(image::ImageBuffer<image::Rgba<u8>, Vec<u8>>),
}

impl Config {
    pub fn new(
        ip: &str,
        port: u16,
        image: ImageEnum,
        x_offset: u32,
        y_offset: u32,
        repeat: bool,
    ) -> Config {
        Config {
            server: SocketAddrV4::new(
                ip.parse().unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1)),
                port,
            ),
            image,
            offset: (x_offset, y_offset),
            rpt: repeat,
        }
    }
}

pub fn run(pixelflut_infos: Config) -> Result<(), std::io::Error> {
    let time: u128 = write_pixels(pixelflut_infos)?;
    println!("[*] {} px/s", time);
    Ok(())
}

pub fn open_image(image_path: &Path) -> ImageEnum {
    match image_path.extension().unwrap().to_str().unwrap() {
        "gif" => ImageEnum::GifImage(open_gif_image(image_path)),
        _ => ImageEnum::StaticImage(open(image_path).unwrap().to_rgba()),
    }
}

fn open_gif_image(image_path: &Path) -> image::ImageResult<Vec<image::Frame>> {
    let gif_file = File::open(image_path).expect("File not found");
    let decoder = Decoder::new(gif_file).unwrap();
    let frames = decoder.into_frames();
    frames.collect_frames()
}

fn write_pixels(pixelflut_infos: Config) -> Result<u128, std::io::Error> {
    let con = TcpStream::connect(pixelflut_infos.server);

    let (x, y) = pixelflut_infos.offset;

    let mut counter = 0;
    let time = Instant::now();

    // Caching
    let mut pixel_cache: Vec<Vec<u8>> = Vec::new();
    let mut frame_cache: Vec<image::RgbaImage> = Vec::new();

    match pixelflut_infos.image {
        ImageEnum::StaticImage(pixels) => frame_cache.push(pixels),
        ImageEnum::GifImage(frameholder) => match frameholder {
            Ok(frames) => {
                for frame in frames {
                    frame_cache.push(frame.into_buffer());
                }
            }
            Err(e) => panic!(e),
        },
    };

    for frame in frame_cache {
        for pixel in frame.enumerate_pixels() {
            let color = pixel
                .2
                .data
                .into_iter()
                .map(|s| format!("{:0>2x}", s))
                .collect::<String>();
            if color.ends_with("ff") {
                if pixel.2.data[2] != 0u8 {
                    pixel_cache.push(
                        format!("PX {} {} {}\n", x + pixel.0, y + pixel.1, color).into_bytes(),
                    );
                };
            };
        }
    }

    match con {
        Ok(mut connection) => {
            loop {
                for pixel in &pixel_cache {
                    match connection.write(&pixel) {
                        Ok(_) => counter += 1,
                        Err(e) => {
                            println!("[*] Err: {}", e);
                        }
                    }
                }

                if !pixelflut_infos.rpt {
                    break;
                };
            }
            Ok(60 * counter / (time.elapsed().as_millis() / 100))
        }
        Err(e) => Err(e),
    }
}
