extern crate image;

use std::net::TcpStream;
use std::io::Write;
use image::open;
use std::path::Path;
use std::time::Instant;
use std::net::{Ipv4Addr,SocketAddrV4};

pub struct Config{
    pub server: SocketAddrV4,
    pub image: image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
    pub offset: (u32,u32),
    pub rpt: bool,
}


impl Config {
    pub fn new(ip: &str,port: u16,image: image::RgbaImage,x_offset: u32,y_offset: u32,repeat: bool) -> Config {
        Config {
            server : SocketAddrV4::new(
                        ip.parse().unwrap_or(
                            Ipv4Addr::new(127,0,0,1)
                        ),
                        port),
            image : image,
            offset: (x_offset, y_offset),
            rpt: repeat,
        }
    }
}

pub fn run(pixelflut_infos: Config) -> Result<(),std::io::Error> {
    let time: u128 = write_pixels(pixelflut_infos)?;
    println!("[*] {} px/s", time);
    Ok(())
}

pub fn open_image(image_path: &Path) -> Result<image::RgbaImage,image::ImageError> {
    let image: Result<image::DynamicImage, image::ImageError> = open(image_path);

    match image {
        Ok(imagebuffer) => Ok(imagebuffer.to_rgba()),
        Err(e) => Err(e),
    }
}

fn write_pixels(pixelflut_infos: Config) -> Result<u128,std::io::Error> {
    let con =  TcpStream::connect(pixelflut_infos.server);

    let (x,y) = pixelflut_infos.offset;
    let image_pixels = pixelflut_infos.image;

    let mut counter = 0;
    let time = Instant::now();

    match con {
        Ok(mut connection) => {
            loop {
                for pixel in image_pixels.enumerate_pixels() {
                    match connection.write(&format!("PX {} {} {:0>8}\n",
                                    x + pixel.0,
                                    y + pixel.1,
                                    pixel.2.data.iter().map(|s| format!("{:x}",s)).collect::<String>()).as_bytes()){
                        Ok(_) => counter += 1,
                        Err(e) => {
                            println!("[*] Err: {}",e);
                        },
                    }
                };
                if !pixelflut_infos.rpt {
                    break
                }
            };

            Ok(60*counter/(time.elapsed().as_millis()/100))
        }
        Err(e) => {
            Err(e)
        },
    }

}
