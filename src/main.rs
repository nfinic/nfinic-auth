use std::env;

extern crate jsonwebtoken as jwt;
use jwt::{encode,decode,Header,Algorithm,Validation};
use jwt::errors::ErrorKind;

#[macro_use]
extern crate serde_derive;

mod encode;

mod site;

#[derive(Debug,Serialize,Deserialize)]

struct Claims{
    sub: String,
    company: String,
    exp: usize
}

fn main() {

    let mut my_claims = Claims{
        sub:String::from("elielmathe@nfinic.com"),
        company:String::from("nfinic"),
        exp: 1_000_000_000
    };

    let key = "secret";

    let token = encode(&Header::default(),&my_claims,key.as_ref());

    let arguments : Vec<String> = env::args().collect();

    println!("Operation: {}",&arguments[1]);

    if &arguments[1] == "encode" {
        println!("We encode");
        encode::get_string("Eliel est la");

        if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
           let version = u64::from_str_radix(&v, 16).unwrap();

           if version >= 0x1_01_01_00_0 {
               println!("cargo:rustc-cfg=openssl111");
           }else{
               println!("No, cargo is a bit depracated");
           }
       }else{
           println!("Not found");
       }
    }else{
        println!("Decode not yet implemented yet");
    }
}
