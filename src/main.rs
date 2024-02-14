use std::{fs::File, io::Write};

use clap::{CommandFactory, Parser};
use clap::error::ErrorKind;
use image::io::Reader;

#[derive(Parser)]
struct Cli {
    /// The image you want to convert to PPM (supported formats: .png, .jpg)
    input: String,
    /// The output file name and path
    output: String,
}

fn main() {
    let args = Cli::parse();
    let mut command = Cli::command();

    if !args.output.ends_with(".ppm") {
        let error = command.error(ErrorKind::Io, "Please provide a valid output file name with the .ppm extension");
        error.exit();
    }

    let result = Reader::open(&args.input);
    match result {
        Ok(reader) => {
            if let Ok(img) = reader.decode() {
                let bytes = img.as_bytes();
                let width = img.width();
                let height = img.height();
                
                println!("{}: {} by {}", &args.input, width, height);
                println!("Bytes to write: {}", bytes.len());
                if let Ok(mut file) = File::create(&args.output) {
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

                    file.write(data.as_bytes())
                        .expect("Failed to write metadata to the file");
                    
                    let alpha_layer = img.color().has_alpha();
                    println!("Image has alpha layer: {}", if alpha_layer { "yes" } else { "no" });

                    let pixels_written;
                    if alpha_layer {
                        let mut index = 0;
                        while index < bytes.len() {
                            index += 4; // skip alpha layer
                            file.write(&bytes[index - 4..index - 1])
                                .expect("Failed to write pixels to file");
                        }
                        pixels_written = bytes.len() / 4;
                    } else {
                        file.write(bytes).expect("Failed to write pixels to file");
                        pixels_written = bytes.len() / 3;
                    }

                    println!("{}: {} pixels written", &args.output, pixels_written);
                }
            }
        }
        Err(err) => {
            let error = command.error(ErrorKind::Io, err.to_string());
            error.exit();
        }
    }
}
