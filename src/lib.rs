mod decoder;
mod codestream;

pub use decoder::*;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum DataRegister { 
    D0, D1, D2, D3, D4, D5, D6, D7,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum AddressRegister { 
    A0, A1, A2, A3, A4, A5, A6, A7,
}

pub use DataRegister::*;
pub use AddressRegister::*;

#[derive(Debug, PartialEq)]
pub enum MemoryIndirection {
    NoIndirection,
    Indirect,
    IndirectPreIndexed,
    IndirectPostIndexed,
}

pub use MemoryIndirection::*;

#[derive(Debug, PartialEq)]
pub enum Indexer {
    NoIndexer,
    DR(DataRegister, u8),
    AR(AddressRegister, u8),
}

#[derive(Debug, PartialEq)]
pub struct Displacement {
    pub base_displacement: i32,
    pub outer_displacement: i32,
    pub indexer: Indexer,
    pub indirection: MemoryIndirection
}

pub fn simple_disp(disp: i32) -> Displacement {
    Displacement {
        base_displacement: disp,
        outer_displacement: 0,
        indexer: Indexer::NoIndexer,
        indirection: NoIndirection,
    }
}

pub fn dr_disp_scale(dr: DataRegister, disp: i32, scale: u8) -> Displacement {
    Displacement {
        base_displacement: disp,
        outer_displacement: 0,
        indexer: Indexer::DR(dr, scale),
        indirection: NoIndirection,
    }
}

pub fn dr_disp(dr: DataRegister, disp: i32) -> Displacement {
    dr_disp_scale(dr, disp, 0)
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    NoOperand,
    Implied,
    IMM8(u8),
    IMM16(u16),
    IMM32(u32),
    ABS16(i16),
    ABS32(u32),
    DR(DataRegister),
    AR(AddressRegister),
    ARIND(AddressRegister),
    ARINC(AddressRegister),
    ARDEC(AddressRegister),
    ARDISP(AddressRegister, Displacement),
    PCDISP(Displacement),
    DISP(Displacement),
    DPAIR(DataRegister, DataRegister),
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub size: i32,
    pub operation: Operation,
    pub operands: [Operand;2],
}

pub use Operation::*;
pub use Operand::*;

#[derive(Debug,PartialEq)]
pub struct DecodedInstruction {
    pub bytes_used: u32,
    pub instruction: Instruction,
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum DecodingError {
    NotImplemented,
    OutOfSpace,
    BadRegister,
    BadSize,
    Reserved,
}
pub use DecodingError::*;

// Test support functions
pub fn test_decoding_result_ok(bytes: &[u8], expected: Instruction, asm: &[&str]) {
    let r = decode_instruction(&bytes);

    match r {
        Err(e) => {
            println!("Expected: {:?}", expected);
            println!("Got: {:?}", e);
            for l in asm {
                println!("{}", l);
            }
            assert!(false);
        },
        Ok(DecodedInstruction { bytes_used, instruction }) => {
            assert!(bytes_used == bytes.len() as u32);
            if instruction != expected {
                println!("Expected: {:?}", expected);
                println!("Got: {:?}", instruction);
                for l in asm {
                    println!("{}", l);
                }
                assert!(false);
            }
        },
    }
}

pub fn test_decoding_result_err(bytes: &[u8], expected: DecodingError, asm: &[&str]) {
    let r = decode_instruction(&bytes);
    match r {
        Err(e) => {
            if e == expected { return; }
        }
        _ => (),
    };

    println!("Expected: {:?}", expected);
    println!("Got: {:?}", r);
    for l in asm {
        println!("{}", l);
    }
    assert!(false);
}
