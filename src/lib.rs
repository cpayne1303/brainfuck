use std::collections::HashMap;
use std::fs;
use std::io::Read;
#[derive(Clone)]
pub struct ByteCodeInterpreter {
    instructions: ByteCodeObject,
    tape: Vec<u8>,
    tape_pointer: usize,
}

impl ByteCodeInterpreter {
    pub fn new(instructions: ByteCodeObject) -> ByteCodeInterpreter {
        let tape: Vec<u8> = vec![0; 30000];
        ByteCodeInterpreter {
            instructions,
            tape,
            tape_pointer: 0,
        }
    }
    pub fn execute_program(&mut self) {
	    println!("running");
	    self.execute_program_helper(&self.instructions.clone());
    }
    fn execute_program_helper(&mut self, instructions: &ByteCodeObject) {
        let mut symbol_num = 0;
        while symbol_num < instructions.instructions.len() {
		// println!("{symbol_num}");
            match &instructions.instructions[symbol_num] {
                Instruction::Memory(operand) => {
                    if let Option::Some(val) = operand {
                        self.tape[self.tape_pointer] =
                            self.tape[self.tape_pointer].wrapping_add(*val);
                    } else {
                        self.tape[self.tape_pointer] = self.tape[self.tape_pointer].wrapping_add(1);
                    }
                }
                Instruction::Pointer(operand) => {
                    if let Option::Some(val) = operand {
                        self.tape_pointer = self.tape_pointer.wrapping_add(*val);
                    } else {
                        self.tape_pointer += 1;
                    }
                }
                Instruction::LoopInstruction(instructions2) => {
                    if self.tape[self.tape_pointer]!=0 {
self.execute_program_helper(instructions2);
			    continue;
				    }
                }
                Instruction::Input => {
                    let mut buffer = [0u8; 1];
                    let _ = std::io::stdin().read_exact(&mut buffer);
                    let num = buffer[0];
                    self.tape[self.tape_pointer] = num;
                }
                Instruction::Output => {
                    let thing = self.tape[self.tape_pointer] as char;
                    print!("{thing}");
                }
            }
	symbol_num+=1;
    }
    }
}
#[derive(Clone)]
pub struct ByteCodeObject {
    instructions: Vec<Instruction>,
}
impl ByteCodeObject {
	pub fn from_file(fname: &str) -> ByteCodeObject {
		let program = read_program(fname);
		ByteCodeObject::new(&program)
	}
    pub fn new(program2: &[char]) -> ByteCodeObject {
        let program = cleanup(program2);
        let mut instructions: Vec<Instruction> = Vec::new();
	    let matches = get_matches(&program);
let mut i=0;
	    while i<program.len() {
            let instruction = match program[i] {
                '+' => Instruction::add(Option::Some(1)),
                '-' => Instruction::add(Option::Some(255)),
                '>' => Instruction::add_pointer(Option::Some(1)),
                '<' => Instruction::add_pointer(Option::Some(usize::MAX)),
                '[' => {
			let matching:usize = *matches.get(&i).expect("not in dictionary");
			let subprogram = &program[i+1..matching];
			let obj = ByteCodeObject::new(subprogram);
			i=matching;
			Instruction::LoopInstruction(obj)
			}
                ',' => Instruction::input(),
                _ => Instruction::output(),
            };
            instructions.push(instruction);
	    i+=1;
        }
        let mut tmp = ByteCodeObject { instructions };
        tmp.optimize();
        tmp
    }
    fn group_add_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            let mut instruction:Instruction = self.instructions[i].clone();
		if let Instruction::LoopInstruction(ref mut instructions) = instruction {
			instructions.group_add_instructions();
			// instruction = Instruction::LoopInstruction(instructions);
		}
            if let Instruction::Memory(Some(ref mut val)) = instruction {
                while i < self.instructions.len() - 1 {
                    if let Instruction::Memory(operand2) = &self.instructions[i + 1] {
                        i += 1;
                        if let Option::Some(val2) = operand2 {
                            (*val) = (*val).wrapping_add(*val2);
                        }
                    } else {
                        break;
                    }
                }
            };
            instructions.push(instruction);
            i += 1;
        }
        self.instructions = instructions;
    }
    fn group_add_pointer_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            let mut instruction:Instruction = self.instructions[i].clone();
		if let Instruction::LoopInstruction(ref mut instructions) = instruction {
			instructions.group_add_pointer_instructions();
		}
            if let Instruction::Pointer(Some(ref mut val)) = instruction {
                while i < self.instructions.len() - 1 {
                    if let Instruction::Pointer(operand2) = &self.instructions[i + 1] {
                        i += 1;
                        if let Option::Some(val2) = operand2 {
                            (*val) = (*val).wrapping_add(*val2);
                        }
                    } else {
                        break;
                    }
                }
            }
            instructions.push(instruction);
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
enum Instruction {
    Memory(Option<u8>),
    Pointer(Option<usize>),
    LoopInstruction(ByteCodeObject),
    Input,
    Output,
}
impl Instruction {
    fn add(num: Option<u8>) -> Instruction {
        Instruction::Memory(num)
    }
    fn loop_instruction(instructions: ByteCodeObject) -> Instruction {
        Instruction::LoopInstruction(instructions)
    }
    fn input() -> Instruction {
        Instruction::Input
    }
    fn output() -> Instruction {
        Instruction::Output
    }
    fn add_pointer(num: Option<usize>) -> Instruction {
        Instruction::Pointer(num)
    }
}
fn cleanup(program: &[char]) -> Vec<char> {
    let instructions = ['+', '-', '<', '>', '[', ']', ',', '.'];
    program
        .iter()
        .filter(|i| instructions.contains(i))
        .copied()
        .collect::<Vec<char>>()
}
fn find_matching(data: &[char], symbol_num: usize) -> usize {
    let mut left = 1;
    let mut right = 0;
    let mut symbol_num2 = symbol_num;
    while right < left {
	    // println!("{symbol_num2}");
        symbol_num2 += 1;
        match &data[symbol_num2] {
            '[' => {
                left += 1;
            }
            ']' => {
                right += 1;
            }
            _ => {}
        }
    }
    symbol_num2
}
fn get_matches(data: &[char]) -> HashMap<usize, usize> {
    let mut matches: HashMap<usize, usize> = HashMap::new();
    for (i, v) in data.iter().enumerate() {
        if v == &'[' {
            let matching = find_matching(data, i);
            matches.insert(i, matching);
        }
    }
    matches
}
pub fn read_program(filename: &str) -> Vec<char> {
    fs::read_to_string(filename).unwrap().chars().collect()
}
