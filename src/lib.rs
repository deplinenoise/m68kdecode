
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
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    LEA,
    MOVE,
    MOVEA,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub size: i32,
    pub operation: Operation,
    pub operands: [Operand;2],
}

pub use Operation::*;
pub use Operand::*;

pub struct DecodedInstruction {
    pub bytes_used: u32,
    pub instruction: Instruction,
}

#[derive(Debug)]
pub enum DecodingError {
    NotImplemented,
    OutOfSpace,
    BadRegister,
    BadSize,
    Reserved,
}
pub use DecodingError::*;

fn pull_16(bytes: &[u8], offset: &mut usize) -> Result<u16, DecodingError> {
    if bytes.len() < *offset + 2 {
        return Err(OutOfSpace);
    }
    let hi = bytes[*offset] as u16;
    let lo = bytes[*offset + 1] as u16;
    *offset += 2;
    Ok((hi << 8) | lo)
}

fn pull_32(bytes: &[u8], offset: &mut usize) -> Result<u32, DecodingError> {
    if bytes.len() < *offset + 4 {
        return Err(OutOfSpace);
    }
    let b0 = bytes[*offset] as u32;
    let b1 = bytes[*offset + 1] as u32;
    let b2 = bytes[*offset + 2] as u32;
    let b3 = bytes[*offset + 3] as u32;
    *offset += 4;
    Ok((b0 << 24) | (b1 << 16) | (b2 << 8) | b3)
}

fn get_bits(word: u16, first: i32, last: i32) -> u16 {
    let s = word >> first;
    s & (1 << (last - first + 1)) - 1
}

fn data_reg(r: u16) -> Result<DataRegister, DecodingError> {
    match r {
        0 => Ok(D0), 1 => Ok(D1), 2 => Ok(D2), 3 => Ok(D3),
        4 => Ok(D4), 5 => Ok(D5), 6 => Ok(D6), 7 => Ok(D7),
        _ => Err(BadRegister)
    }
}

fn address_reg(r: u16) -> Result<AddressRegister, DecodingError> {
    match r {
        0 => Ok(A0), 1 => Ok(A1), 2 => Ok(A2), 3 => Ok(A3),
        4 => Ok(A4), 5 => Ok(A5), 6 => Ok(A6), 7 => Ok(A7),
        _ => Err(BadRegister)
    }
}

fn decode_extended_ea(src_reg: Option<AddressRegister>, offset: &mut usize, extensions: &[u8]) -> Result<Operand, DecodingError> {
    let ext = pull_16(extensions, offset)?;
    let scale = get_bits(ext, 9, 10) as u8;
    let idx = get_bits(ext, 12, 14);
    let idx_is_a = get_bits(ext, 15, 15) == 1;

    if 0 != (ext & (1 << 8)) {
        // Handle full extension word.
        let bd = get_bits(ext, 4, 5);
        let od = get_bits(ext, 1, 0);
        let disp = match bd {
            0 => Err(Reserved),
            1 => Ok(0u32),
            2 => Ok(pull_16(extensions, offset)? as i16 as i32 as u32),
            3 => Ok(pull_32(extensions, offset)?),
            _ => Err(NotImplemented),
        }?;
        let odisp = match od {
            0 => Ok(0u32),
            1 => Ok(0u32),
            2 => Ok(pull_16(extensions, offset)? as i16 as i32 as u32),
            3 => Ok(pull_32(extensions, offset)?),
            _ => Err(NotImplemented),
        }?;

        let suppress_base = get_bits(ext, 7, 7) == 1;
        let suppress_indexer = get_bits(ext, 6, 6) == 1;

        let indirection_mode = match suppress_indexer {
            false => {
                match get_bits(ext, 0, 2) {
                    0b000 => Ok(NoIndirection),
                    0b001 => Ok(IndirectPreIndexed),
                    0b010 => Ok(IndirectPreIndexed),
                    0b011 => Ok(IndirectPreIndexed),
                    0b100 => Err(Reserved),
                    0b101 => Ok(IndirectPostIndexed),
                    0b110 => Ok(IndirectPostIndexed),
                    0b111 => Ok(IndirectPostIndexed),
                    _ => Err(NotImplemented),
                }?
            },
            true => {
                match get_bits(ext, 0, 2) {
                    0b000 => Ok(NoIndirection),
                    0b001 => Ok(Indirect),
                    0b010 => Ok(Indirect),
                    0b011 => Ok(Indirect),
                    _ => Err(Reserved),
                }?
            },
        };

        let indexer = match suppress_indexer {
            true => Indexer::NoIndexer,
            false => if idx_is_a {
                Indexer::AR(address_reg(idx).unwrap(), scale)
            } else {
                Indexer::DR(data_reg(idx).unwrap(), scale)
            },
        };

        if suppress_base {
            Ok(DISP(Displacement {
                base_displacement: disp as i32,
                outer_displacement: odisp as i32,
                indexer: indexer,
                indirection: indirection_mode,
            }))
        } else {
            match src_reg {
                None => Ok(PCDISP(Displacement {
                    base_displacement: disp as i32,
                    outer_displacement: odisp as i32,
                    indexer: indexer,
                    indirection: indirection_mode,
                })),
                Some(reg) => Ok(ARDISP(reg, Displacement {
                    base_displacement: disp as i32,
                    outer_displacement: odisp as i32,
                    indexer: indexer,
                    indirection: indirection_mode,
                })),
            }
        }
    } else {
        // Handle brief extension word
        let disp = ((ext & 0xff) as i8) as i32;
        let indexer =  if idx_is_a {
            Indexer::AR(address_reg(idx).unwrap(), scale)
        } else {
            Indexer::DR(data_reg(idx).unwrap(), scale)
        };

        match src_reg {
            None => Ok(PCDISP(simple_disp(disp))),
            Some(r) => Ok(ARDISP(r, Displacement { 
                base_displacement: disp,
                outer_displacement: 0,
                indexer: indexer,
                indirection: NoIndirection,
            })),
        }
    }
}

