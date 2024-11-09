use std::collections::HashMap;
use std::fs;
use std::io::Read;
#[derive(Clone, Debug)]
pub struct ByteCodeInterpreter {
    instructions: ByteCodeObject,
    tape: Vec<u8>,
    tape_pointer: usize,
    output_log: String,
}

impl ByteCodeInterpreter {
    pub fn new(instructions: ByteCodeObject) -> ByteCodeInterpreter {
        let tape: Vec<u8> = vec![0; 30000];
        ByteCodeInterpreter {
            instructions,
            tape,
            tape_pointer: 0,
            output_log: String::new(),
        }
    }
    pub fn execute_program(&mut self) {
        self.execute_program_helper(&self.instructions.clone());
    }
    fn execute_program_helper(&mut self, instructions: &ByteCodeObject) {
        let mut symbol_num = 0;
        while symbol_num < instructions.instructions.len() {
            match &instructions.instructions[symbol_num] {
                Instruction::Memory(operand) => {
                    self.tape[self.tape_pointer] =
                        self.tape[self.tape_pointer].wrapping_add(*operand);
                }
                Instruction::Pointer(operand) => {
                    self.tape_pointer = self.tape_pointer.wrapping_add(*operand);
                }
                Instruction::Loop(instructions2) => {
                    if self.tape[self.tape_pointer] != 0 {
                        self.execute_program_helper(instructions2);
                        continue;
                    }
                }
		Instruction::ClearCell => {
			self.tape[self.tape_pointer]=0;
		}
                Instruction::Input => {
                    let mut buffer = [0u8; 1];
                    let _ = std::io::stdin().read_exact(&mut buffer);
                    let num = buffer[0];
                    self.tape[self.tape_pointer] = num;
                }
                Instruction::Output => {
                    let thing = self.tape[self.tape_pointer] as char;
                    self.output_log.push(thing);
                    print!("{thing}");
                }
            }
            symbol_num += 1;
        }
    }
}
#[derive(Clone, Debug)]
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
        let mut i = 0;
        while i < program.len() {
            let instruction = match program[i] {
                '+' => Instruction::add(1),
                '-' => Instruction::add(255),
                '>' => Instruction::add_pointer(1),
                '<' => Instruction::add_pointer(usize::MAX),
                '[' => {
                    let matching: usize = *matches.get(&i).expect("not in dictionary");
                    let subprogram = &program[i + 1..matching];
                    let obj = ByteCodeObject::new(subprogram);
                    i = matching;
                    Instruction::loop_instruction(obj)
                }
                ',' => Instruction::input(),
                _ => Instruction::output(),
            };
            instructions.push(instruction);
            i += 1;
        }
        let mut tmp = ByteCodeObject { instructions };
        tmp.optimize();
        tmp
    }
    fn group_add_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            let mut instruction: Instruction = self.instructions[i].clone();
            if let Instruction::Loop(ref mut instructions) = instruction {
                instructions.group_add_instructions();
            }
            if let Instruction::Memory(ref mut val) = instruction {
                while i < self.instructions.len() - 1 {
                    if let Instruction::Memory(operand2) = &self.instructions[i + 1] {
                        i += 1;
                        (*val) = (*val).wrapping_add(*operand2);
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
            let mut instruction: Instruction = self.instructions[i].clone();
            if let Instruction::Loop(ref mut instructions) = instruction {
                instructions.group_add_pointer_instructions();
            }
            if let Instruction::Pointer(ref mut val) = instruction {
                while i < self.instructions.len() - 1 {
                    if let Instruction::Pointer(operand2) = &self.instructions[i + 1] {
                        i += 1;
                        (*val) = (*val).wrapping_add(*operand2);
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
    fn add_clear_cell_instructions(&mut self) {
	    let mut instructions: Vec<Instruction> = Vec::new();
	    let mut i=0;
	    while i<self.instructions.len() {
		    let mut instruction = self.instructions[i].clone();
		    if let Instruction::Loop(ref mut instructions2) = instruction {
			    if instructions2.instructions.len() == 1 {
				    if let Instruction::Memory(val) = instructions2.instructions[0] {
					    if val == 255 {
						    instruction = Instruction::ClearCell;
					    }
				    }
			    }
			    else {
instructions2.add_clear_cell_instructions();
			    }
		    }
		    instructions.push(instruction);
		    i+=1;
	    }
	    self.instructions = instructions;
    }
    fn optimize(&mut self) {
        self.group_add_instructions();
        self.group_add_pointer_instructions();
	    self.add_clear_cell_instructions();
    }
}
#[derive(Clone, Debug)]
enum Instruction {
    Memory(u8),
    Pointer(usize),
    Loop(ByteCodeObject),
	ClearCell,
    Input,
    Output,
}
impl Instruction {
    fn add(num: u8) -> Instruction {
        Instruction::Memory(num)
    }
    fn loop_instruction(instructions: ByteCodeObject) -> Instruction {
        Instruction::Loop(instructions)
    }
    fn input() -> Instruction {
        Instruction::Input
    }
    fn output() -> Instruction {
        Instruction::Output
    }
    fn add_pointer(num: usize) -> Instruction {
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
fn get_matches(data: &[char]) -> HashMap<usize, usize> {
    let mut matches: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<usize> = Vec::new();
    for (i, v) in data.iter().enumerate() {
        if v == &'[' {
            stack.push(i);
        }
        if v == &']' {
            let start = stack.pop().expect("brackets do not match");
            matches.insert(start, i);
        }
    }
    matches
}
pub fn read_program(filename: &str) -> Vec<char> {
    fs::read_to_string(filename).unwrap().chars().collect()
}
