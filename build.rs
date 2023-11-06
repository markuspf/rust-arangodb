use cmake;

fn main() {
    let arango = cmake::Config::new("arangodb")
        .build_target("arango_aql")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/bin",
        arango.display()
    );
    println!("cargo:rustc-link-lib=static=arango_aql");

    cxx_build::bridge("src/lib.rs")
        .file("src/aql.cpp")
        .flag_if_supported("-std=c++20")
        .compile("aql");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/aql.cpp");
    println!("cargo:rerun-if-changed=include/aql.h");
}
