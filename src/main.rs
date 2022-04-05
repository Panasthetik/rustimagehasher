use error_chain::error_chain;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
}


extern crate image;
extern crate img_hash;
use hex;
extern crate crypto;
use base64;

use img_hash::{Hasher, HasherConfig, HashAlg};
use image::DynamicImage;

// use data_encoding::{HEXUPPER, DecodeError};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        } 
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn convert_base64_to_hex(bs64: &str) -> String {
    hex::encode(base64::decode(bs64).unwrap())
}


fn main() {
    let image1 = File::open("image1.png").unwrap();
    let image2 = image::open("image1.png").unwrap();

    let reader = BufReader::new(image1);

    
    let hasher = HasherConfig::new().to_hasher();

    let hash1 = hasher.hash_image(&image2);


    let printhash1 = hash1.to_base64();

    
    println!("Base64 of image: {}", printhash1);


    let heximage1 = convert_base64_to_hex(&printhash1);

    println!("HEX value of image: {}", heximage1);

    let digest1 = sha256_digest(reader);


    println!("SHA256 digest of the image: {:?}", digest1);





}



