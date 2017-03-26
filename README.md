# rust_stuff

installation:
1. install rustc and cargo
2. git clone and cd into this repo
3. cargo build
4. cargo run



PEM format keys can be generated with openssl:

 1366  openssl genrsa -out key.pem 4096
 1367  openssl rsa -in key.pem -pubout > key.pub


readpem.rs
	pub fn read_pem_file(f: String) -> Vec<u8>

main.rs
	reads key.pem and outputs:
	Extracting 2349 bytes from key.pem
	[48, 130, 9, 41, 2, 1, 0, 2, 130, 2, 1, 0, 190, 250, ...