use std::collections::HashMap;
use rand::Rng;
use std::time::Instant;
use std::io::Read;
use std::fs;
struct ByteCodeInterpreter {
	instructions: ByteCodeObject,
	tape: Vec<u8>,
	tape_pointer: usize,
}

impl ByteCodeInterpreter {
fn new(instructions: ByteCodeObject) -> ByteCodeInterpreter {
	let mut tape: Vec<u8> = vec![0;30000];
	ByteCodeInterpreter {
instructions,
		tape,
		tape_pointer: 0,
	}
}
fn execute_program(&mut self) {
	let mut stack: Vec<usize> = Vec::new();
	let mut symbol_num = 0;
	let mut num_digits=0;
	let matches = get_matches(&self.instructions);
	let mut num=10;
	let mut rand = rand::thread_rng();
	while symbol_num < self.instructions.instructions.len() {
		// println!("{symbol_num}");
match self.instructions.instructions[symbol_num].instruction_type {
Type::Add => {
	match self.instructions.instructions[symbol_num].operand {
		Option::Some(val) => {
			self.tape[self.tape_pointer]+=val;
		},
		_ => {
	self.tape[self.tape_pointer]+=1;
		},
	}
			symbol_num +=1;
			continue;
		},
Type::Sub => {
		match self.instructions.instructions[symbol_num].operand {
Option::Some(val) => {
			self.tape[self.tape_pointer] -= val;
		},
		_ => {
		self.tape[self.tape_pointer] -=1;
		},
	}
			symbol_num+= 1;
			continue;
		},
Type::AddPointer => {
			match self.instructions.instructions[symbol_num].operand {
Option::Some(val) => {
self.tape_pointer+=val;
},
_ => {
		self.tape_pointer+=1;
},
}
			symbol_num += 1;
			continue;
		},
Type::SubPointer => {
			match self.instructions.instructions[symbol_num].operand {
Option::Some(val) => {
self.tape_pointer-=val;
},
_ => {
		self.tape_pointer -= 1;
},
}
			symbol_num += 1;
			continue;
		},
Type::Loop => {
		if self.tape[self.tape_pointer] == 0 {
				symbol_num = matches.get(&symbol_num).unwrap()+1;
				continue;
			}
			else {
			stack.push(symbol_num);
				symbol_num += 1;
				continue;
			}
		},
Type::LoopEnd => {
		if self.tape[self.tape_pointer] > 0 {
		let tmp = *stack.last().unwrap();
		symbol_num = tmp+1;
		continue;
	}
	else {
		symbol_num += 1;
		stack.pop();
		continue;
	}
},
Type::Input => {
num =10;
	if num_digits < 2000 {
		if num_digits>0 {
		num = rand.gen_range(0..=9);
		}
		else {
			num = rand.gen_range(1..=9);
		}
		num+=48;
	}
	 if num_digits == 2000 {
		 num=10;
		// println!("done");
	}
	if num_digits > 2000 {
		// println!("causing program exit");
		num=0;
	}
		num_digits+=1;

	// let num2 = num as char;
	// println!("{num_digits}");
	// println!("{num2}");
	self.tape[self.tape_pointer] = num;
	symbol_num += 1;
	continue;
},
Type::Output => {
let thing = self.tape[self.tape_pointer] as char;
	println!("{thing}");
	symbol_num+=1;
	continue;
},
}
}
}

}
struct ByteCodeObject {
	instructions: Vec<Instruction>,
}
impl ByteCodeObject {
	fn new(program: &Vec<char>) -> ByteCodeObject {
let mut instructions: Vec<Instruction> = Vec::new();
		for code in program {
			let instruction = match code {
				'+' => Instruction::add(Option::None),
				'-' => Instruction::sub(Option::None),
				'>' => Instruction::add_pointer(Option::None),
				'<' => Instruction::sub_pointer(Option::None),
				'[' => Instruction::loop_start(),
				']' => Instruction::loop_end(),
				',' => Instruction::input(),
				_ => Instruction::output(),
			};
			instructions.push(instruction);
		}
ByteCodeObject {
	instructions,
}
	}
	}
enum Type {
	Add,
	Sub,
	AddPointer,
	SubPointer,
	Loop,
	LoopEnd,
	Input,
	Output,
}
struct Instruction {
	instruction_type: Type,
	operand: Option<u8>,
}
impl Instruction {
fn add(num: Option<u8>) -> Instruction {
	let mut instruction_type = Type::Add;
	Instruction {
		instruction_type,
		operand: num,
	}
}
fn add_pointer(num: Option<u8>) -> Instruction {
	let mut instruction_type = Type::AddPointer;
	Instruction {
		instruction_type,
		operand: num,
	}
}
fn sub(num: Option<u8>) -> Instruction {
	let mut instruction_type = Type::Sub;
	Instruction {
		instruction_type,
		operand: num,
	}
}
fn sub_pointer(num: Option<u8>) -> Instruction {
	let mut instruction_type = Type::SubPointer;
	Instruction {
		instruction_type,
		operand: num,
	}
}
fn loop_start() -> Instruction {
	let mut instruction_type = Type::Loop;
	Instruction {
		instruction_type,
		operand: Option::None,
	}
}
fn loop_end() -> Instruction {
	let mut instruction_type = Type::LoopEnd;
	Instruction {
		instruction_type,
		operand: Option::None,
	}
}
fn input() -> Instruction {
	let mut instruction_type = Type::Input;
	Instruction {
		instruction_type,
		operand: Option::None,
	}
}
fn output() -> Instruction {
	let mut instruction_type = Type::Output;
	Instruction {
		instruction_type,
		operand: Option::None,
	}
}
}
fn cleanup(program: &Vec<char>) -> Vec<char> {
	let instructions = ['+', '-', '<', '>', '[', ']', ',', '.'];
	let mut program2: Vec<char> = Vec::with_capacity(program.len());
	for i in program {
if instructions.contains(i) {
		program2.push(*i);
		}
	}
	program2
}
fn find_matching(data: &ByteCodeObject, symbol_num: usize) -> usize {
	let mut left=1;
	let mut right=0;
	let mut symbol_num2 = symbol_num;
	while right < left {
				symbol_num2+=1;
match data.instructions[symbol_num2].instruction_type {
	Type::Loop => {
		left+=1;
	},
	Type::LoopEnd => {
		right+=1;
	},
	_ => {},
}
		}
	symbol_num2
}
fn get_matches(data: &ByteCodeObject) -> HashMap<usize, usize> {
	let mut matches: HashMap<usize, usize> = HashMap::new();
	for (i, v) in data.instructions.iter().enumerate() {
		match v.instruction_type {
			Type::Loop => {
			let matching = find_matching(data, i);
			matches.insert(i, matching);
			},
			_ => {},
		}
	}
	matches
}
fn read_program(filename: &str) -> Vec<char> {
	let contents:String = fs::read_to_string(filename).unwrap();
	contents.chars().collect()
}
fn main() {
	let program = read_program("collatz.b");
	let bytecode_object = ByteCodeObject::new(&program);
	let mut bytecode_interpreter = ByteCodeInterpreter::new(bytecode_object);
	bytecode_interpreter.execute_program();
	let st = Instant::now();
	let en = st.elapsed();
	println!("{en:?}");
	}
