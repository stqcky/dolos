pub enum Register {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
}

enum Value {
    Immediate(f32),
    Register(Register),
}

pub enum Instruction {
    Return,
    Push(Value),
    Pop(Register),
    Add(Register, Register, Value),
    Subtract(Register, Register, Value),
    Multiply(Register, Register, Value),
    Divide(Register, Register, Value),
    Mov(Register, Value),
}

pub type Chunk = Vec<Instruction>;
