use std::env;

fn main() {
    let tux = env::var("TUXDIR").unwrap();
    println!("cargo:rustc-link-search={}/lib",tux);
    println!("cargo:rustc-link-lib=dylib=fml");
    println!("cargo:rustc-link-lib=dylib=fml32");
    println!("cargo:rustc-link-lib=dylib=tux");
    println!("cargo:rustc-link-lib=dylib=giconv");
    println!("cargo:rustc-link-lib=dylib=engine");
    println!("cargo:rustc-link-lib=dylib=buft");
    println!("cargo:rustc-link-lib=dylib=utrace");
    println!("cargo:rustc-link-lib=dylib=usort");
}
