use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::time::Instant;
#[derive(Clone)]
struct ByteCodeInterpreter {
    instructions: ByteCodeObject,
    tape: Vec<u8>,
    tape_pointer: usize,
}

impl ByteCodeInterpreter {
    fn new(instructions: ByteCodeObject) -> ByteCodeInterpreter {
        let tape: Vec<u8> = vec![0; 30000];
        ByteCodeInterpreter {
            instructions,
            tape,
            tape_pointer: 0,
        }
    }
    fn execute_program(&mut self) {
        let mut stack: Vec<usize> = Vec::new();
        let mut symbol_num = 0;
        let matches = get_matches(&self.instructions);
        let mut num = 0;
        while symbol_num < self.instructions.instructions.len() {
            // println!("{symbol_num}");
            match &self.instructions.instructions[symbol_num] {
                Instruction::Memory(memory_instruction) => {
                    match memory_instruction.instruction_type {
                        Type::Add => {
                            match memory_instruction.operand {
                                Option::Some(val) => {
                                    self.tape[self.tape_pointer] += val;
                                }
                                _ => {
                                    self.tape[self.tape_pointer] += 1;
                                }
                            }
                            symbol_num += 1;
                            continue;
                        }
                        Type::Loop => {
                            if self.tape[self.tape_pointer] == 0 {
                                symbol_num = matches.get(&symbol_num).unwrap() + 1;
                                continue;
                            } else {
                                stack.push(symbol_num);
                                symbol_num += 1;
                                continue;
                            }
                        }
                        Type::LoopEnd => {
                            if self.tape[self.tape_pointer] > 0 {
                                let tmp = *stack.last().unwrap();
                                symbol_num = tmp + 1;
                                continue;
                            } else {
                                symbol_num += 1;
                                stack.pop();
                                continue;
                            }
                        }
                        Type::Input => {
                            let mut buffer = [0u8; 1];
                            let _ = std::io::stdin().read_exact(&mut buffer);
                            num = buffer[0];
                            self.tape[self.tape_pointer] = num;
                            symbol_num += 1;
                            continue;
                        }
                        Type::Output => {
                            let thing = self.tape[self.tape_pointer] as char;
                            print!("{thing}");
                            symbol_num += 1;
                            continue;
                        }
                        _ => {}
                    }
                }
                Instruction::Pointer(pointer_instruction) => {
                    match pointer_instruction.instruction_type {
                        Type::AddPointer => {
                            match pointer_instruction.operand {
                                Option::Some(val) => {
                                    self.tape_pointer += val;
                                }
                                _ => {
                                    self.tape_pointer += 1;
                                }
                            }
                            symbol_num += 1;
                            continue;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
#[derive(Clone)]
struct ByteCodeObject {
    instructions: Vec<Instruction>,
}
impl ByteCodeObject {
    fn new(program2: &Vec<char>) -> ByteCodeObject {
        let program = cleanup(program2);
        let mut instructions: Vec<Instruction> = Vec::new();
        for code in program {
            let instruction = match code {
                '+' => Instruction::add(Option::Some(1)),
                '-' => Instruction::add(Option::Some(255)),
                '>' => Instruction::add_pointer(Option::Some(1)),
                '<' => Instruction::add_pointer(Option::Some(usize::MAX)),
                '[' => Instruction::loop_start(),
                ']' => Instruction::loop_end(),
                ',' => Instruction::input(),
                _ => Instruction::output(),
            };
            instructions.push(instruction);
        }
        let mut tmp = ByteCodeObject { instructions };
        tmp.optimize();
        tmp
    }
    fn group_add_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            // println!("first loop {i}");
            let mut instruction = self.instructions[i].clone();
            if let Instruction::Memory(ref mut instruction2) = instruction {
                // println!("instruction is of type Memory");
                if let Type::Add = instruction2.instruction_type {
                    // println!("instruction is an Add operation");
                    match instruction2.operand {
                        Option::Some(ref mut val) => {
                            // println!("the instruction has a valid operand");
                            // println!("all conditions met");
                            while i < self.instructions.len() - 1 {
                                // println!("looking at {} inside loop", i+1);
                                if let Instruction::Memory(instruction3) = &self.instructions[i + 1]
                                {
                                    // println!("the next instruction is also a Memory instruction");
                                    if let Type::Add = instruction3.instruction_type {
                                        // println!("the next instruction is also of type Add");
                                        i += 1;
                                        match instruction3.operand {
                                            Option::Some(val2) => {
                                                // println!("the next instruction also has a valid operand");
                                                *val = *val + val2;
                                                // println!("{val}");
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        // println!("the next instruction is not of type Add");
                                        break;
                                    }
                                } else {
                                    // println!("the next instruction is is not a Memory instruction");
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            let instruction4 = instruction.clone();
            instructions.push(instruction4);
            i += 1;
        }
        self.instructions = instructions;
    }
    fn group_add_pointer_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            // println!("first loop {i}");
            let mut instruction = self.instructions[i].clone();
            if let Instruction::Pointer(ref mut instruction2) = instruction {
                // println!("instruction is of type pointer");
                if let Type::AddPointer = instruction2.instruction_type {
                    // 	println!("instruction is an addpointer operation");
                    match instruction2.operand {
                        Option::Some(ref mut val) => {
                            // println!("the instruction has a valid operand");
                            // println!("all conditions met");
                            while i < self.instructions.len() - 1 {
                                // println!("looking at {} inside loop", i+1);
                                if let Instruction::Pointer(instruction3) =
                                    &self.instructions[i + 1]
                                {
                                    // println!("the next instruction is also a pointer instruction");
                                    if let Type::AddPointer = instruction3.instruction_type {
                                        // println!("the next instruction is also of type addpointer");
                                        i += 1;
                                        match instruction3.operand {
                                            Option::Some(val2) => {
                                                // println!("the next instruction also has a valid operand");
                                                *val = *val + val2;
                                                // 	println!("{val}");
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        // println!("the next instruction is not of type addpointer");
                                        break;
                                    }
                                } else {
                                    // 					println!("the next instruction is is not a pointer instruction");
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            let mut instruction4 = instruction.clone();
            instructions.push(instruction4);
            i += 1;
        }
        self.instructions = instructions;
    }
    fn optimize(&mut self) {
        self.group_add_instructions();
        self.group_add_pointer_instructions();
        println!("finished optimizing");
    }
}
#[derive(Clone)]
enum Type {
    Add,
    AddPointer,
    Loop,
    LoopEnd,
    Input,
    Output,
}
#[derive(Clone)]
struct MemoryInstruction {
    instruction_type: Type,
    operand: Option<u8>,
}
#[derive(Clone)]
struct PointerInstruction {
    instruction_type: Type,
    operand: Option<usize>,
}
#[derive(Clone)]
enum Instruction {
    Memory(MemoryInstruction),
    Pointer(PointerInstruction),
}
impl Instruction {
    fn add(num: Option<u8>) -> Instruction {
        let instruction_type = Type::Add;
        Instruction::Memory(MemoryInstruction {
            instruction_type,
            operand: num,
        })
    }
    fn loop_start() -> Instruction {
        let instruction_type = Type::Loop;
        Instruction::Memory(MemoryInstruction {
            instruction_type,
            operand: Option::None,
        })
    }
    fn loop_end() -> Instruction {
        let instruction_type = Type::LoopEnd;
        Instruction::Memory(MemoryInstruction {
            instruction_type,
            operand: Option::None,
        })
    }
    fn input() -> Instruction {
        let instruction_type = Type::Input;
        Instruction::Memory(MemoryInstruction {
            instruction_type,
            operand: Option::None,
        })
    }
    fn output() -> Instruction {
        let instruction_type = Type::Output;
        Instruction::Memory(MemoryInstruction {
            instruction_type,
            operand: Option::None,
        })
    }
    fn add_pointer(num: Option<usize>) -> Instruction {
        let instruction_type = Type::AddPointer;
        Instruction::Pointer(PointerInstruction {
            instruction_type,
            operand: num,
        })
    }
}
fn cleanup(program: &Vec<char>) -> Vec<char> {
    let instructions = ['+', '-', '<', '>', '[', ']', ',', '.'];
    program
        .iter()
        .filter(|i| instructions.contains(&i))
        .map(|i| *i)
        .collect::<Vec<char>>()
}
fn find_matching(data: &ByteCodeObject, symbol_num: usize) -> usize {
    let mut left = 1;
    let mut right = 0;
    let mut symbol_num2 = symbol_num;
    while right < left {
        symbol_num2 += 1;
        if let Instruction::Memory(instruction) = &data.instructions[symbol_num2] {
            match instruction.instruction_type {
                Type::Loop => {
                    left += 1;
                }
                Type::LoopEnd => {
                    right += 1;
                }
                _ => {}
            }
        }
    }
    symbol_num2
}
fn get_matches(data: &ByteCodeObject) -> HashMap<usize, usize> {
    let mut matches: HashMap<usize, usize> = HashMap::new();
    for (i, v) in data.instructions.iter().enumerate() {
        if let Instruction::Memory(v2) = v {
            if let Type::Loop = v2.instruction_type {
                let matching = find_matching(data, i);
                matches.insert(i, matching);
            }
        }
    }
    matches
}
fn read_program(filename: &str) -> Vec<char> {
    let contents: String = fs::read_to_string(filename).unwrap();
    contents.chars().collect()
}
fn main() {
    let st = Instant::now();
    let program = read_program("mandelbrot.b");
    let bytecode_object = ByteCodeObject::new(&program);
    let mut bytecode_interpreter = ByteCodeInterpreter::new(bytecode_object);
    bytecode_interpreter.execute_program();
    let en = st.elapsed();
    println!("{en:?}");
}
