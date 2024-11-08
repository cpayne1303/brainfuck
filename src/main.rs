use brainfuck2::*;
use std::time::Instant;
fn main() {
    let st = Instant::now();
    let bytecode_object = ByteCodeObject::from_file("mandelbrot.b");
    let mut bytecode_interpreter = ByteCodeInterpreter::new(bytecode_object);
    bytecode_interpreter.execute_program();
    let en = st.elapsed();
    println!("{en:?}");
}
