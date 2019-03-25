//! An instruction decoder for M680x0 family instructions.

mod decoder;
mod codestream;

pub use decoder::Operation;

/// One of the 8 data registers.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum DataRegister { 
    D0, D1, D2, D3, D4, D5, D6, D7,
}

/// One of the 8 address registers.
///
/// Note that A7 means `USP` in user mode and `SSP`/`ISP` in supervisor mode.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum AddressRegister { 
    A0, A1, A2, A3, A4, A5, A6, A7,
}

/// One of the 8 floating point registers.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum FloatingRegister { 
    FP0, FP1, FP2, FP3, FP4, FP5, FP6, FP7,
}

/// Indicates how memory indexing is to be performed for 68020+ double memory operands.
#[derive(Debug, PartialEq)]
pub enum MemoryIndirection {
    /// No memory indirection.
    NoIndirection,
    /// Regular memory indirection.
    Indirect,
    /// Memory pre-indexed indirection (indexer applies to inner array).
    IndirectPreIndexed,
    /// Memory post-indexed indirection (indexer applies to outer array).
    IndirectPostIndexed,
}

/// Indicates how indexing is to be performed.
#[derive(Debug, PartialEq)]
pub enum Indexer {
    /// Indexing suppressed
    NoIndexer,
    /// Index using data register
    DR(DataRegister, u8),
    /// Index using address register
    AR(AddressRegister, u8),
}

/// Indicates how a memory operand effective address is to be computed using displacement.
#[derive(Debug, PartialEq)]
pub struct Displacement {
    /// Base displacement. This is always sign-extended to 32-bit by the instruction decoder.
    pub base_displacement: i32,
    /// Outer displacement (only used for 68020+ double indirection modes)
    pub outer_displacement: i32,
    /// Indicates how indexing is to be performed.
    pub indexer: Indexer,
    /// Indicates what type of memory indirection is to be performed.
    pub indirection: MemoryIndirection
}

/// Describes one of the two operands in an instruction
#[derive(Debug, PartialEq)]
pub enum Operand {
    /// No operand present, used for instructions that have fewer than two operands.
    NoOperand,
    /// The operand is implied, for example when the CCR is being moved to or from.
    ///
    /// When the operand is implied, the instruction documentation must be consulted to see what
    /// data is being affected.
    Implied,
    /// An immediate 8-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    IMM8(u8),
    /// An immediate 16-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    IMM16(u16),
    /// An immediate 32-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    IMM32(u32),
    /// An absolute address, given as a sign-extended 16-bit value.
    ABS16(i16),
    /// An absolute address, given as a full 32-bit value.
    ABS32(u32),
    /// A data register
    DR(DataRegister),
    /// An address register
    AR(AddressRegister),
    /// A floating point register
    FR(FloatingRegister),
    /// A vanilla indrection without displacement, e.g. `(a0)`
    ARIND(AddressRegister),
    /// Address register indirect with post-increment, e.g. `(a0)+`
    ARINC(AddressRegister),
    /// Address register indirect with pre-decrement, e.g. `-(a0)`
    ARDEC(AddressRegister),
    /// Address register indirect with displacement, e.g. `123(pc,d0)`
    ARDISP(AddressRegister, Displacement),
    /// Program counter indirect with displacement, e.g. `123(pc,d0)`
    PCDISP(u8, Displacement),
    /// Just a displacement (skipping the base register), e.g. `123(d0)`
    DISP(Displacement),
    /// A data register pair, used for 64-bit multiply/divide results.
    DPAIR(DataRegister, DataRegister),
    /// A floating point register pair, used for `FSINCOS`. First register receives the sine, the
    /// other the cosine.
    FPAIR(FloatingRegister, FloatingRegister),
    /// A register bitmask for `MOVEM` or `FMOVEM`. The order of registers is reversed depending on
    /// whether the address register is pre-decremented or post-incremented.
    REGLIST(u16),
}

