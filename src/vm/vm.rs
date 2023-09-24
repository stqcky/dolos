// use log::debug;
//
// use crate::vm::disassembler::get_instruction_representation;
//
// use super::intrinsics::{Chunk, Instruction};
//
// #[derive(Debug)]
// pub struct VM {
//     stack: Vec<f32>,
//     ip: usize,
//     sp: usize,
// }
//
// impl VM {
//     fn push(&mut self, value: f32) {
//         self.sp += 1;
//         self.stack.push(value);
//     }
//
//     fn pop(&mut self) -> f32 {
//         self.sp -= 1;
//         let value = self.stack.pop();
//
//         match value {
//             Some(value) => value,
//             None => panic!("stack underflow"),
//         }
//     }
//
//     pub fn new() -> Self {
//         Self {
//             stack: vec![],
//             ip: 0,
//             sp: 1,
//         }
//     }
//
//     fn print_stack(&self) {
//         debug!("{:#?}", self.stack);
//     }
//
//     pub fn execute(&mut self, chunk: Chunk) {
//         loop {
//             let instruction = match chunk.get(self.ip) {
//                 Some(ins) => ins,
//                 None => panic!("invalid ip"),
//             };
//
//             debug!("executing {}", get_instruction_representation(instruction));
//             self.print_stack();
//
//             match instruction {
//                 Instruction::Return => {
//                     self.return_instruction();
//                     return;
//                 }
//                 Instruction::Add => self.add_instruction(),
//                 Instruction::Subtract => self.sub_instruction(),
//                 Instruction::Multiply => self.mul_instruction(),
//                 Instruction::Divide => self.div_instruction(),
//                 Instruction::Load(value) => self.load_instruction(*value),
//             };
//
//             self.ip += 1;
//         }
//     }
//
//     fn return_instruction(&mut self) {
//         println!("return {}", self.pop());
//     }
//
//     fn add_instruction(&mut self) {
//         let b = self.pop();
//         let a = self.pop();
//
//         self.push(a + b);
//     }
//
//     fn sub_instruction(&mut self) {
//         let b = self.pop();
//         let a = self.pop();
//
//         self.push(a - b);
//     }
//
//     fn mul_instruction(&mut self) {
//         let b = self.pop();
//         let a = self.pop();
//
//         self.push(a * b);
//     }
//
//     fn div_instruction(&mut self) {
//         let b = self.pop();
//         let a = self.pop();
//
//         self.push(a / b);
//     }
//
//     fn load_instruction(&mut self, value: f32) {
//         self.push(value);
//     }
// }