fn decode_ea(src_reg: u16, src_mod: u16, offset: &mut usize, extensions: &[u8], size: i32) -> Result<Operand, DecodingError> {
    match src_mod {
        0b000 => Ok(DR(data_reg(src_reg)?)),
        0b001 => Ok(AR(address_reg(src_reg)?)),
        0b010 => Ok(ARIND(address_reg(src_reg)?)),
        0b011 => Ok(ARINC(address_reg(src_reg)?)),
        0b100 => Ok(ARDEC(address_reg(src_reg)?)),
        0b101 => {
            let disp = pull_16(extensions, offset)?;
            Ok(ARDISP(address_reg(src_reg)?, simple_disp(disp as i16 as i32)))
        },
        0b110 => decode_extended_ea(Some(address_reg(src_reg)?), offset, extensions),
        0b111 => match src_reg {
            0b000 => Ok(ABS16(pull_16(extensions, offset)? as i16)),
            0b001 => Ok(ABS32(pull_32(extensions, offset)?)),
            0b010 => Ok(PCDISP(simple_disp(pull_16(extensions, offset)? as i16 as i32))),
            0b011 => decode_extended_ea(None, offset, extensions),
            0b100 => match size {
                1 => Ok(IMM8(pull_16(extensions, offset)? as u8)),
                2 => Ok(IMM16(pull_16(extensions, offset)?)),
                4 => Ok(IMM32(pull_32(extensions, offset)?)),
                _ => Err(BadSize),
            },
            _ => Err(NotImplemented),
        },
        _ => Err(NotImplemented)
    }
}

fn decode_move(opword: u16, extensions: &[u8], size: i32) -> Result<DecodedInstruction, DecodingError> {

    let mut offset = 0usize;

    let src_reg = get_bits(opword, 0, 2);
    let src_mod = get_bits(opword, 3, 5);
    let src_op = decode_ea(src_reg, src_mod, &mut offset, extensions, size)?;

    let dst_reg = get_bits(opword, 9, 11);
    let dst_mod = get_bits(opword, 6, 8);
    let dst_op = decode_ea(dst_reg, dst_mod, &mut offset, extensions, size)?;

    Ok(DecodedInstruction {
        bytes_used: 2 + offset as u32,
        instruction: Instruction {
            size: size,
            operation: if dst_mod == 0b001 { MOVEA } else { MOVE },
            operands: [
                src_op,
                dst_op,
            ],
        },
    })
}

pub fn decode_instruction(code_bytes: &[u8]) -> Result<DecodedInstruction, DecodingError> {

    let mut offset = 0usize;

    let opword = pull_16(code_bytes, &mut offset)?;

    match opword >> 12 {
        0b0001 => decode_move(opword, &code_bytes[2..], 1),
        0b0010 => decode_move(opword, &code_bytes[2..], 4),
        0b0011 => decode_move(opword, &code_bytes[2..], 2),
        _ => Err(NotImplemented),
    }
}

