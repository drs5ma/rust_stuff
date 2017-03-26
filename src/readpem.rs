use std::str;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rustc_serialize::base64::{FromBase64};


static BEGIN_RSA: &'static str = "-----BEGIN RSA PRIVATE KEY-----";
static END_RSA: &'static str = "-----END RSA PRIVATE KEY-----";

pub fn read_pem_file(f: String) -> Vec<u8>{
    // Path takes a reference to a string
    let path = Path::new(&f);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut data_string = String::new();

    match file.read_to_string(&mut data_string) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),

        Ok(_) => {}, //print!("{} contains:\n{}", display, data_string),
    }


    // this string holds our concatented b64 string to decode
    let mut b64_string = String::new();
    
    let lines = data_string.split("\n");
    for s in lines {
        if s == BEGIN_RSA{}
        else if s == END_RSA{}
        else { b64_string = b64_string + s.trim(); }
    }
    

   
    //couldn't fiugre out how to do proper error handling for b64 decode using match
    let bytes:Vec<u8> = b64_string.from_base64().unwrap();

    //return Vec<u8> representing the bytes of the base64 decoded pem key
    bytes
    
    
}
