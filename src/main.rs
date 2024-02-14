use std::{fs::File, io::Write};

use clap::{Parser};
use image::io::Reader;

#[derive(Parser)]
struct Cli {
    /// The image you want to convert to PPM (supported formats: .png)
    input: String,
    /// The output file name and path
    output: String,
}

fn main() {
    let args = Cli::parse();
    
    let result = Reader::open(args.input);
    match result {
        Ok(reader) => {
            if let Ok(img) = reader.decode() {
                let bytes = img.as_bytes();
                write_ppm_image(args.output, img.width(), img.height(), bytes);
            }
        },
        Err(err) => {
            println!("error: {}", err.to_string())
        }
    }

}

fn write_ppm_image(path: String, width: u32, height: u32, bytes: &[u8]) {
    println!("Image is {} by {}", width, height);
    if let Ok(mut file) = File::create(&path) {
        
        let mut data = String::new();

        // metadata
        data.push_str("P6");
        data.push(' ');
        data.push_str(width.to_string().as_str());
        data.push(' ');
        data.push_str(height.to_string().as_str());
        data.push(' ');

        let largest_pixel_value = bytes.into_iter().max().unwrap().to_string();
        data.push_str(&largest_pixel_value);

        data.push('\n');
        file.write(data.as_bytes()).expect("Failed to write metadata to the file");

        let image_has_alpha = ((bytes.len() / 4) as u32) == width * height;
        println!("Bytes to write: {}", bytes.len());
        println!("Image has alpha layer: {}", image_has_alpha);

        let mut pixels_written = 0;
        if image_has_alpha {
            // write
            let mut index = 1;
            while index < bytes.len() {
                if index % 4 == 0 { // skip alpha byte
                    file.write(&bytes[index - 4..index - 1]).expect("Failed to write pixels to file");
                    pixels_written += 1;
                }
                index += 1;
            }

            // write final pixel
            file.write(&bytes[index - 4..index - 1]).expect("Failed to write pixels to file");
            pixels_written += 1;
        } else {
            file.write(bytes).expect("Failed to write pixels to file");
            pixels_written += bytes.len() / 3;
        }


        println!("{} pixels written to {}", pixels_written, &path);
    }
}