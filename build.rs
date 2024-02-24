


fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("ThreatExchange");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path]).build()?;

    b.flag_if_supported("-std=c++14")
    .compile("pdqhash-sys");

    println!("cargo:rerun-if-changed=src/main.rs");

    Ok(())
}