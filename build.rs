fn main() {
    println!("cargo:rerun-if-changed=src/processor/sys");

    cc::Build::new()
        .file("src/processor/sys/wdc816.cc")
        .file("src/processor/sys/mem816.cc")
        .file("src/processor/sys/emu816.cc")
        .file("src/processor/sys/ffi.cpp")
        .cpp(true)
        .flag_if_supported("-std=c++20")
        .compile("emu816");
}
