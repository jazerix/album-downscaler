use std::{fs::File, io::{self, Read}};
use crate::JpegMarkers::{*};

fn main() -> io::Result<()> {
    println!("Attempting to read image!");

    let image_file = File::open("album.jpeg")?;
    let mut buffer = [0; 2];
    let byte_stream_iter = image_file
        .bytes()
        .map(|byte| byte.unwrap_or(0));

    let mut initalized = false;
    for byte in byte_stream_iter {
        buffer[0] = buffer[1];
        buffer[1] = byte;
        if initalized == false {
            initalized = true;
            continue;
        }
        match_marker(buffer);
        //println!("{:?}", buffer.map(|b| format!("{:02x}", b)));
    }

    println!("Finished reading image");
    Ok(())
}

fn match_marker(buffer: [u8; 2]) {
    let marker = JpegMarkers::from(buffer);
    if matches!(marker, Ignore) {
        return;
    } else if matches!(marker, EndOfImage) {
        println!("End of image!");
    } else if matches!(marker, StartOfImage) {
        println!("Start of image!");
    } else if matches!(marker, BaselineStartOfFrame) {
        println!("Baseline start of frame")
    }
}

#[derive(Debug)]
enum JpegMarkers {
    StartOfImage,
    EndOfImage,
    Ignore,
    BaselineStartOfFrame,
}

impl From<[u8; 2]> for JpegMarkers {
    fn from(value: [u8; 2]) -> Self {
        match value {
            [0xff, 0xd8] => StartOfImage,
            [0xff, 0xd9] => EndOfImage,
            [0xff, 0xc0] => BaselineStartOfFrame,
            _ => Ignore
        }
    }
}
