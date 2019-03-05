#[derive(Debug, PartialEq)]
#[repr(u8)]
enum Register {
    Nothing,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    CCR,
    SR,
    PC,
}

use Register::*;

#[derive(Debug, PartialEq)]
enum MemoryIndirection {
    NoIndirection,
    Indirect,
    IndirectPreIndexed,
    IndirectPostIndexed,
}

use MemoryIndirection::*;

#[derive(Debug, PartialEq)]
struct MemoryOperand {
    base_register      : Register,
    base_displacement  : i32,
    indexer            : Register,
    outer_displacement : i32,
    scale_log2         : u8,
    incdec             : i8,
    indirection        : MemoryIndirection,
}

impl MemoryOperand {
    fn address_register_indirect(r: Register, incdec: i32) -> MemoryOperand {
        MemoryOperand {
            base_register: r,
            base_displacement: 0,
            indexer: Nothing,
            outer_displacement: 0,
            scale_log2: 0,
            incdec: incdec as i8,
            indirection: NoIndirection,
        }
    }

    fn absolute(address: u32) -> MemoryOperand {
        MemoryOperand {
            base_register: Nothing,
            base_displacement: address as i32,
            indexer: Nothing,
            outer_displacement: 0,
            scale_log2: 0,
            incdec: 0,
            indirection: NoIndirection,
        }
    }

    fn pcrelative(disp: i32) -> MemoryOperand {
        MemoryOperand {
            base_register: PC,
            base_displacement: disp,
            indexer: Nothing,
            outer_displacement: 0,
            scale_log2: 0,
            incdec: 0,
            indirection: NoIndirection,
        }
    }

    fn address_register_indirect_disp(r: Register, disp: i32) -> MemoryOperand {
        MemoryOperand {
            base_register: r,
            base_displacement: disp,
            indexer: Nothing,
            outer_displacement: 0,
            scale_log2: 0,
            incdec: 0,
            indirection: NoIndirection,
        }
    }

