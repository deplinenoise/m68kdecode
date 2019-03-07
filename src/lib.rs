
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
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    ORITOCCR,
    ORITOSR,
    ORI,
    ANDITOCCR,
    ANDITOSR,
    ANDI,
    EORITOCCR,
    EORITOSR,
    EORI,
    ADDI,
    SUBI,
    LEA,
    MOVE,
    MOVEA,
    RTM,
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

    println!("decode_extended_ea: {:?}", idx);

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
        println!("indexer: {:?}", indexer);

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
        println!("  brief disp {} src_reg {:?}", disp, src_reg);
        let indexer =  if idx_is_a {
            Indexer::AR(address_reg(idx).unwrap(), scale)
        } else {
            Indexer::DR(data_reg(idx).unwrap(), scale)
        };

        let displacement = Displacement { 
            base_displacement: disp,
            outer_displacement: 0,
            indexer: indexer,
            indirection: NoIndirection,
        };

        match src_reg {
            None => Ok(PCDISP(displacement)),
            Some(r) => Ok(ARDISP(r, displacement)),
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

fn decode_ccr_sr_immediate_op(op: Operation, size: i32, extensions: &[u8]) -> Result<DecodedInstruction, DecodingError> {
    let mut offset = 0usize;
    Ok(DecodedInstruction {
        bytes_used: 4,
        instruction: Instruction {
            size: size,
            operation: op,
            operands: [
                match size {
                    1 => IMM8(pull_16(extensions, &mut offset)? as u8),
                    2 => IMM16(pull_16(extensions, &mut offset)?),
                    _ => return Err(NotImplemented),
                },
                Implied
            ],
        },
    })
}

fn decode_alu_imm(op: Operation, opword: u16, extensions: &[u8]) -> Result<DecodedInstruction, DecodingError> {
    let sz = get_bits(opword, 6, 7);
    let mut offset = 0usize;

    let imm = match sz {
        0b00 => IMM8(pull_16(extensions, &mut offset)? as u8),
        0b01 => IMM16(pull_16(extensions, &mut offset)?),
        0b10 => IMM32(pull_32(extensions, &mut offset)?),
        _ => return Err(BadSize)
    };

    let dst_reg = get_bits(opword, 0, 2);
    let dst_mod = get_bits(opword, 3, 5);
    let dst_op = decode_ea(dst_reg, dst_mod, &mut offset, extensions, 1 << sz)?;

    Ok(DecodedInstruction {
        bytes_used: 2 + offset as u32,
        instruction: Instruction {
            size: 1 << sz,
            operation: op,
            operands: [
                imm,
                dst_op,
            ],
        },
    })
}

fn decode_bitmap(opword: u16, extensions: &[u8]) -> Result<DecodedInstruction, DecodingError> {
    // Handle 1:1 matches first
    match opword {
        0b0000_0000_0011_1100 => return decode_ccr_sr_immediate_op(ORITOCCR, 1, extensions),
        0b0000_0000_0111_1100 => return decode_ccr_sr_immediate_op(ORITOSR, 2, extensions),
        0b0000_0010_0011_1100 => return decode_ccr_sr_immediate_op(ANDITOCCR, 1, extensions),
        0b0000_0010_0111_1100 => return decode_ccr_sr_immediate_op(ANDITOSR, 2, extensions),
        0b0000_1010_0011_1100 => return decode_ccr_sr_immediate_op(EORITOCCR, 1, extensions),
        0b0000_1010_0111_1100 => return decode_ccr_sr_immediate_op(EORITOSR, 2, extensions),
        _ => (),
    };

    // RTM uses illegal size of following instructions, so check it first
    if (opword & 0b1111_1111_1111_0000) == 0b0000_0110_1100_0000 {
        let da = opword & (1 << 3);
        let reg = get_bits(opword, 0, 2);
        return Ok(DecodedInstruction {
            bytes_used: 2,
            instruction: Instruction {
                size: 0,
                operation: RTM,
                operands: [
                    if da != 0 { AR(address_reg(reg)?) } else { DR(data_reg(reg)?) },
                    NoOperand,
                ],
            }
        });
    }

    match opword & 0b1111_1111_0000_0000 {
        0b0000_0000_0000_0000 => return decode_alu_imm(ORI, opword, extensions),
        0b0000_0010_0000_0000 => return decode_alu_imm(ANDI, opword, extensions),
        0b0000_1010_0000_0000 => return decode_alu_imm(EORI, opword, extensions),
        0b0000_0100_0000_0000 => return decode_alu_imm(SUBI, opword, extensions),
        0b0000_0110_0000_0000 => return decode_alu_imm(ADDI, opword, extensions),
        _ => (),
    };


    Err(NotImplemented)
}


fn decode_misc(opword: u16, extensions: &[u8]) -> Result<DecodedInstruction, DecodingError> {

    let mut offset = 0usize;

    if (opword & 0b0100_0001_1100_0000) == 0b0100_0001_1100_0000 {
        // Handle LEA

        let src_reg = get_bits(opword, 0, 2);
        let src_mod = get_bits(opword, 3, 5);
        let src_op = decode_ea(src_reg, src_mod, &mut offset, extensions, 4)?;

        let dst_reg = get_bits(opword, 9, 11);

        Ok(DecodedInstruction {
            bytes_used: 2 + offset as u32,
            instruction: Instruction {
                size: 4,
                operation: LEA,
                operands: [
                    src_op,
                    AR(address_reg(dst_reg)?),
                ],
            },
        })
    }
    else {
        Err(NotImplemented)
    }
}

pub fn decode_instruction(code_bytes: &[u8]) -> Result<DecodedInstruction, DecodingError> {

    let mut offset = 0usize;

    let opword = pull_16(code_bytes, &mut offset)?;

    match opword >> 12 {
        0b0000 => decode_bitmap(opword, &code_bytes[2..]),
        0b0001 => decode_move(opword, &code_bytes[2..], 1),
        0b0010 => decode_move(opword, &code_bytes[2..], 4),
        0b0011 => decode_move(opword, &code_bytes[2..], 2),
        0b0100 => decode_misc(opword, &code_bytes[2..]),
        _ => Err(NotImplemented),
    }
}

