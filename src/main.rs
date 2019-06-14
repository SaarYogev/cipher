extern crate structopt;

use structopt::StructOpt;
use std::collections::HashMap;

#[derive(StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Change to decipher mode
    #[structopt(short = "d")]
    decipher: bool,
    /// The cipher method
    cipher: String,
    /// Text to cipher
    text: String
}

trait CipherType {
    fn cipher(&self, text: &str) -> &str;
}

fn main() {
    let args = Opt::from_args();
    let mut ciphers: HashMap<&str, Box<CipherType>> = HashMap::new();
    ciphers.insert("Cesar", Box::new(CesarCipher));
    ciphers.insert("Fence", Box::new(FenceCipher));
    ciphers.get(args.cipher.as_str()).unwrap().cipher(&args.text);
}

struct CesarCipher;
impl CipherType for CesarCipher {
    fn cipher(&self, text: &str) -> &str{
        unimplemented!()
    }
}

struct FenceCipher;
impl CipherType for FenceCipher {
    fn cipher(&self, text: &str) -> &str{
        unimplemented!()
    }
}
