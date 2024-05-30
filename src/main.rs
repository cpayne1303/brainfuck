use std::io::Read;
use std::fs;
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
	let mut program2: Vec<char> = program.chars().collect();
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
		stack.pop();
		symbol_num = tmp;
		continue;
	}
	else {
		symbol_num += 1;
		stack.pop();
		continue;
	}
}
if program2[symbol_num] == ',' {
	let mut buffer = [0;1];
	println!("getting input");
	std::io::stdin().read_exact(&mut buffer).expect("error");
	tape[tape_pointer] = buffer[0] as u8;
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
execute_program(&program);
	}