/// Describes one leg of a 68020+ bitfield specification
#[derive(Debug, PartialEq)]
pub enum BitfieldData {
    /// The offset or width is static.
    STATIC(u8),
    /// The offset or width is dynamic and specified via a data register.
    DYNAMIC(DataRegister)
}

/// A CPU condition code from the CCR
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConditionCode {
    /// Always True        
    CC_T  = 0b0000,
    /// Always False       
    CC_F  = 0b0001,
    /// High               
    CC_HI = 0b0010,
    /// Low or Same        
    CC_LS = 0b0011,
    /// Carry Clear        
    CC_CC = 0b0100,
    /// Carry Set          
    CC_CS = 0b0101,
    /// Not Equal          
    CC_NE = 0b0110,
    /// Equal              
    CC_EQ = 0b0111,
    /// Overflow Clear     
    CC_VC = 0b1000,
    /// Overflow Set       
    CC_VS = 0b1001,
    /// Plus               
    CC_PL = 0b1010,
    /// Negative           
    CC_MI = 0b1011,
    /// Greater or Equal   
    CC_GE = 0b1100,
    /// Less               
    CC_LT = 0b1101,
    /// Greater            
    CC_GT = 0b1110,
    /// Less or Equal      
    CC_LE = 0b1111,
}

/// A FPU condition code
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FPConditionCode {
    /// False
    FPCC_F    = 0b000000,
    /// Equal
    FPCC_EQ   = 0b000001,
    /// Ordered Greater Than
    FPCC_OGT  = 0b000010,
    /// Ordered Greater Than or Equal
    FPCC_OGE  = 0b000011,
    /// Ordered Less Than
    FPCC_OLT  = 0b000100,
    /// Ordered Less Than or Equal
    FPCC_OLE  = 0b000101,
    /// Ordered Greater Than or Less Than
    FPCC_OGL  = 0b000110,
    /// Ordered
    FPCC_OR   = 0b000111,
    /// Unordered
    FPCC_UN   = 0b001000,
    /// Unordered or Equal
    FPCC_UEQ  = 0b001001,
    /// Unordered or Greater Than
    FPCC_UGT  = 0b001010,
    /// Unordered or Greater Than or Equal
    FPCC_UGE  = 0b001011,
    /// Unordered or Less Than
    FPCC_ULT  = 0b001100,
    /// Unordered or Less Than or Equal
    FPCC_ULE  = 0b001101,
    /// Not Equal
    FPCC_NE   = 0b001110,
    /// True
    FPCC_T    = 0b001111,
    /// Signaling False
    FPCC_SF   = 0b010000,
    /// Signaling Equal
    FPCC_SEQ  = 0b010001,
    /// Greater Than
    FPCC_GT   = 0b010010,
    /// Greater Than or Equal
    FPCC_GE   = 0b010011,
    /// Less Than
    FPCC_LT   = 0b010100,
    /// Less Than or Equal
    FPCC_LE   = 0b010101,
    /// Greater Than or Less Than
    FPCC_GL   = 0b010110,
    /// Greater Than or Less Than or Equal
    FPCC_GLE  = 0b010111,
    /// Not (Greater Than or Less Than or Equal)
    FPCC_NGLE = 0b011000,
    /// Not (Greater Than or Less Than)
    FPCC_NGL  = 0b011001,
    /// Not (Less Than or Equal)
    FPCC_NLE  = 0b011010,
    /// Not (Less Than)
    FPCC_NLT  = 0b011011,
    /// Not (Greater Than or Equal)
    FPCC_NGE  = 0b011100,
    /// Not (Greater Than)
    FPCC_NGT  = 0b011101,
    /// Signaling Not Equal
    FPCC_SNE  = 0b011110,
    /// Signaling True
    FPCC_ST   = 0b011111,
}

