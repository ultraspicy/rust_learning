fn main() {
    cc::Build::new()
        .file("src/extern/add.c")
        .file("src/extern/abs.c")
        .compile("whatever");
}
