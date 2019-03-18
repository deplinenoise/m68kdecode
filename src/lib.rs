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
    PCDISP(u8, Displacement),
    DISP(Displacement),
    DPAIR(DataRegister, DataRegister),
    REGLIST(u16),
}

#[derive(Debug, PartialEq)]
pub enum BitfieldData {
    STATIC(u8),
    DYNAMIC(DataRegister)
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConditionCode {
    CC_T  = 0b0000,         // Always True
    CC_F  = 0b0001,         // Always False
    CC_HI = 0b0010,         // High
    CC_LS = 0b0011,         // Low or Same
    CC_CC = 0b0100,         // Carry Clear
    CC_CS = 0b0101,         // Carry Set
    CC_NE = 0b0110,         // Not Equal
    CC_EQ = 0b0111,         // Equal
    CC_VC = 0b1000,         // Overflow Clear
    CC_VS = 0b1001,         // Overflow Set
    CC_PL = 0b1010,         // Plus
    CC_MI = 0b1011,         // Negative
    CC_GE = 0b1100,         // Greater or Equal
    CC_LT = 0b1101,         // Less
    CC_GT = 0b1110,         // Greater
    CC_LE = 0b1111,         // Less or Equal
}

#[derive(Debug, PartialEq)]
pub enum InstructionExtra {
    NoExtra,
    Bitfield(BitfieldData, BitfieldData),
    Condition(ConditionCode),
    PackAdjustment(u16),
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub size: i32,
    pub operation: Operation,
    pub operands: [Operand;2],
    pub extra: InstructionExtra,
}

pub use Operation::*;
pub use Operand::*;
pub use InstructionExtra::*;
pub use BitfieldData::*;
pub use ConditionCode::*;

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
