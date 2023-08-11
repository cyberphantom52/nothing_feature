mod nt_feature;
use crate::nt_feature::*;
use std::str::FromStr;

fn parse_hex(mut hex: String) -> Option<u64> {
    if hex.len() > 2 && &hex[0..2] == "0x" {
        hex = hex.strip_prefix("0x").unwrap().to_string();
    }

    if hex.chars().all(|c| c.is_digit(16)) {
        return u64::from_str_radix(&hex, 16).ok();
    }
    None
}

fn binary_to_hex(binary: &str) -> String {
    let n: u64 = u64::from_str_radix(binary, 2).unwrap();
    format!("0x{:x}", n)
}

fn show_usage() {
    println!("Usage: nothing_feature <feature_base> <feature_device> <feature_product> [options] [features]");
    println!("");
    println!("Options:");
    println!("  --help     Show this help message");
    println!("  --base     Set the base feature set");
    println!("  --device   Set the device feature set");
    println!("  --product  Set the product feature set");
    println!("  --enable   Enable the specified features");
    println!("  --disable  Disable the specified features");
    println!("");
    println!("Features:");
    for i in 0..35 {
        println!("  {:2} - {:?}", i, NtFeature::from(i));
    }
}

pub fn gen_custom_feature(
    mut feature_base: u64,
    feature_device: u64,
    feature_product: u64,
    enable: Vec<NtFeature>,
    disable: Vec<NtFeature>,
) -> String {
    feature_base ^= feature_device ^ feature_product;
    let mut feature_set = feature_base;

    // Set enable bits
    for feature in enable {
        feature_set |= 1 << feature as u8;
    }

    // Clear disable bits
    for feature in disable {
        feature_set &= !(1 << feature as u8);
    }

    // Get the final result
    let result = feature_base ^ feature_set;

    binary_to_hex(&format!("{:b}", result))
}

fn main() {
    // Setup the arguments
    let mut args = std::env::args().skip(1).peekable();
    let mut feature_base: u64 = 0;
    let mut feature_device: u64 = 0;
    let mut feature_product: u64 = 0;
    let mut enable: Vec<NtFeature> = Vec::new();
    let mut disable: Vec<NtFeature> = Vec::new();

    // Parse the arguments
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--base" => {
                feature_base = parse_hex(args.next().unwrap()).unwrap_or(0);
            }
            "--device" => {
                feature_device = parse_hex(args.next().unwrap()).unwrap_or(0);
            }
            "--product" => {
                feature_product = parse_hex(args.next().unwrap()).unwrap_or(0);
            }
            "--enable" => {
                while let Some(feature) = args.peek() {
                    if feature.as_str().starts_with("--") {
                        break;
                    }
                    match NtFeature::from_str(&feature) {
                        Ok(feature) => enable.push(feature),
                        Err(_) => println!("Unknown feature: {}", feature),
                    }
                    args.next();
                }
            }
            "--disable" => {
                // keep pushing features until we hit a "--enable" or run out of args
                while let Some(feature) = args.peek() {
                    if feature.as_str().starts_with("--") {
                        break;
                    }
                    match NtFeature::from_str(&feature) {
                        Ok(feature) => disable.push(feature),
                        Err(_) => println!("Unknown feature: {}", feature),
                    }
                    args.next();
                }
            }
            "--help" => {
                show_usage();
                return;
            }
            _ => {
                println!("unknown argument: {}", arg);
                show_usage();
            }
        }
    }
    // base - 0x4245b9fff
    // device - 0x3d1a42000
    // product - 0x0
    let custom_str = gen_custom_feature(
        feature_base,
        feature_device,
        feature_product,
        enable,
        disable,
    );

    println!("{}", custom_str)
}
