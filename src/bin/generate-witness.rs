fn main() {
    #[cfg(feature = "build-witness")]
    witness::generate::build_witness();
}
