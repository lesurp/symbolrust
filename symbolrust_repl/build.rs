fn main() {
    lalrpop::Configuration::default()
        .set_out_dir(std::env::var("OUT_DIR").unwrap())
        .always_use_colors()
        .process()
        .unwrap();
}
