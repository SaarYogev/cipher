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
    fn get_name(&self) -> &str;
    fn cipher(&self, text: &str) -> &str;
}

fn main() {
    let ciphers_vec: Vec<Box<CipherType>> = vec![Box::new(CesarCipher), Box::new(FenceCipher)];
    let mut ciphers: HashMap<&str, Box<CipherType>> = HashMap::new();
    for cipher in ciphers_vec {
        let name = cipher.get_name();
        ciphers.insert(name, cipher);
    }
    let args = Opt::from_args();
    ciphers.get(args.cipher.as_str()).unwrap().cipher(&args.text);
}

struct CesarCipher;
impl CipherType for CesarCipher {

    fn get_name(&self) -> &str {
        return "Cesar";
    }

    fn cipher(&self, text: &str) -> &str{
        unimplemented!()
    }
}

struct FenceCipher;
impl CipherType for FenceCipher {

    fn get_name(&self) -> &str {
        return "Fence";
    }

    fn cipher(&self, text: &str) -> &str{
        unimplemented!()
    }
}
