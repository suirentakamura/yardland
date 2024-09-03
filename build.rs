fn main() {
    return;

    println!("cargo:rerun-if-changed=src/processor/sys");

    cc::Build::new()
        .file("src/processor/sys/emu65x64.cpp")
        .cpp(true)
        .flag_if_supported("-std=c++20")
        .compile("emu65x64");
}
