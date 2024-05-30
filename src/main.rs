use rand::Rng;
use std::time::Instant;
use std::io::Read;
use std::fs;
fn cleanup(program: &Vec<char>) -> Vec<char> {
	let mut program2: Vec<char> = Vec::with_capacity(program.len());
	for i in 0..program.len() {
		if program[i]=='+' || program[i]=='-' || program[i]=='<' || program[i]=='>' || program[i] == '[' || program[i] == ']' || program[i] == ',' || program[i] == '.' {
			program2.push(program[i]);
		}
	}
	program2
}
fn find_matching(program: &Vec<char>, symbol_num: usize) -> usize {
	let mut left=1;
	let mut right=0;
	let mut symbol_num2 = symbol_num;
	while right < left {
				symbol_num2+=1;
		if program[symbol_num2] == '[' {
			left+=1;
		}
		if program[symbol_num2] == ']' {
			right+=1;
		}
	}
	symbol_num2
}
fn execute_program(program: &str) {
	let mut tape: Vec<u8> = vec![0;30000];
	let mut stack: Vec<usize> = Vec::new();
	let mut tape_pointer = 0;
	let mut symbol_num = 0;
	let mut program3: Vec<char> = program.chars().collect();
	let mut program2 = cleanup(&program3);
	let mut num_digits=0;
	let mut rand = rand::thread_rng();
	while symbol_num < program2.len() {
		// println!("{symbol_num}");
		if program2[symbol_num] == '+' {
			tape[tape_pointer]+=1;
			symbol_num +=1;
			continue;
		}
		if program2[symbol_num] == '-' {
			tape[tape_pointer] -=1;
			symbol_num+= 1;
			continue;
		}
		if program2[symbol_num] == '>' {
			tape_pointer+=1;
			symbol_num += 1;
			continue;
		}
		if program2[symbol_num] == '<' {
			tape_pointer -= 1;
			symbol_num += 1;
			continue;
		}
		if program2[symbol_num] == '[' {
			stack.push(symbol_num);
			if tape[tape_pointer] == 0 {
				symbol_num = find_matching(&program2, symbol_num)+1;
				stack.pop();
				continue;
			}
			else {
				symbol_num += 1;
				continue;
			}
		}
if program2[symbol_num] == ']' {
	if tape[tape_pointer] > 0 {
		let tmp = *stack.last().unwrap();
		symbol_num = tmp+1;
		continue;
	}
	else {
		symbol_num += 1;
		stack.pop();
		continue;
	}
}
if program2[symbol_num] == ',' {
let mut num: u8 =10;
	if num_digits < 640 {
		if num_digits>0 {
		num = rand.gen_range(0..=9);
		}
		else {
			num = rand.gen_range(1..=9);
		}
		num+=48;
	}
	 if num_digits == 640 {
		 num=10;
		// println!("done");
	}
	if num_digits > 640 {
		// println!("causing program exit");
		num=0;
	}
		num_digits+=1;

	let num2 = num as char;
	// println!("{num_digits}");
	// println!("{num2}");
	tape[tape_pointer] = num as u8;
	symbol_num += 1;
	continue;
}
if program2[symbol_num] == '.' {
	let mut thing = tape[tape_pointer] as char;
	println!("{thing}");
	symbol_num+=1;
	continue;
}
else {
	symbol_num+=1;
	continue;
}
}
}
fn read_program(filename: &str) -> String {
	let mut contents:String = fs::read_to_string(filename).unwrap();
	contents
}
fn main() {
	let mut program = read_program("collatz.b");
	let st = Instant::now();
execute_program(&program);
	let en = st.elapsed();
	println!("{en:?}");
	}