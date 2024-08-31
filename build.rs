use std::env;

fn main() {
    match env::var("TARGET") {
        Ok(target)=>{
            match target.as_str() {
                "aarch64-unknown-linux-gnu"=>{println!("cargo:rustc-link-search=./lib/aarch64")}
                "x86_64-pc-windows-gnu"=>{println!("cargo:rustc-link-search=./lib/x86_64")}
                "x86_64-pc-windows-msvc"=>{println!("cargo:rustc-link-search=./lib/x86_64")}
                _=>{}
            }
        }
        Err(_)=>{}
    }    
}