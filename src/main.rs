use brainfuck2::*;
use std::time::Instant;
fn main() {
    let st = Instant::now();
    let program = read_program("mandelbrot.b");
    let bytecode_object = ByteCodeObject::new(&program);
    let mut bytecode_interpreter = ByteCodeInterpreter::new(bytecode_object);
    bytecode_interpreter.execute_program();
    let en = st.elapsed();
    println!("{en:?}");
}
