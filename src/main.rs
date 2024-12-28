use brainfuck::*;
use clap::Parser;
use std::time::Instant;
#[derive(Parser)]
#[command(name = "brainfuck_interpreter")]
#[command(about = "brainfuck interpreter")]
struct Cli {
    #[arg()]
    file_name: Option<String>,
    #[arg(short = 'c', long)]
    brainfuck_code: Option<String>,
    #[arg(short, long = "time")]
    time: bool,
    #[arg(short, long = "no-optimize")]
    disable_optimizations: bool,
    #[arg(short, long = "no-execute")]
    no_execute_code: bool,
}
fn main() {
    let st = Instant::now();
    let cli = Cli::parse();
    let mut bytecode_object = if let Some(file_name) = cli.file_name {
        ByteCodeObject::from_file(&file_name)
    } else if let Some(brainfuck_code) = cli.brainfuck_code {
        let tmp = brainfuck_code.chars().collect::<Vec<char>>();
        ByteCodeObject::unoptimized_new(&tmp)
    } else {
        eprintln!("no file or code object passed");
        return;
    };
    if !cli.disable_optimizations {
        bytecode_object.optimize();
    }
    let mut bytecode_interpreter = ByteCodeInterpreter::new();
    if !cli.no_execute_code {
        bytecode_interpreter.execute_program(&bytecode_object);
    }
    let en = st.elapsed();
    if cli.time {
        println!("{en:?}");
    }
}
