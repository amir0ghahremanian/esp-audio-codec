use std::env;

enum MCU {
    Esp32,
    Esp32c3,
    Esp32s2,
    Esp32s3,
}

#[cfg(feature = "generate_binding")]
use std::path::PathBuf;

/// Generates a new binding at `src/lib.rs` using `src/wrapper.h`.
#[cfg(feature = "generate_binding")]
fn generate_binding() {
    const ALLOW_UNCONVENTIONALS: &'static str = "#![allow(non_upper_case_globals)]\n\
                                                 #![allow(non_camel_case_types)]\n\
                                                 #![allow(non_snake_case)]\n";

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .raw_line(ALLOW_UNCONVENTIONALS)
        .generate()
        .expect("Unable to generate binding");

    let binding_target_path = PathBuf::new().join("src").join("lib.rs");

    bindings
        .write_to_file(binding_target_path)
        .expect("Could not write binding to the file at `src/lib.rs`");

    println!("cargo:info=Successfully generated binding.");
}

fn link_lib(mcu: MCU) {
    println!("cargo:rustc-link-lib=static=esp_audio_codec");
    println!("cargo:rustc-link-search=native=./esp_audio_codec/lib/{}", match mcu {
        MCU::Esp32 => "esp32",
        MCU::Esp32c3 => "esp32c3",
        MCU::Esp32s2 => "esp32s2",
        MCU::Esp32s3 => "esp32s3"
    });
}

fn select_mcu() -> MCU {
    match env::var("MCU").expect("MCU environment variable not set").as_str() {
        "esp32" => MCU::Esp32,
        "esp32c3" => MCU::Esp32c3,
        "esp32s2" => MCU::Esp32s2,
        "esp32s3" => MCU::Esp32s3,
        _ => {
            panic!("MCU model not supported");
        }
    }
}

fn main() {
    #[cfg(feature = "generate_binding")]
    generate_binding();

    let mcu = select_mcu();

    link_lib(mcu);
}
