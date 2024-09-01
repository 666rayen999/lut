use clap::{Arg, Command};
use lut::*;

fn main() {
    let matches = Command::new("LUT Generator + Applier")
        .version("1.0")
        .author("Rayen G. <666rayen999@gmail.com>")
        .about("Generates and applies 64³ PNG LUTs")
        .subcommand(
            Command::new("apply")
                .about("Applies a PNG LUT to an image")
                .arg(
                    Arg::new("lut")
                        .short('l')
                        .long("lut")
                        .value_name("LUT")
                        .default_value("lut.png"),
                )
                .arg(
                    Arg::new("image")
                        .short('i')
                        .long("image")
                        .value_name("IMAGE")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .default_value("output.png"),
                ),
        )
        .subcommand(
            Command::new("generate")
                .about("Generates a 64³ PNG LUT")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .default_value("output.png"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("apply") {
        let lut_path = matches.get_one::<String>("lut").unwrap();
        let image_path = matches.get_one::<String>("image").unwrap();
        let output_path = matches.get_one::<String>("output").unwrap();

        // apply_lut(lut_path, image_path, output_path);
        unsafe_apply_lut(&image_path, &lut_path, &output_path).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let output_path = matches.get_one::<String>("output").unwrap();

        gen_lut(&output_path).unwrap();
    }
}
