use std::{env, fs, path::Path, process::Command};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    if cfg!(feature = "build-witness") {
        let witness_cpp = env::var("WITNESS_CPP").unwrap();
        let circuit_file = Path::new(&witness_cpp);
        let circuit_name = circuit_file.file_stem().unwrap().to_str().unwrap();
        let cpp = Path::new("./")
            .join(circuit_name.to_owned() + "_cpp")
            .join(circuit_name.to_owned() + ".cpp");
        if !cpp.exists() {
            let status = Command::new("circom")
                .args([
                    fs::canonicalize(circuit_file).unwrap().to_str().unwrap(),
                    "--c",
                ])
                .status()
                .unwrap();
            assert!(status.success());

            println!("cargo:warning=\"{}\"", cpp.to_str().unwrap());
        }
        let status = Command::new("./script/replace.sh")
            .arg(cpp.to_str().unwrap())
            .status()
            .unwrap();
        assert!(status.success());
        let clang = Path::new("/usr/bin/clang++");
        cxx_build::bridge("src/generate.rs")
            .file("src/circuit.cc")
            .cpp(true)
            .opt_level(0)
            .compiler(clang)
            .flag("-std=c++14")
            .flag("-w")
            .flag("-d")
            .flag("-g")
            .compile("witness");

        println!("cargo:rerun-if-changed={}", circuit_file.to_str().unwrap());
        println!("cargo:rerun-if-changed=src/circuit.cc");
    }
}