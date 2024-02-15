# image-to-ppm

[PPM](https://en.wikipedia.org/wiki/Netpbm) is like the [WAV](https://en.wikipedia.org/wiki/WAV) file of images. It's just raw pixel data. 
So I wrote this Rust program to make images worse by converting them to PPM.

Currently, this program can convert JPG and PNG images to PPM.

You can install the program using [Cargo](https://crates.io/crates/image-to-ppm), simply run: `cargo install image-to-ppm` and then you can use the command from your command line (assuming your Rust installation is correctly configured).

## Building

You can build the project using the traditional `cargo build` command. If you need a release build, you can use `cargo build --release`.

## Usage

To use it, simply run: `image-to-ppm <input file> <output file>`
 
 - `<input file>` will accept any .jpg or .png image.
 - `<output file>` will accept any file that ends in .ppm.

You can use `image-to-ppm --help` to see this information in the shell.