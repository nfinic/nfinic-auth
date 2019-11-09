use std::env;

extern crate jsonwebtoken as jwt;
use jwt::{encode,decode,Header,Algorithm,Validation};
use jwt::errors::ErrorKind;

#[macro_use]
extern crate serde_derive;

mod encode;

mod site;

mod var;

#[derive(Debug,Serialize,Deserialize)]

struct Claims{
    sub: String,
    company: String,
    exp: usize
}

#[derive(Debug,Serialize,Deserialize)]
struct NfinicClaims{
    code: String, //code du site
    hote: String, //nom du domaine
    fields: String //champ a echanger
}

fn main() {

    let arguments : Vec<String> = env::args().collect();

    println!("Operation: {}",&arguments[1]);

    if &arguments[1] == "encode" {

        let mut my_claims = NfinicClaims{
            code:String::from("AFJDKSFFSDHJDSFHJFSDHJFSHJFSD"),
            hote: String::from("https://do.nfinic.com"),
            fields: String::from("[username,email,telephone]")
        };

        let token = match encode(&Header::default(),&my_claims,(var::get_nf_key()).as_ref()){
            Ok(t) => t,
            Err(_) => panic!()
        };

        println!("\n\nThe token is : \n{}\n\n",token);

    }else{
        println!("Decode not yet implemented yet");
    }
}
