use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-rust", "src"].iter().collect();

    cc::Build::new()
        .warnings(false)
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-rust");

    // let dir: PathBuf = ["tree-sitter-python", "src"].iter().collect();
    // cc::Build::new()
    //     .include(&dir)
    //     .file(dir.join("scanner.cc"))
    //     .cpp(true)
    //     .compile("tree-sitter-python");

    // let dir: PathBuf = ["tree-sitter-python", "src"].iter().collect();
    // cc::Build::new()
    //     .include(&dir)
    //     .file(dir.join("parser.c"))
    //     .compile("tree-sitter-python");

    let dir: PathBuf = ["tree-sitter-cpp", "src"].iter().collect();
    cc::Build::new()
        .warnings(false)
        .include(&dir)
        .object(dir.join("scanner.o"))
        .object(dir.join("parser.o"))
        .cpp(true)
        .compile("tree-sitter-cpp");

    // let dir: PathBuf = ["tree-sitter-cpp", "src"].iter().collect();
    // cc::Build::new()
    //     .include(&dir)
    //     .object(obj)
    //     .file(dir.join("parser.c"))
    //     .compile("tree-sitter-cpp");
}
