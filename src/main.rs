use reactor::{module::Module, runtime::Runtime};

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let file = std::fs::read(file).unwrap();

    let module = Module::new(&file[..]);

    let runtime = Runtime::new(&module);
    runtime.execute();
}