/// Indicates the floating point format in memory for a FPU operation
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FPFormat {
    /// The memory operand is a 32-bit integer.
    FPF_LONG_INT,
    /// The memory operand is a IEEE 32-bit single.
    FPF_SINGLE,
    /// The memory operand is a 68k extended-precision real (10 bytes).
    FPF_EXTENDED_REAL,
    /// The memory operand is a 68k packed decimal real (BCD encoded) (12 bytes).
    /// For `FMOVE`, this also includes a static _K-factor_ to be applied to the result.
    FPF_PACKED_DECIMAL_REAL_STATIC(i32), // Includes fmove K-factor
    /// The memory operand is a 68k packed decimal real (BCD encoded) (12 bytes).
    /// For `FMOVE`, this also includes a dynamic _K-factor_ to be applied to the result.
    FPF_PACKED_DECIMAL_REAL_DYNAMIC(DataRegister), // Includes fmove K-factor
    /// The memory operand is a 16-bit integer.
    FPF_WORD_INT,
    /// The memory operand is a IEEE 64-bit double.
    FPF_DOUBLE,
    /// The memory operand is an 8-bit integer.
    FPF_BYTE_INT
}

/// Additional attributes for an instruction that don't fit anywhere else.
#[derive(Debug, PartialEq)]
pub enum InstructionExtra {
    /// No additional attributes available.
    NoExtra,
    /// For 68020+ bitfield instructions, specifies the bitfield offset and width of the EA.
    Bitfield(BitfieldData, BitfieldData),
    /// For instructions involving a CPU condition code (e.g. `DBcc`), specifies the condition code
    /// tested.
    Condition(ConditionCode),
    /// For instructions involving a FPU condition code (e.g. `FDBcc`), specifies the condition code
    /// tested.
    FPCondition(FPConditionCode),
    /// Specifies the pack adjustment for BCD packing.
    PackAdjustment(u16),
    /// Specifies the float format of a memory operand for FPU instructions that use a memory EA.
    FloatFormat(FPFormat),
}

/// Represents an instruction.
#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// The size of any data movement (the number of bytes read or written).
    pub size: i32,
    /// The instruction itself.
    pub operation: Operation,
    /// The two operands involved, source and destination.
    pub operands: [Operand;2],
    /// Any additional data required to make sense of the operation.
    pub extra: InstructionExtra,
}

/// Represents the result of decoding an instruction from a byte stream.
#[derive(Debug,PartialEq)]
pub struct DecodedInstruction {
    /// The number of bytes that were consumed decoding the instruction.
    pub bytes_used: u32,
    /// The decoded instruction.
    pub instruction: Instruction,
}

/// Enumerates errors that can happen while decoding instructions.
#[derive(Debug,PartialEq,Copy,Clone)]
pub enum DecodingError {
    /// The instruction is not implemented in the decoder.
    NotImplemented,
    /// The code stream doesn't contain enough data to decode the instruction.
    OutOfSpace,
    /// An illegal register was specified in the instruction encoding.
    BadRegister,
    /// An illegal size was specified in the instruction encoding.
    BadSize,
    /// A reserved case was hit in the instruction encoding.
    Reserved,
}
/// Convenience function (for tests and decoding cases) to generate a simple displacement value.
pub fn simple_disp(disp: i32) -> Displacement {
    Displacement {
        base_displacement: disp,
        outer_displacement: 0,
        indexer: Indexer::NoIndexer,
        indirection: MemoryIndirection::NoIndirection,
    }
}

/// Convenience function (for tests and decoding cases) to generate a displacement with a data
/// register and scale.
pub fn dr_disp_scale(dr: DataRegister, disp: i32, scale: u8) -> Displacement {
    Displacement {
        base_displacement: disp,
        outer_displacement: 0,
        indexer: Indexer::DR(dr, scale),
        indirection: MemoryIndirection::NoIndirection,
    }
}

/// Convenience function (for tests and decoding cases) to generate a displacement with a data
/// register.
pub fn dr_disp(dr: DataRegister, disp: i32) -> Displacement {
    dr_disp_scale(dr, disp, 0)
}

/// Attempt to decode a single M68000 instruction starting at `code[0]`
pub fn decode_instruction(code: &[u8]) -> Result<DecodedInstruction, DecodingError> {
    decoder::decode_instruction_generated(code)
}
