extern crate rustc_serialize;
mod readpem;

fn main() {
    // Create a path to the desired file
    let filename:String = String::from("/root/rust_tls13/key.pem");
    
    let bytes:Vec<u8> = readpem::read_pem_file(filename);
    println!("retval: {:?}", bytes);
}
