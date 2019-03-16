use crate::*;
use codestream::*;
#[derive(Debug, PartialEq)]
pub enum Operation {
    ANDITOCCR,
    ANDITOSR,
    EORITOCCR,
    EORITOSR,
    ORITOCCR,
    ORITOSR,
    RTM,
    MOVEA,
    MOVE,
    MOVEP,
    BTST,
    BCHG,
    BCLR,
    BSET,
    CALLM,
    ADDI,
    SUBI,
    ANDI,
    ORI,
    CMP2,
    CHK2,
    EORI,
    CMPI,
    MOVES,
    BGND,
    ILLEGAL,
    NOP,
    RESET,
    RTD,
    RTE,
    RTR,
    RTS,
    STOP,
    TRAPV,
    SWAP,
    BKPT,
    EXTW,
    EXTL,
    EXTBL,
    LEA,
    LINK,
    UNLK,
    TRAP,
    DIVS,
    DIVSL,
    DIVSLL,
}
#[allow(non_snake_case)]
pub fn decode_instruction(code: &[u8]) -> Result<DecodedInstruction, DecodingError> {
    let mut cs = CodeStream::new(code);
    let w0 = cs.pull16();
    let sz;
    let src;
    let dst;
    if (w0 & 0b1111111111111111) == 0b0000001000111100 {
        sz = 1;
        src = cs.imm8();
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDITOCCR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000001001111100 {
        sz = 2;
        src = cs.imm16();
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDITOSR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000101000111100 {
        sz = 1;
        src = cs.imm8();
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EORITOCCR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000101001111100 {
        sz = 2;
        src = cs.imm16();
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EORITOSR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000000000111100 {
        sz = 1;
        src = cs.imm8();
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ORITOCCR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000000001111100 {
        sz = 2;
        src = cs.imm16();
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ORITOSR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111110000) == 0b0000011011000000 {
        let a = get_bits(w0, 3, 1);
        let r = get_bits(w0, 0, 3);
        sz = 0;
        src = cs.dar(a, r);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTM,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b0011000001000000 {
        let R = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = cs.ea(R, 0b001, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEA,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b0010000001000000 {
        let R = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.ea(r, m, 4);
        dst = cs.ea(R, 0b001, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEA,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000000000000) == 0b0001000000000000 {
        let R = get_bits(w0, 9, 3);
        let M = get_bits(w0, 6, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.ea(r, m, 1);
        dst = cs.ea(R, M, 1);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVE,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000000000000) == 0b0011000000000000 {
        let R = get_bits(w0, 9, 3);
        let M = get_bits(w0, 6, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = cs.ea(R, M, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVE,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000000000000) == 0b0010000000000000 {
        let R = get_bits(w0, 9, 3);
        let M = get_bits(w0, 6, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.ea(r, m, 4);
        dst = cs.ea(R, M, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVE,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000110111000) == 0b0000000100001000 {
        let d = get_bits(w0, 9, 3);
        let s = get_bits(w0, 6, 1);
        let a = get_bits(w0, 0, 3);
        sz = 1 << (s + 1);
        src = ARIND(cs.address_reg(a));
        dst = cs.data_reg_op(d);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEP,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000110111000) == 0b0000000100101000 {
        let d = get_bits(w0, 9, 3);
        let s = get_bits(w0, 6, 1);
        let a = get_bits(w0, 0, 3);
        sz = 1 << (s + 1);
        src = DR(cs.data_reg(d));
        dst = ARIND(cs.address_reg(a));
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEP,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b0000000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.data_reg_op(d);
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BTST,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b0000000101000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.data_reg_op(d);
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BCHG,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b0000000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.data_reg_op(d);
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BCLR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b0000000111000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.data_reg_op(d);
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BSET,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000100000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111111000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let n = get_bits(w1, 0, 9);
            cs.skip_words(1);
            sz = 1;
            src = IMM16(n);
            dst = cs.ea(r, m, 1);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BTST,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000100001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111111000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let n = get_bits(w1, 0, 9);
            cs.skip_words(1);
            sz = 1;
            src = IMM16(n);
            dst = cs.ea(r, m, 1);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BCHG,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000100010000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111111000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let n = get_bits(w1, 0, 9);
            cs.skip_words(1);
            sz = 1;
            src = IMM16(n);
            dst = cs.ea(r, m, 1);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BCLR,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000100011000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111111000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let n = get_bits(w1, 0, 9);
            cs.skip_words(1);
            sz = 1;
            src = IMM16(n);
            dst = cs.ea(r, m, 1);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BSET,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111110000) == 0b0000011011000000 {
        let d = get_bits(w0, 3, 1);
        let r = get_bits(w0, 0, 3);
        sz = 0;
        src = cs.dar(d, r);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTM,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000011011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 0;
        src = cs.imm8();
        dst = cs.ea(r, m, 0);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CALLM,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000011000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.imm8();
        dst = cs.ea(r, m, 1);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000011001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.imm16();
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000011010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.imm32();
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000010000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.imm8();
        dst = cs.ea(r, m, 1);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000010001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.imm16();
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000010010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.imm32();
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000001000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.imm8();
        dst = cs.ea(r, m, 1);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000001001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.imm16();
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000001010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.imm32();
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000000000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.imm8();
        dst = cs.ea(r, m, 1);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ORI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000000001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.imm16();
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ORI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000000010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.imm32();
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ORI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111100111000000) == 0b0000000011000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000000000000000 {
            let s = get_bits(w0, 9, 2);
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 1 << s;
            src = cs.ea(r, m, sz);
            dst = cs.dar(a, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: CMP2,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111100111000000) == 0b0000000011000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000100000000000 {
            let s = get_bits(w0, 9, 2);
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 1 << s;
            src = cs.ea(r, m, sz);
            dst = cs.dar(a, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: CHK2,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000101000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.imm8();
        dst = cs.ea(r, m, 1);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EORI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000101001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.imm16();
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EORI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000101010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.imm32();
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EORI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000110000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.imm8();
        dst = cs.ea(r, m, 1);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000110001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.imm16();
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000110010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.imm32();
        dst = cs.ea(r, m, 4);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPI,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0000111000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 1;
            dst = cs.dar(a, d);
            src = cs.ea(r, m, 1);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MOVES,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000111001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 2;
            dst = cs.dar(a, d);
            src = cs.ea(r, m, 2);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MOVES,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000111010000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 4;
            dst = cs.dar(a, d);
            src = cs.ea(r, m, 4);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MOVES,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000111000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000100000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 1;
            src = cs.dar(a, d);
            dst = cs.ea(r, m, 1);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MOVES,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000111001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000100000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 2;
            src = cs.dar(a, d);
            dst = cs.ea(r, m, 2);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MOVES,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0000111010000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b0000111111111111) == 0b0000100000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let a = get_bits(w1, 15, 1);
            let d = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.dar(a, d);
            dst = cs.ea(r, m, 4);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MOVES,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111111111) == 0b0100101011111010 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BGND,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100101011111100 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ILLEGAL,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110001 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NOP,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110000 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RESET,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110100 {
        sz = 0;
        src = cs.imm16();
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTD,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110011 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTE,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110111 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTR,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110101 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTS,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110010 {
        sz = 0;
        src = cs.imm16();
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: STOP,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110110 {
        sz = 0;
        src = NoOperand;
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TRAPV,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100001000000 {
        let r = get_bits(w0, 0, 3);
        sz = 0;
        src = cs.data_reg_op(r);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SWAP,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100001001000 {
        let n = get_bits(w0, 0, 3);
        sz = 0;
        src = IMM8(n as u8);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BKPT,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100010000000 {
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.data_reg_op(r);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXTW,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100011000000 {
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.data_reg_op(r);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXTL,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100111000000 {
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.data_reg_op(r);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXTBL,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b0100000111000000 {
        let n = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.ea(r, m, 4);
        dst = cs.address_reg_op(n);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LEA,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001010000 {
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.address_reg_op(r);
        dst = cs.imm16();
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LINK,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100000001000 {
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.address_reg_op(r);
        dst = cs.imm32();
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LINK,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001011000 {
        let r = get_bits(w0, 0, 3);
        sz = 0;
        src = cs.address_reg_op(r);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: UNLK,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111110000) == 0b0100111001000000 {
        let v = get_bits(w0, 0, 4);
        sz = 0;
        src = IMM8(v as u8);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TRAP,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000111000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = cs.data_reg_op(d);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: DIVS,
            operands: [src, dst],
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000110000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let q = get_bits(w1, 12, 3);
            let R = get_bits(w1, 0, 3);
            if R != q {
                cs.skip_words(1);
                sz = 4;
                src = cs.ea(r, m, 4);
                dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
                return cs.check_overflow(Instruction {
                    size: sz,
                    operation: DIVSL,
                    operands: [src, dst],
                });
            }
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000110000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let q = get_bits(w1, 12, 3);
            let R = get_bits(w1, 0, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.ea(r, m, 4);
            dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
            return cs.check_overflow(Instruction {
                size: sz,
                operation: DIVSL,
                operands: [src, dst],
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000100000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let q = get_bits(w1, 12, 3);
            let R = get_bits(w1, 0, 3);
            if R != q {
                cs.skip_words(1);
                sz = 4;
                src = cs.ea(r, m, 4);
                dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
                return cs.check_overflow(Instruction {
                    size: sz,
                    operation: DIVSLL,
                    operands: [src, dst],
                });
            }
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000100000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let q = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.ea(r, m, 4);
            dst = cs.data_reg_op(q);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: DIVSL,
                operands: [src, dst],
            });
        }
    }
    return Err(NotImplemented);
}
