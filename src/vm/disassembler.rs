// fn get_instruction_size(instruction: &Instruction) -> usize {
//     match instruction {
//         Instruction::Load(_) => 5,
//         _ => 1
//     }
// }
//
// pub fn get_instruction_representation(instruction: &Instruction) -> String {
//     match instruction {
//         Instruction::Return => "ret".to_string(),
//         Instruction::Add => "add".to_string(),
//         Instruction::Subtract => "sub".to_string(),
//         Instruction::Multiply => "mul".to_string(),
//         Instruction::Divide => "div".to_string(),
//         Instruction::Load(value) => format!("load {value}")
//     }
// }
//
// pub fn disassemble(chunk: Chunk) {
//     let mut address = 0;
//
//     for instruction in chunk.iter() {
//         println!("{} \t {}", address, get_instruction_representation(instruction));
//
//         address += get_instruction_size(instruction);
//     }
// }
