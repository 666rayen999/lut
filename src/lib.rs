use image::{open, GenericImageView, ImageBuffer, Rgb};

pub fn gen_lut(lut_path: &str) -> Result<(), ()> {
    ImageBuffer::from_fn(512, 512, |x, y| {
        let r = ((x & 63) << 2) as u8;
        let g = ((y & 63) << 2) as u8;
        let b = (((x >> 6) << 2) + ((y >> 6) << 5)) as u8;
        Rgb([r, g, b])
    })
    .save(lut_path)
    .map_err(|_| ())
}

// pub fn apply_lut(image_path: &str, lut_path: &str, output_path: &str) -> Result<(), ()> {
//     let lut = open(lut_path).map_err(|_| ())?.into_rgb8();
//     let mut img = open(image_path).map_err(|_| ())?.into_rgb8();
//     img.pixels_mut().for_each(|rgb| {
//         let x = (rgb.0[0] >> 2) as u32 + ((((rgb.0[2] >> 2) & 7) as u32) << 6);
//         let y = (rgb.0[1] >> 2) as u32 + (((rgb.0[2] >> 5) as u32) << 6);
//         *rgb = *lut.get_pixel(x, y);
//     });
//     img.save(output_path).map_err(|_| ())
// }

pub fn unsafe_apply_lut(image_path: &str, lut_path: &str, output_path: &str) -> Result<(), ()> {
    let lut = open(lut_path).map_err(|_| ())?.into_rgb8();
    let mut img = open(image_path).map_err(|_| ())?.into_rgb8();
    img.pixels_mut().for_each(|rgb| {
        let x = (rgb.0[0] >> 2) as u32 + ((((rgb.0[2] >> 2) & 7) as u32) << 6);
        let y = (rgb.0[1] >> 2) as u32 + (((rgb.0[2] >> 5) as u32) << 6);
        *rgb = unsafe { lut.unsafe_get_pixel(x, y) };
    });
    img.save(output_path).map_err(|_| ())
}
