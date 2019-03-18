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
    DIVU,
    DIVUL,
    DIVULL,
    JMP,
    JSR,
    MULS,
    MULU,
    NBCD,
    MOVEFROMSR,
    MOVETOSR,
    MOVETOUSP,
    MOVEFROMUSP,
    MOVEFROMCCR,
    MOVETOCCR,
    PEA,
    TAS,
    MOVEM,
    CLR,
    NEG,
    NEGX,
    NOT,
    TST,
    CHK,
    BFCHG,
    BFCLR,
    BFEXTS,
    BFEXTU,
    BFFFO,
    BFINS,
    BFSET,
    BFTST,
    DBCC,
    ADDQ,
}
#[allow(non_snake_case)]
pub fn decode_instruction(code: &[u8]) -> Result<DecodedInstruction, DecodingError> {
    let mut cs = CodeStream::new(code);
    let w0 = cs.pull16();
    let sz;
    let src;
    let dst;
    let mut extra = NoExtra;
    if (w0 & 0b1111111111111111) == 0b0000001000111100 {
        sz = 1;
        src = cs.imm8();
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDITOCCR,
            operands: [src, dst],
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
                extra: extra,
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
                extra: extra,
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
                extra: extra,
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
                extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
                extra: extra,
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
                extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
                extra: extra,
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
                extra: extra,
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
                extra: extra,
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
                extra: extra,
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
                extra: extra,
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
                extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
            extra: extra,
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
                    extra: extra,
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
                extra: extra,
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
                    extra: extra,
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
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111000111000000) == 0b1000000011000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = cs.data_reg_op(d);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: DIVU,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000010000000000 {
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
                    operation: DIVUL,
                    operands: [src, dst],
                    extra: extra,
                });
            }
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000010000000000 {
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
                operation: DIVUL,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000000000000000 {
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
                    operation: DIVULL,
                    operands: [src, dst],
                    extra: extra,
                });
            }
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110001000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let q = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.ea(r, m, 4);
            dst = cs.data_reg_op(q);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: DIVUL,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100111011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 0;
        src = cs.ea(r, m, 0);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: JMP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100111010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 0;
        src = cs.ea(r, m, 0);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: JSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000111000000 {
        let p = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = cs.data_reg_op(p);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MULS,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100110000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000100000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let l = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.ea(r, m, 4);
            dst = cs.data_reg_op(l);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MULS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000110000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let l = get_bits(w1, 12, 3);
            let h = get_bits(w1, 0, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.ea(r, m, 4);
            dst = DPAIR(cs.data_reg(l), cs.data_reg(h));
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MULS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111000111000000) == 0b1100000011000000 {
        let p = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = cs.data_reg_op(p);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MULU,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100110000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let l = get_bits(w1, 12, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.ea(r, m, 4);
            dst = cs.data_reg_op(l);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MULU,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100110000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000111111111000) == 0b0000010000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let l = get_bits(w1, 12, 3);
            let h = get_bits(w1, 0, 3);
            cs.skip_words(1);
            sz = 4;
            src = cs.ea(r, m, 4);
            dst = DPAIR(cs.data_reg(l), cs.data_reg(h));
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MULU,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b0100100000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.ea(r, m, 1);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NBCD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100000011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = Implied;
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEFROMSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100011011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVETOSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001100000 {
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.address_reg_op(r);
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVETOUSP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001101000 {
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = Implied;
        dst = cs.address_reg_op(r);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEFROMUSP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100001011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = Implied;
        dst = cs.ea(r, m, 2);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEFROMCCR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100010011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, 2);
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVETOCCR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100100001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.ea(r, m, 4);
        dst = Implied;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: PEA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100101011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.ea(r, m, 1);
        dst = NoOperand;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TAS,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111110000000) == 0b0100100010000000 {
        let s = get_bits(w0, 6, 1);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2 << s;
        src = REGLIST(cs.pull16());
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEM,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111110000000) == 0b0100110010000000 {
        let s = get_bits(w0, 6, 1);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2 << s;
        dst = REGLIST(cs.pull16());
        src = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEM,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100001000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CLR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100001001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CLR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100001010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CLR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100010000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NEG,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100010001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NEG,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100010010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NEG,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100000000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NEGX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100000001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NEGX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100000010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NEGX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100011000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NOT,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100011001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NOT,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100011010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NOT,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100101000000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TST,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100101001000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TST,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b0100101010000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = Implied;
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TST,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0100000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.ea(r, m, sz);
        dst = cs.data_reg_op(d);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CHK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0100000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.ea(r, m, sz);
        dst = cs.data_reg_op(d);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CHK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110101011000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = NoOperand;
            dst = cs.ea(r, m, 0);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFCHG,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1110110011000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = NoOperand;
            dst = cs.ea(r, m, 0);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFCLR,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1110101111000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let d = get_bits(w1, 12, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = cs.ea(r, m, 0);
            dst = cs.data_reg_op(d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFEXTS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1110100111000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let d = get_bits(w1, 12, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = cs.ea(r, m, 0);
            dst = cs.data_reg_op(d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFEXTU,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1110110111000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let d = get_bits(w1, 12, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = cs.ea(r, m, 0);
            dst = cs.data_reg_op(d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFFFO,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1110111111000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1000000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let d = get_bits(w1, 12, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = cs.data_reg_op(d);
            dst = cs.ea(r, m, 0);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFINS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1110111011000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = NoOperand;
            dst = cs.ea(r, m, 0);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFSET,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1110100011000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111000000000000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let O = get_bits(w1, 11, 1);
            let o = get_bits(w1, 6, 5);
            let W = get_bits(w1, 5, 1);
            let w = get_bits(w1, 0, 5);
            cs.skip_words(1);
            extra = cs.bitfield(O, o, W, w);
            sz = 0;
            src = NoOperand;
            dst = cs.ea(r, m, 0);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFTST,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111000011111000) == 0b0101000011001000 {
        let c = get_bits(w0, 8, 4);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.data_reg_op(r);
        dst = PCDISP(2, simple_disp(cs.pull16() as i16 as i32));
        extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: DBCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0101000000000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 1;
        src = cs.quick_const(d);
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0101000001000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 2;
        src = cs.quick_const(d);
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0101000010000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        sz = 4;
        src = cs.quick_const(d);
        dst = cs.ea(r, m, sz);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
