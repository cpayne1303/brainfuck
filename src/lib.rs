use std::collections::HashMap;
use std::fs;
use std::io::Read;
#[derive(Clone, Debug)]
pub struct ByteCodeInterpreter {
    tape: Vec<u8>,
    tape_pointer: usize,
    output_log: String,
}

impl ByteCodeInterpreter {
    pub fn new() -> ByteCodeInterpreter {
        let tape: Vec<u8> = vec![0; 30000];
        ByteCodeInterpreter {
            tape,
            tape_pointer: 0,
            output_log: String::new(),
        }
    }
    pub fn execute_program(&mut self, instructions: &ByteCodeObject) {
        let mut symbol_num = 0;
        while symbol_num < instructions.instructions.len() {
		// println!("{:?}", &instructions.instructions[symbol_num]);
            match &instructions.instructions[symbol_num] {
                Instruction::Memory(operand) => {
                    self.tape[self.tape_pointer] =
                        self.tape[self.tape_pointer].wrapping_add(*operand);
                }
                Instruction::OffsetAdd((offset, val)) => {
                    self.tape[self.tape_pointer.wrapping_add(*offset)] =
                        self.tape[self.tape_pointer.wrapping_add(*offset)].wrapping_add(*val);
                }
                Instruction::Pointer(operand) => {
                    self.tape_pointer = self.tape_pointer.wrapping_add(*operand);
                }
		Instruction::Spread(memory_value, values) => {
			self.tape[self.tape_pointer]=self.tape[self.tape_pointer].wrapping_add(*memory_value);
			for (offset, val) in values.iter() {
				self.tape[self.tape_pointer.wrapping_add(*offset)] = self.tape[self.tape_pointer.wrapping_add(*offset)].wrapping_add(*val);
			}
		}
                Instruction::Loop(instructions2) => {
                    if self.tape[self.tape_pointer] != 0 {
                        self.execute_program(instructions2);
                        continue;
                    }
                }
                Instruction::ClearCell => {
                    self.tape[self.tape_pointer] = 0;
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
        ByteCodeObject::unoptimized_new(&program)
    }
    pub fn unoptimized_new(program2: &[char]) -> ByteCodeObject {
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
                    let obj = ByteCodeObject::unoptimized_new(subprogram);
                    i = matching;
                    Instruction::loop_instruction(obj)
                }
                ',' => Instruction::input(),
                _ => Instruction::output(),
            };
            instructions.push(instruction);
            i += 1;
        }
        ByteCodeObject { instructions }
    }
    pub fn new(program2: &[char]) -> ByteCodeObject {
        let mut tmp = ByteCodeObject::unoptimized_new(program2);
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
    fn add_spread_instructions(&mut self) {
let mut instructions = Vec::new();
	    let mut i=0;
	    while i<self.instructions.len() {
		    let mut instruction = self.instructions[i].clone();
		    match instruction {
			    Instruction::Loop(ref mut instructions2) => {
				    instructions2.add_spread_instructions();
				    instructions.push(instruction);
i+=1;
				    continue;
			    }
			    Instruction::ClearCell => {
				    instructions.push(instruction);
				    i+=1;
				    continue;
			    }
			    Instruction::Pointer(_) => {
				    instructions.push(instruction);
				    i+=1;
				    continue;
			    }
			    Instruction::Input => {
				    instructions.push(instruction);
				    i+=1;
				    continue;
			    }
			    Instruction::Output => {
				    instructions.push(instruction);
				    i+=1;
				    continue;
			    }
			    _ => {
let mut keepgoing = true;
				    let mut num_memory_instructions = 0;
				    if let Instruction::Memory(_) = instruction {
					    num_memory_instructions+=1;
				    }
				    let mut j = i;
				    while keepgoing && j+1<self.instructions.len() {
					    let next_instruction = self.instructions[j+1].clone();
					    match next_instruction {
						    Instruction::Memory(_) => {
							    num_memory_instructions += 1;
							    if num_memory_instructions>1 {
							    keepgoing=false;
						    }
					    }
					    Instruction::OffsetAdd(_) => {
						    keepgoing=true;
					    }
					    _ => {
						    keepgoing=false;
					    }
				    }
				    if keepgoing {
					    j+=1;
				    }
			    }
			    if i==j {
				    instructions.push(instruction);
				    i+=1;
				    continue;
			    }
			    else {
				    let mut memory_value = 0;
				    let mut k=i;
				    while k<=j {
					    let instruction3 = self.instructions[k].clone();
					    if let Instruction::Memory(val) = instruction3 {
						    memory_value = val;
						    break;
					    }
					    k+=1;
				    }
				    k=i;
				    let mut values: Vec<(usize, u8)> = Vec::new();
				    while k<=j {
					    let instruction4 = self.instructions[k].clone();
					    if let Instruction::OffsetAdd((offset2, val2)) = instruction4 {
						    values.push((offset2, val2));
					    }
					    k+=1;
				    }
				    instructions.push(Instruction::Spread(memory_value, values));
				    i=j+1;
				    continue;
			    }
		    }
	    }
    }
	    self.instructions = instructions;
    }
    fn add_clear_cell_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            let mut instruction = self.instructions[i].clone();
            if let Instruction::Loop(ref mut instructions2) = instruction {
                if instructions2.instructions.len() == 1 {
                    if let Instruction::Memory(val) = instructions2.instructions[0] {
                        if val == 255 {
                            instruction = Instruction::ClearCell;
                        }
                    } else {
                        instructions2.add_clear_cell_instructions();
                    }
                } else {
                    instructions2.add_clear_cell_instructions();
                }
            }
            instructions.push(instruction);
            i += 1;
        }
        self.instructions = instructions;
    }
    fn add_add_offset_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        let mut current_instruction = self.instructions[i].clone();
        while i < self.instructions.len() {
            if let Instruction::Loop(ref mut instructions2) = current_instruction {
                instructions2.add_add_offset_instructions();
                instructions.push(current_instruction);
                if i + 1 < self.instructions.len() {
                    current_instruction = self.instructions[i + 1].clone();
                } else {
                    break;
                }
                i += 1;
                continue;
            }
            if i + 1 == self.instructions.len() {
                instructions.push(current_instruction);
                break;
            }
            if let Instruction::Pointer(ref mut offset) = current_instruction {
                if let Instruction::Pointer(offset2) = self.instructions[i + 1] {
                    *offset = offset.wrapping_add(offset2);
                    i += 1;
                    continue;
                }
                if let Instruction::Memory(val) = self.instructions[i + 1] {
                    let current_instruction2 = Instruction::OffsetAdd((*offset, val));
                    let next_instruction = Instruction::Pointer(*offset);
                    instructions.push(current_instruction2);
                    current_instruction = next_instruction;
                    i += 1;
                } else {
                    instructions.push(current_instruction);
                    current_instruction = self.instructions[i + 1].clone();
                    i += 1;
                }
            } else {
                instructions.push(current_instruction);
                current_instruction = self.instructions[i + 1].clone();
                i += 1;
            }
        }
        self.instructions = instructions;
    }
    fn eliminate_dead_pointer_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            let mut instruction = self.instructions[i].clone();
            if let Instruction::Loop(ref mut instructions2) = instruction {
                instructions2.eliminate_dead_pointer_instructions();
                instructions.push(instruction);
                i += 1;
                continue;
            }
            if let Instruction::Pointer(offset) = instruction {
                if offset != 0 {
                    instructions.push(instruction);
                    i += 1;
                    continue;
                } else {
                    i += 1;
                }
            } else {
                instructions.push(instruction);
                i += 1;
                continue;
            }
        }
        self.instructions = instructions;
    }

    fn eliminate_dead_memory_instructions(&mut self) {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = 0;
        while i < self.instructions.len() {
            if let Instruction::Loop(ref mut instructions2) = self.instructions[i] {
                instructions2.eliminate_dead_memory_instructions();
                instructions.push(self.instructions[i].clone());
                i += 1;
                continue;
            }
            if let Instruction::Memory(val) = self.instructions[i] {
                if val != 0 {
                    instructions.push(self.instructions[i].clone());
                    i += 1;
                    continue;
                } else {
                    i += 1;
                }
            } else {
                instructions.push(self.instructions[i].clone());
                i += 1;
                continue;
            }
        }
        self.instructions = instructions;
    }
    fn eliminate_dead_offset_add_instructions(&mut self) {
        let mut i = 0;
        let mut instructions = Vec::new();
        while i < self.instructions.len() {
            let mut instruction = self.instructions[i].clone();
            match instruction {
                Instruction::Loop(ref mut instructions2) => {
                    instructions2.eliminate_dead_offset_add_instructions();
                    instructions.push(instruction);
                    i += 1;
                    continue;
                }
                Instruction::OffsetAdd((offset, val)) => {
                    if offset == 0 && val == 0 {
                        i += 1;
                        continue;
                    }
                    if val == 0 {
                        instructions.push(Instruction::Pointer(offset));
                        i += 1;
                        continue;
                    }
                    if offset == 0 {
                        instructions.push(Instruction::Memory(val));
                        i += 1;
                        continue;
                    } else {
                        instructions.push(instruction);
                        i += 1;
                        continue;
                    }
                }
                _ => {
                    instructions.push(instruction);
                    i += 1;
                    continue;
                }
            }
        }
        self.instructions = instructions;
    }
    pub fn optimize(&mut self) {
        self.group_add_instructions();
        self.group_add_pointer_instructions();
        self.add_clear_cell_instructions();
        self.add_add_offset_instructions();
        self.eliminate_dead_memory_instructions();
        self.eliminate_dead_pointer_instructions();
        self.eliminate_dead_offset_add_instructions();
	    self.add_spread_instructions();
    }
}
#[derive(Clone, Debug)]
enum Instruction {
    Memory(u8),
    Pointer(usize),
    OffsetAdd((usize, u8)),
	Spread(u8, Vec<(usize, u8)>),
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
#[test]
fn mandelbrot() {
    let bytecode_object = ByteCodeObject::from_file("./src/mandelbrot.b");
    let mut interpreter = ByteCodeInterpreter::new(bytecode_object);
    interpreter.execute_program();
    let correct_output = r#"AAAAAAAAAAAAAAAABBBBBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDEGFFEEEEDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAAAABBBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDEEEFGIIGFFEEEDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAABBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEFFFI KHGGGHGEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEFFGHIMTKLZOGFEEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAABBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEEFGGHHIKPPKIHGFFEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBBBB
AAAAAAAAAABBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGHIJKS  X KHHGFEEEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBB
AAAAAAAAABBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGQPUVOTY   ZQL[MHFEEEEEEEDDDDDDDCCCCCCCCCCCBBBBBBBBBBBBBB
AAAAAAAABBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEFFFFFGGHJLZ         UKHGFFEEEEEEEEDDDDDCCCCCCCCCCCCBBBBBBBBBBBB
AAAAAAABBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEFFFFFFGGGGHIKP           KHHGGFFFFEEEEEEDDDDDCCCCCCCCCCCBBBBBBBBBBB
AAAAAAABBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEEFGGHIIHHHHHIIIJKMR        VMKJIHHHGFFFFFFGSGEDDDDCCCCCCCCCCCCBBBBBBBBB
AAAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDEEEEEEFFGHK   MKJIJO  N R  X      YUSR PLV LHHHGGHIOJGFEDDDCCCCCCCCCCCCBBBBBBBB
AAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDEEEEEEEEEFFFFGH O    TN S                       NKJKR LLQMNHEEDDDCCCCCCCCCCCCBBBBBBB
AAAAABBCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDEEEEEEEEEEEEFFFFFGHHIN                                 Q     UMWGEEEDDDCCCCCCCCCCCCBBBBBB
AAAABBCCCCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEFFFFFFGHIJKLOT                                     [JGFFEEEDDCCCCCCCCCCCCCBBBBB
AAAABCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEEFFFFFFGGHYV RQU                                     QMJHGGFEEEDDDCCCCCCCCCCCCCBBBB
AAABCCCCCCCCCCCCCCCCCDDDDDDDEEFJIHFFFFFFFFFFFFFFGGGGGGHIJN                                            JHHGFEEDDDDCCCCCCCCCCCCCBBB
AAABCCCCCCCCCCCDDDDDDDDDDEEEEFFHLKHHGGGGHHMJHGGGGGGHHHIKRR                                           UQ L HFEDDDDCCCCCCCCCCCCCCBB
AABCCCCCCCCDDDDDDDDDDDEEEEEEFFFHKQMRKNJIJLVS JJKIIIIIIJLR                                               YNHFEDDDDDCCCCCCCCCCCCCBB
AABCCCCCDDDDDDDDDDDDEEEEEEEFFGGHIJKOU  O O   PR LLJJJKL                                                OIHFFEDDDDDCCCCCCCCCCCCCCB
AACCCDDDDDDDDDDDDDEEEEEEEEEFGGGHIJMR              RMLMN                                                 NTFEEDDDDDDCCCCCCCCCCCCCB
AACCDDDDDDDDDDDDEEEEEEEEEFGGGHHKONSZ                QPR                                                NJGFEEDDDDDDCCCCCCCCCCCCCC
ABCDDDDDDDDDDDEEEEEFFFFFGIPJIIJKMQ                   VX                                                 HFFEEDDDDDDCCCCCCCCCCCCCC
ACDDDDDDDDDDEFFFFFFFGGGGHIKZOOPPS                                                                      HGFEEEDDDDDDCCCCCCCCCCCCCC
ADEEEEFFFGHIGGGGGGHHHHIJJLNY                                                                        TJHGFFEEEDDDDDDDCCCCCCCCCCCCC
A                                                                                                 PLJHGGFFEEEDDDDDDDCCCCCCCCCCCCC
ADEEEEFFFGHIGGGGGGHHHHIJJLNY                                                                        TJHGFFEEEDDDDDDDCCCCCCCCCCCCC
ACDDDDDDDDDDEFFFFFFFGGGGHIKZOOPPS                                                                      HGFEEEDDDDDDCCCCCCCCCCCCCC
ABCDDDDDDDDDDDEEEEEFFFFFGIPJIIJKMQ                   VX                                                 HFFEEDDDDDDCCCCCCCCCCCCCC
AACCDDDDDDDDDDDDEEEEEEEEEFGGGHHKONSZ                QPR                                                NJGFEEDDDDDDCCCCCCCCCCCCCC
AACCCDDDDDDDDDDDDDEEEEEEEEEFGGGHIJMR              RMLMN                                                 NTFEEDDDDDDCCCCCCCCCCCCCB
AABCCCCCDDDDDDDDDDDDEEEEEEEFFGGHIJKOU  O O   PR LLJJJKL                                                OIHFFEDDDDDCCCCCCCCCCCCCCB
AABCCCCCCCCDDDDDDDDDDDEEEEEEFFFHKQMRKNJIJLVS JJKIIIIIIJLR                                               YNHFEDDDDDCCCCCCCCCCCCCBB
AAABCCCCCCCCCCCDDDDDDDDDDEEEEFFHLKHHGGGGHHMJHGGGGGGHHHIKRR                                           UQ L HFEDDDDCCCCCCCCCCCCCCBB
AAABCCCCCCCCCCCCCCCCCDDDDDDDEEFJIHFFFFFFFFFFFFFFGGGGGGHIJN                                            JHHGFEEDDDDCCCCCCCCCCCCCBBB
AAAABCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEEFFFFFFGGHYV RQU                                     QMJHGGFEEEDDDCCCCCCCCCCCCCBBBB
AAAABBCCCCCCCCCCCCCCCCCCCCCCCCCDDDDEEEEEEEEEEEEEEEFFFFFFGHIJKLOT                                     [JGFFEEEDDCCCCCCCCCCCCCBBBBB
AAAAABBCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDEEEEEEEEEEEEFFFFFGHHIN                                 Q     UMWGEEEDDDCCCCCCCCCCCCBBBBBB
AAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDEEEEEEEEEFFFFGH O    TN S                       NKJKR LLQMNHEEDDDCCCCCCCCCCCCBBBBBBB
AAAAAABBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDEEEEEEFFGHK   MKJIJO  N R  X      YUSR PLV LHHHGGHIOJGFEDDDCCCCCCCCCCCCBBBBBBBB
AAAAAAABBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEEFGGHIIHHHHHIIIJKMR        VMKJIHHHGFFFFFFGSGEDDDDCCCCCCCCCCCCBBBBBBBBB
AAAAAAABBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEFFFFFFGGGGHIKP           KHHGGFFFFEEEEEEDDDDDCCCCCCCCCCCBBBBBBBBBBB
AAAAAAAABBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEFFFFFGGHJLZ         UKHGFFEEEEEEEEDDDDDCCCCCCCCCCCCBBBBBBBBBBBB
AAAAAAAAABBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGQPUVOTY   ZQL[MHFEEEEEEEDDDDDDDCCCCCCCCCCCBBBBBBBBBBBBBB
AAAAAAAAAABBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDDEEEEEEFFGHIJKS  X KHHGFEEEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBB
AAAAAAAAAAABBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEEFGGHHIKPPKIHGFFEEEDDDDDDDDDCCCCCCCCCCBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAABBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDDDEEEEEFFGHIMTKLZOGFEEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAABBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDDDEEEEFFFI KHGGGHGEDDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBB
AAAAAAAAAAAAAAABBBBBBBBBBBBBCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCDDDDDDDDDDEEEFGIIGFFEEEDDDDDDDDCCCCCCCCCBBBBBBBBBBBBBBBBBBBBBBBBBB
"#;
    assert_eq!(correct_output, interpreter.output_log);
}
