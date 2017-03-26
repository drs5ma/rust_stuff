extern crate rustc_serialize;
mod readpem;

fn main() {
    // Create a path to the desired file
    let filename:String = String::from("key.pem");

    // Call fn from readpem module, store result in bytes
    let bytes:Vec<u8> = readpem::read_pem_file(filename.to_string());

    
    println!("Extracting {} bytes from {}",bytes.len(), filename);
    println!("{:?}", bytes);
}
