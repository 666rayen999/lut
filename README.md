# LUT

This LUT app allows you to generate a 64³ PNG Lookup Table (LUT) and apply any PNG LUT to a given image. LUTs are used to map one set of colors to another, enabling various color grading effects for images.

### Features
- **Generate 64³ LUT**: Create a 64x64x64 PNG LUT file, so you can use it for color grading and visual effects.
- **Apply LUT to Image**: Apply any PNG LUT to an image to transform its colors based on the LUT.

### Installation
##### 1. Clone the Repository
```sh
git clone https://github.com/666rayen999/lut.git
cd lut/
```
##### 2. Build the Project
```sh
cargo b -r
```

### Usage
##### 1. Get the executable
```sh
cp target/release/lut .
```
##### 2. Generate a 64³ PNG LUT
```sh
./lut generate -o lut.png
# it defaults to "lut.png"
./lut generate
```
##### 3. Apply a PNG LUT to an Image
```sh
./lut apply -l lut.png -i "image.png" -o "output.png"
# -l defaults to "lut.png"
# -o defaults to "output.png"
./lut apply -i "image.png"
```

### Contributing

Contributions are welcome! If you have ideas for improvements or find a bug, feel free to open an issue or submit a pull request.

### License

This project is licensed under the MIT License.

### Acknowledgments

This project uses the [image](https://crates.io/crates/image) crate for image processing.