    fn address_register_indirect_index_disp(r: Register, index: Register, disp: i32) -> MemoryOperand {
        MemoryOperand {
            base_register: r,
            base_displacement: disp,
            indexer: index,
            outer_displacement: 0,
            scale_log2: 0,
            incdec: 0,
            indirection: NoIndirection,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operand {
    RegisterDirect(Register),
    Memory(MemoryOperand),
    Immediate8(u8),
    Immediate16(u16),
    Immediate32(u32),
}

#[derive(Debug, PartialEq)]
enum Operation {
    LEA,
    MOVE,
    MOVEA,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    size: i32,
    operation: Operation,
    operands: [Operand;2],
}

use Operation::*;
use Operand::*;

struct DecodedInstruction {
    bytes_used: u32,
    instruction: Instruction,
}

#[derive(Debug)]
enum DecodingError {
    NotImplemented,
    OutOfSpace,
    BadRegister,
    BadSize,
    Reserved,
}
use DecodingError::*;

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

fn data_reg(r: u16) -> Result<Register, DecodingError> {
    match r {
        0 => Ok(D0), 1 => Ok(D1), 2 => Ok(D2), 3 => Ok(D3),
        4 => Ok(D4), 5 => Ok(D5), 6 => Ok(D6), 7 => Ok(D7),
        _ => Err(BadRegister)
    }
}

fn address_reg(r: u16) -> Result<Register, DecodingError> {
    match r {
        0 => Ok(A0), 1 => Ok(A1), 2 => Ok(A2), 3 => Ok(A3),
        4 => Ok(A4), 5 => Ok(A5), 6 => Ok(A6), 7 => Ok(A7),
        _ => Err(BadRegister)
    }
}

fn decode_extended_ea(src_reg: Register, offset: &mut usize, extensions: &[u8]) -> Result<Operand, DecodingError> {
    let ext = pull_16(extensions, offset)?;
    let scale = get_bits(ext, 9, 10) as u8;
    let idx = get_bits(ext, 12, 14);
    if 0 != (ext & (1 << 8)) {
        // Handle full extension word.
        let bd = get_bits(ext, 4, 5);
        let od = get_bits(ext, 1, 0);
        let disp = match bd {
            0 => Err(Reserved),
            1 => Ok(0u32),
            2 => Ok(pull_16(extensions, offset)? as u32),
            3 => Ok(pull_32(extensions, offset)?),
            _ => Err(NotImplemented),
        }?;
        let odisp = match od {
            0 => Ok(0u32),
            1 => Ok(0u32),
            2 => Ok(pull_16(extensions, offset)? as u32),
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

        Ok(Memory(MemoryOperand {
            base_register: if suppress_base { Nothing } else { src_reg },
            base_displacement: disp as i32,
            indexer: if suppress_indexer { Nothing } else { data_reg(idx)? },
            outer_displacement: odisp as i32,
            scale_log2: scale,
            incdec: 0,
            indirection: indirection_mode,
        }))
    } else {
        // Handle brief extension word
        let disp = ((ext & 0xff) as i8) as i32;

        Ok(Memory(MemoryOperand {
            base_register: src_reg,
            base_displacement: disp,
            indexer: data_reg(idx)?,
            outer_displacement: 0,
            scale_log2: scale,
            incdec: 0,
            indirection: NoIndirection,
        }))
    }
}

fn decode_ea(src_reg: u16, src_mod: u16, offset: &mut usize, extensions: &[u8], size: i32) -> Result<Operand, DecodingError> {
    match src_mod {
        0b000 => Ok(RegisterDirect(data_reg(src_reg)?)),
        0b001 => Ok(RegisterDirect(address_reg(src_reg)?)),
        0b010 => Ok(Memory(MemoryOperand::address_register_indirect(address_reg(src_reg)?, 0))),
        0b011 => Ok(Memory(MemoryOperand::address_register_indirect(address_reg(src_reg)?, 1))),
        0b100 => Ok(Memory(MemoryOperand::address_register_indirect(address_reg(src_reg)?, -1))),
        0b101 => {
            let disp = pull_16(extensions, offset)?;
            Ok(Memory(MemoryOperand::address_register_indirect_disp(address_reg(src_reg)?, disp as i16 as i32)))
        },
        0b110 => decode_extended_ea(address_reg(src_reg)?, offset, extensions),
        0b111 => match src_reg {
            0b000 => Ok(Memory(MemoryOperand::absolute(pull_16(extensions, offset)? as u32))),
            0b001 => Ok(Memory(MemoryOperand::absolute(pull_32(extensions, offset)?))),
            0b010 => Ok(Memory(MemoryOperand::pcrelative(pull_16(extensions, offset)? as i16 as i32))),
            0b011 => decode_extended_ea(PC, offset, extensions),
            0b100 => match size {
                1 => Ok(Immediate8(pull_16(extensions, offset)? as u8)),
                2 => Ok(Immediate16(pull_16(extensions, offset)?)),
                4 => Ok(Immediate32(pull_32(extensions, offset)?)),
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

fn decode_instruction(code_bytes: &[u8]) -> Result<DecodedInstruction, DecodingError> {

    let mut offset = 0usize;

    let opword = pull_16(code_bytes, &mut offset)?;

    match opword >> 12 {
        0b0001 => decode_move(opword, &code_bytes[2..], 1),
        0b0010 => decode_move(opword, &code_bytes[2..], 4),
        0b0011 => decode_move(opword, &code_bytes[2..], 2),
        _ => Err(NotImplemented),
    }
}

fn main() {
    let i = Instruction {
        size: 4,
        operation: MOVE,
        operands: [
            RegisterDirect(D0),
            Memory(MemoryOperand::address_register_indirect(A3, 0)),
        ]
    };

    println!("Hello, world!");
    println!("{:?}", i);
}

#[cfg(test)]
mod tests {

    use crate::*;

    fn do_test(bytes: &[u8], expected: Instruction) {
        let r = decode_instruction(&bytes).unwrap();
        assert!(r.bytes_used == bytes.len() as u32);
        if (r.instruction != expected) {
            println!("Expected: {:?}", expected);
            println!("Got: {:?}", r.instruction);
            assert!(false);
        }
    }

    //#[test]
    fn test_lea_l() {
        do_test(&[0x43, 0xf0, 0x00, 0x7b], Instruction {
            size: 4,
            operation: LEA,
            operands: [
                Memory(MemoryOperand::address_register_indirect_index_disp(A0, D0, 123)),
                RegisterDirect(A1),
            ],
        });
    }

    #[test]
    fn test_move() {
        do_test(&[0x26, 0x30, 0x00, 0x7b], Instruction {
            size: 4,
            operation: MOVE,
            operands: [
                Memory(MemoryOperand::address_register_indirect_index_disp(A0, D0, 123)),
                RegisterDirect(D3),
            ],
        });
    }

    #[test]
    fn test_movea_scale() {
        do_test(&[0x22, 0x70, 0x02, 0x7b], Instruction {
            size: 4,
            operation: MOVEA,
            operands: [
                Memory(MemoryOperand {
                    base_register: A0,
                    base_displacement: 123,
                    indexer: D0,
                    outer_displacement: 0,
                    scale_log2: 1,
                    incdec: 0,
                    indirection: NoIndirection,
                }),
                RegisterDirect(A1),
            ],
        });
    }

    #[test]
    fn test_move_b_imm_dr() {
        do_test(&[0x1e, 0x3c, 0x00, 0xff], Instruction {
            size: 1, operation: MOVE, operands: [ Immediate8(0xff), RegisterDirect(D7), ],
        });
    }

    #[test]
    fn test_move_w_imm_dr() {
        do_test(&[0x3a, 0x3c, 0x12, 0x34], Instruction {
            size: 2, operation: MOVE, operands: [ Immediate16(0x1234), RegisterDirect(D5), ],
        });
    }

    #[test]
    fn test_move_l_imm_dr() {
        do_test(&[0x28, 0x3c, 0x12, 0x34, 0x56, 0x78], Instruction {
            size: 4, operation: MOVE, operands: [ Immediate32(0x12345678), RegisterDirect(D4), ],
        });
    }

    #[test]
    fn test_move_l_ar_predec() {
        do_test(&[0x25, 0x01], Instruction {
            size: 4, operation: MOVE, operands: [ RegisterDirect(D1), Memory(MemoryOperand::address_register_indirect(A2, -1)), ],
        });
    }

    #[test]
    fn test_move_l_ar_postinc() {
        do_test(&[0x24, 0xc1], Instruction {
            size: 4, operation: MOVE, operands: [ RegisterDirect(D1), Memory(MemoryOperand::address_register_indirect(A2, 1)), ],
        });
    }

    #[test]
    fn test_move_l_pre_post() {
        do_test(&[0x24, 0xe4], Instruction {
            size: 4, operation: MOVE, operands: [ 
                Memory(MemoryOperand::address_register_indirect(A4, -1)),
                Memory(MemoryOperand::address_register_indirect(A2, 1)),
            ],
        });
    }

    #[test]
    fn test_move_l_abs16() {
        do_test(&[0x20, 0x78, 0x00, 0x04], Instruction {
            size: 4, operation: MOVEA, operands: [ 
                Memory(MemoryOperand::absolute(4)),
                RegisterDirect(A0),
            ],
        });
    }

    #[test]
    fn test_move_l_abs32() {
        do_test(&[0x20, 0x79, 0x11, 0x22, 0x33, 0x44], Instruction {
            size: 4, operation: MOVEA, operands: [ 
                Memory(MemoryOperand::absolute(0x11223344)),
                RegisterDirect(A0),
            ],
        });
    }

    // 31 bc 12 34 01 a0 00 7b
    #[test]
    fn test_move_imm_020_dx_base() {
        do_test(&[0x31, 0xbc, 0x12, 0x34, 0x01, 0xa0, 0x00, 0x7b], Instruction {
            size: 2, operation: MOVE, operands: [ 
                Immediate16(0x1234),
                Memory(MemoryOperand {
                    base_register: Nothing,
                    base_displacement: 123,
                    indexer: D0,
                    outer_displacement: 0,
                    scale_log2: 0,
                    incdec: 0,
                    indirection: NoIndirection,
                }),
            ],
        });
    }

    #[test]
    fn test_move_neg_pc_dx() {
        do_test(&[0x36, 0x3a, 0xff, 0xf8], Instruction {
            size: 2, operation: MOVE, operands: [ 
                Memory(MemoryOperand {
                    base_register: PC,
                    base_displacement: -8,
                    indexer: Nothing,
                    outer_displacement: 0,
                    scale_log2: 0,
                    incdec: 0,
                    indirection: NoIndirection,
                }),
                RegisterDirect(D3),
            ],
        });
    }

    // 3e 3b 26 f8

    #[test]
    fn test_move_neg_pc_dx_scale() {
        do_test(&[0x3e, 0x3b, 0x26, 0xf8], Instruction {
            size: 2, operation: MOVE, operands: [ 
                Memory(MemoryOperand {
                    base_register: PC,
                    base_displacement: -8,
                    indexer: D2,
                    outer_displacement: 0,
                    scale_log2: 3,
                    incdec: 0,
                    indirection: NoIndirection,
                }),
                RegisterDirect(D7),
            ],
        });
    }
    #[test]
    fn test_move_w_memi_memi() {
        do_test(&[0x35, 0xb1, 0x24, 0x7b, 0x33, 0x20, 0x26, 0x94], Instruction {
            size: 2,
            operation: MOVE,
            operands: [
                Memory(MemoryOperand {
                    base_register: A1,
                    base_displacement: 123,
                    indexer: D2,
                    outer_displacement: 0,
                    scale_log2: 2,
                    incdec: 0,
                    indirection: NoIndirection,
                }),
                Memory(MemoryOperand {
                    base_register: A2,
                    base_displacement: 9876,
                    indexer: D3,
                    outer_displacement: 0,
                    scale_log2: 1,
                    incdec: 0,
                    indirection: NoIndirection,
                })
            ],
        });
    }


}
