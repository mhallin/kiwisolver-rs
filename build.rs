fn main() {
    cxx_build::bridge("src/sys.rs")
        .flag_if_supported("-std=c++17")
        .compile("kiwi");
}
