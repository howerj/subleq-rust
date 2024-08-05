extern crate subleq;

fn main() {
    println!("SUBLEQ VM Running...");
    let mut vm = subleq::VM::new();
    std::process::exit(vm.run(&mut std::io::stdin(), &mut std::io::stdout()));
}
