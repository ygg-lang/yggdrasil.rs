use peginator::buildscript::Compile;

fn main() {
    let path = format!("{}/src/ygg.rs", module_path!());
    println!("{}", path);
    Compile::file("ygg.ebnf").destination(path).format().run_exit_on_error();
    println!("cargo:rerun-if-changed=ygg.ebnf");
}
