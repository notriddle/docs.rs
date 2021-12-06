use std::{
    env,
    fs::{read_dir, File},
    io::{Read, Write},
    path::Path,
};

fn main() {
    println!("cargo:rustc-cfg=font_awesome_out_dir");
    write_fontawesome_sprite();
}

fn write_fontawesome_sprite() {
    let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("fontawesome.rs");
    let mut dest_file = File::create(&dest_path).unwrap();
    dest_file
        // Using `as_bytes()` here is a workaround for a limitation of const fn,
        // which produces E0015 when we match on a str directly, but is fine with [u8]:
        // https://play.rust-lang.org/?version=stable&edition=2021&gist=2425956c7cc5c5a04db41c5d9757b7ed
        .write_all(b"const fn fontawesome_svg(dir:&str,file:&str)->&'static str{match(dir.as_bytes(),file.as_bytes()){")
        .expect("fontawesome fn write");
    for dirname in &["brands", "regular", "solid"] {
        let dir = read_dir(Path::new("fontawesome-free-5.14.0-web/svgs").join(dirname)).unwrap();
        let mut data = String::new();
        for file in dir {
            let file = file.expect("fontawesome directory access");
            let filename = file
                .file_name()
                .into_string()
                .expect("fontawesome filenames are unicode");
            let mut file = File::open(file.path()).expect("fontawesome file access");
            data.clear();
            file.read_to_string(&mut data)
                .expect("fontawesome file read");
            // if this assert goes off, add more hashes here and in the format! below
            assert!(!data.contains("###"), "file {} breaks raw string", filename);
            dest_file
                .write_all(
                    format!(
                        r####"(b"{dirname}",b"{filename}")=>r###"{data}"###,"####,
                        data = data,
                        dirname = dirname,
                        filename = filename.replace(".svg", ""),
                    )
                    .as_bytes(),
                )
                .expect("write fontawesome file");
        }
    }
    dest_file
        .write_all(b"_=>\"\"}}")
        .expect("fontawesome fn write");
}
