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
    MOVEP,
    BTST,
    BCHG,
    BCLR,
    BSET,
    RTM,
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
    MOVE,
    MOVEA,
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
    DIVSL,
    DIVSLL,
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
    DBCC,
    ADDQ,
    SUBQ,
    TRAPCC,
    SCC,
    BRA,
    BSR,
    BCC,
    MOVEQ,
    PACK,
    UNPK,
    SBCD,
    DIVS,
    DIVU,
    OR,
    SUBX,
    SUB,
    CMPA,
    CMPM,
    CMP,
    EOR,
    ABCD,
    EXG,
    AND,
    ADDX,
    ADD,
    BFCHG,
    BFCLR,
    BFEXTS,
    BFEXTU,
    BFFFO,
    BFINS,
    BFSET,
    BFTST,
    ASL,
    ASR,
    LSL,
    LSR,
    ROXL,
    ROXR,
    ROL,
    ROR,
    FABS,
    FSABS,
    FDABS,
    FACOS,
    FADD,
    FSADD,
    FDADD,
    FASIN,
    FATAN,
    FATANH,
    FBCC,
    FCMP,
    FCOS,
    FCOSH,
    FDBCC,
    FSIN,
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0000(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111111111111111) == 0b0000001000111100 {
        let sz = 1;
        let src = cs.imm8();
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDITOCCR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000001001111100 {
        let sz = 2;
        let src = cs.imm16();
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ANDITOSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000101000111100 {
        let sz = 1;
        let src = cs.imm8();
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EORITOCCR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000101001111100 {
        let sz = 2;
        let src = cs.imm16();
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EORITOSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000000000111100 {
        let sz = 1;
        let src = cs.imm8();
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ORITOCCR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0000000001111100 {
        let sz = 2;
        let src = cs.imm16();
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ORITOSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000110111000) == 0b0000000100001000 {
        let d = get_bits(w0, 9, 3);
        let s = get_bits(w0, 6, 1);
        let a = get_bits(w0, 0, 3);
        let sz = 1 << (s + 1);
        let src = ARIND(cs.address_reg(a));
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
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
        let sz = 1 << (s + 1);
        let src = DR(cs.data_reg(d));
        let dst = ARIND(cs.address_reg(a));
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
            let sz = 1;
            let src = IMM16(n);
            let dst = cs.ea(r, m, 1);
            let extra = NoExtra;
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
            let sz = 1;
            let src = IMM16(n);
            let dst = cs.ea(r, m, 1);
            let extra = NoExtra;
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
            let sz = 1;
            let src = IMM16(n);
            let dst = cs.ea(r, m, 1);
            let extra = NoExtra;
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
            let sz = 1;
            let src = IMM16(n);
            let dst = cs.ea(r, m, 1);
            let extra = NoExtra;
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
        let sz = 0;
        let src = cs.dar(d, r);
        let dst = NoOperand;
        let extra = NoExtra;
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
        let sz = 0;
        let src = cs.imm8();
        let dst = cs.ea(r, m, 0);
        let extra = NoExtra;
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
        let sz = 1;
        let src = cs.imm8();
        let dst = cs.ea(r, m, 1);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.imm16();
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.imm32();
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
        let sz = 1;
        let src = cs.imm8();
        let dst = cs.ea(r, m, 1);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.imm16();
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.imm32();
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
        let sz = 1;
        let src = cs.imm8();
        let dst = cs.ea(r, m, 1);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.imm16();
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.imm32();
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
        let sz = 1;
        let src = cs.imm8();
        let dst = cs.ea(r, m, 1);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.imm16();
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.imm32();
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
            let sz = 1 << s;
            let src = cs.ea(r, m, sz);
            let dst = cs.dar(a, d);
            let extra = NoExtra;
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
            let sz = 1 << s;
            let src = cs.ea(r, m, sz);
            let dst = cs.dar(a, d);
            let extra = NoExtra;
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
        let sz = 1;
        let src = cs.imm8();
        let dst = cs.ea(r, m, 1);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.imm16();
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.imm32();
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
        let sz = 1;
        let src = cs.imm8();
        let dst = cs.ea(r, m, 1);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.imm16();
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.imm32();
        let dst = cs.ea(r, m, 4);
        let extra = NoExtra;
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
            let sz = 1;
            let dst = cs.dar(a, d);
            let src = cs.ea(r, m, 1);
            let extra = NoExtra;
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
            let sz = 2;
            let dst = cs.dar(a, d);
            let src = cs.ea(r, m, 2);
            let extra = NoExtra;
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
            let sz = 4;
            let dst = cs.dar(a, d);
            let src = cs.ea(r, m, 4);
            let extra = NoExtra;
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
            let sz = 1;
            let src = cs.dar(a, d);
            let dst = cs.ea(r, m, 1);
            let extra = NoExtra;
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
            let sz = 2;
            let src = cs.dar(a, d);
            let dst = cs.ea(r, m, 2);
            let extra = NoExtra;
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
            let sz = 4;
            let src = cs.dar(a, d);
            let dst = cs.ea(r, m, 4);
            let extra = NoExtra;
            return cs.check_overflow(Instruction {
                size: sz,
                operation: MOVES,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111110000) == 0b0000011011000000 {
        let a = get_bits(w0, 3, 1);
        let r = get_bits(w0, 0, 3);
        let sz = 0;
        let src = cs.dar(a, r);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTM,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0001(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000000000000) == 0b0001000000000000 {
        let R = get_bits(w0, 9, 3);
        let M = get_bits(w0, 6, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.ea(r, m, 1);
        let dst = cs.ea(R, M, 1);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVE,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0010(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000111000000) == 0b0010000001000000 {
        let R = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, 4);
        let dst = cs.ea(R, 0b001, 4);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000000000000) == 0b0010000000000000 {
        let R = get_bits(w0, 9, 3);
        let M = get_bits(w0, 6, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, 4);
        let dst = cs.ea(R, M, 4);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVE,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0011(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000111000000) == 0b0011000001000000 {
        let R = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = cs.ea(R, 0b001, 2);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000000000000) == 0b0011000000000000 {
        let R = get_bits(w0, 9, 3);
        let M = get_bits(w0, 6, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = cs.ea(R, M, 2);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVE,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0100(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111111111111111) == 0b0100101011111010 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BGND,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100101011111100 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ILLEGAL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110001 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: NOP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110000 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RESET,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110100 {
        let sz = 0;
        let src = cs.imm16();
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110011 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTE,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110111 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110101 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: RTS,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110010 {
        let sz = 0;
        let src = cs.imm16();
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: STOP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0100111001110110 {
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TRAPV,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100001000000 {
        let r = get_bits(w0, 0, 3);
        let sz = 0;
        let src = cs.data_reg_op(r);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SWAP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100001001000 {
        let n = get_bits(w0, 0, 3);
        let sz = 0;
        let src = IMM8(n as u8);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BKPT,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100010000000 {
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(r);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXTW,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100011000000 {
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(r);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXTL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100111000000 {
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(r);
        let dst = NoOperand;
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.ea(r, m, 4);
        let dst = cs.address_reg_op(n);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LEA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001010000 {
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.address_reg_op(r);
        let dst = cs.imm16();
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LINK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100100000001000 {
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.address_reg_op(r);
        let dst = cs.imm32();
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LINK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001011000 {
        let r = get_bits(w0, 0, 3);
        let sz = 0;
        let src = cs.address_reg_op(r);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: UNLK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111110000) == 0b0100111001000000 {
        let v = get_bits(w0, 0, 4);
        let sz = 0;
        let src = IMM8(v as u8);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TRAP,
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
                let sz = 4;
                let src = cs.ea(r, m, 4);
                let dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
                let extra = NoExtra;
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
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
            let extra = NoExtra;
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
                let sz = 4;
                let src = cs.ea(r, m, 4);
                let dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
                let extra = NoExtra;
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
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = cs.data_reg_op(q);
            let extra = NoExtra;
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
        if (w1 & 0b1000111111111000) == 0b0000010000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let q = get_bits(w1, 12, 3);
            let R = get_bits(w1, 0, 3);
            if R != q {
                cs.skip_words(1);
                let sz = 4;
                let src = cs.ea(r, m, 4);
                let dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
                let extra = NoExtra;
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
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
            let extra = NoExtra;
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
                let sz = 4;
                let src = cs.ea(r, m, 4);
                let dst = DPAIR(cs.data_reg(q), cs.data_reg(R));
                let extra = NoExtra;
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
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = cs.data_reg_op(q);
            let extra = NoExtra;
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
        let sz = 0;
        let src = cs.ea(r, m, 0);
        let dst = NoOperand;
        let extra = NoExtra;
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
        let sz = 0;
        let src = cs.ea(r, m, 0);
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: JSR,
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
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = cs.data_reg_op(l);
            let extra = NoExtra;
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
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = DPAIR(cs.data_reg(l), cs.data_reg(h));
            let extra = NoExtra;
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
        if (w1 & 0b1000111111111000) == 0b0000000000000000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let l = get_bits(w1, 12, 3);
            cs.skip_words(1);
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = cs.data_reg_op(l);
            let extra = NoExtra;
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
            let sz = 4;
            let src = cs.ea(r, m, 4);
            let dst = DPAIR(cs.data_reg(l), cs.data_reg(h));
            let extra = NoExtra;
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
        let sz = 1;
        let src = cs.ea(r, m, 1);
        let dst = NoOperand;
        let extra = NoExtra;
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
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVETOSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001100000 {
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.address_reg_op(r);
        let dst = Implied;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVETOUSP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111000) == 0b0100111001101000 {
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = Implied;
        let dst = cs.address_reg_op(r);
        let extra = NoExtra;
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
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, 2);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = Implied;
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.ea(r, m, 4);
        let dst = Implied;
        let extra = NoExtra;
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
        let sz = 1;
        let src = cs.ea(r, m, 1);
        let dst = NoOperand;
        let extra = NoExtra;
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
        let sz = 2 << s;
        let src = REGLIST(cs.pull16());
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2 << s;
        let dst = REGLIST(cs.pull16());
        let src = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 1;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 4;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 1;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 4;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 1;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 4;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 1;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 4;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 1;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 4;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CHK,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0101(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000011111000) == 0b0101000011001000 {
        let c = get_bits(w0, 8, 4);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(r);
        let dst = PCDISP(2, simple_disp(cs.pull16() as i16 as i32));
        let extra = cs.cc(c);
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
        let sz = 1;
        let src = cs.quick_const(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 2;
        let src = cs.quick_const(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
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
        let sz = 4;
        let src = cs.quick_const(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0101000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.quick_const(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0101000101000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.quick_const(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b0101000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.quick_const(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000011111111) == 0b0101000011111100 {
        let c = get_bits(w0, 8, 4);
        let sz = 0;
        let src = NoOperand;
        let dst = NoOperand;
        let extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TRAPCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000011111111) == 0b0101000011111010 {
        let c = get_bits(w0, 8, 4);
        let sz = 2;
        let src = cs.imm16();
        let dst = NoOperand;
        let extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TRAPCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000011111111) == 0b0101000011111011 {
        let c = get_bits(w0, 8, 4);
        let sz = 4;
        let src = cs.imm32();
        let dst = NoOperand;
        let extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: TRAPCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000011000000) == 0b0101000011000000 {
        let c = get_bits(w0, 8, 4);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = Implied;
        let dst = cs.ea(r, m, 1);
        let extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0110(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111111111111111) == 0b0110000000000000 {
        let sz = 2;
        let src = PCDISP(2, simple_disp(cs.pull16() as i16 as i32));
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BRA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0110000011111111 {
        let sz = 4;
        let src = PCDISP(2, simple_disp(cs.pull32() as i32));
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BRA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111100000000) == 0b0110000000000000 {
        let d = get_bits(w0, 0, 8);
        let sz = 1;
        let src = PCDISP(2, simple_disp(d as i8 as i32));
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BRA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0110000100000000 {
        let sz = 2;
        let src = PCDISP(2, simple_disp(cs.pull16() as i16 as i32));
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111111111) == 0b0110000111111111 {
        let sz = 4;
        let src = PCDISP(2, simple_disp(cs.pull32() as i32));
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111100000000) == 0b0110000100000000 {
        let d = get_bits(w0, 0, 8);
        let sz = 1;
        let src = PCDISP(2, simple_disp(d as i8 as i32));
        let dst = NoOperand;
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000011111111) == 0b0110000000000000 {
        let c = get_bits(w0, 8, 4);
        let sz = 2;
        let src = PCDISP(2, simple_disp(cs.pull16() as i16 as i32));
        let dst = NoOperand;
        let extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000011111111) == 0b0110000011111111 {
        let c = get_bits(w0, 8, 4);
        let sz = 4;
        let src = PCDISP(2, simple_disp(cs.pull32() as i32));
        let dst = NoOperand;
        let extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000000000000) == 0b0110000000000000 {
        let c = get_bits(w0, 8, 4);
        let d = get_bits(w0, 0, 8);
        let sz = 1;
        let src = PCDISP(2, simple_disp(d as i8 as i32));
        let dst = NoOperand;
        let extra = cs.cc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: BCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_0111(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000100000000) == 0b0111000000000000 {
        let r = get_bits(w0, 9, 3);
        let n = get_bits(w0, 0, 8);
        let sz = 4;
        let src = IMM8(n as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MOVEQ,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_1000(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000111111000) == 0b1000000101000000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 0;
        let src = cs.data_reg_op(x);
        let dst = cs.data_reg_op(y);
        let extra = PackAdjustment(cs.pull16());
        return cs.check_overflow(Instruction {
            size: sz,
            operation: PACK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1000000101001000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 0;
        let src = ARDEC(cs.address_reg(x));
        let dst = ARDEC(cs.address_reg(y));
        let extra = PackAdjustment(cs.pull16());
        return cs.check_overflow(Instruction {
            size: sz,
            operation: PACK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1000000110000000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 0;
        let src = cs.data_reg_op(x);
        let dst = cs.data_reg_op(y);
        let extra = PackAdjustment(cs.pull16());
        return cs.check_overflow(Instruction {
            size: sz,
            operation: UNPK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1000000110001000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 0;
        let src = ARDEC(cs.address_reg(x));
        let dst = ARDEC(cs.address_reg(y));
        let extra = PackAdjustment(cs.pull16());
        return cs.check_overflow(Instruction {
            size: sz,
            operation: UNPK,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1000000100000000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(x);
        let dst = cs.data_reg_op(y);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SBCD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1000000100001000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 1;
        let src = ARDEC(cs.address_reg(x));
        let dst = ARDEC(cs.address_reg(y));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SBCD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000111000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: DIVS,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000011000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: DIVU,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000000000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: OR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000001000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: OR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000010000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: OR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: OR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000101000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: OR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1000000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: OR,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_1001(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000111111000) == 0b1001000100000000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(y);
        let dst = cs.data_reg_op(x);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1001000101000000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(y);
        let dst = cs.data_reg_op(x);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1001000110000000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(y);
        let dst = cs.data_reg_op(x);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1001000100001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 1;
        let src = ARDEC(cs.address_reg(y));
        let dst = ARDEC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1001000101001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 2;
        let src = ARDEC(cs.address_reg(y));
        let dst = ARDEC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1001000110001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = ARDEC(cs.address_reg(y));
        let dst = ARDEC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUBX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1001000000000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUB,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1001000001000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUB,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1001000010000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUB,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1001000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUB,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1001000101000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUB,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1001000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: SUB,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_1011(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000111000000) == 0b1011000011000000 {
        let a = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, sz);
        let dst = cs.address_reg_op(a);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1011000111000000 {
        let a = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, sz);
        let dst = cs.address_reg_op(a);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPA,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1011000100001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 1;
        let src = ARINC(cs.address_reg(y));
        let dst = ARINC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPM,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1011000101001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 2;
        let src = ARINC(cs.address_reg(y));
        let dst = ARINC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPM,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1011000110001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = ARINC(cs.address_reg(y));
        let dst = ARINC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMPM,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1011000000000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1011000001000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1011000010000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: CMP,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1011000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EOR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1011000101000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EOR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1011000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EOR,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_1100(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000111111000) == 0b1100000100000000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(x);
        let dst = cs.data_reg_op(y);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ABCD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1100000100001000 {
        let y = get_bits(w0, 9, 3);
        let x = get_bits(w0, 0, 3);
        let sz = 1;
        let src = ARDEC(cs.address_reg(x));
        let dst = ARDEC(cs.address_reg(y));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ABCD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000011000000 {
        let p = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = cs.data_reg_op(p);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MULU,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000111000000 {
        let p = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, 2);
        let dst = cs.data_reg_op(p);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: MULS,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1100000101000000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(x);
        let dst = cs.data_reg_op(y);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXG,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1100000101001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.address_reg_op(x);
        let dst = cs.address_reg_op(y);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXG,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1100000110001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(x);
        let dst = cs.address_reg_op(y);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: EXG,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000000000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: AND,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000001000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: AND,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000010000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: AND,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: AND,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000101000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: AND,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1100000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: AND,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_1101(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111000111111000) == 0b1101000100000000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(y);
        let dst = cs.data_reg_op(x);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1101000101000000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(y);
        let dst = cs.data_reg_op(x);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1101000110000000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(y);
        let dst = cs.data_reg_op(x);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1101000100001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 1;
        let src = ARDEC(cs.address_reg(y));
        let dst = ARDEC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1101000101001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 2;
        let src = ARDEC(cs.address_reg(y));
        let dst = ARDEC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1101000110001000 {
        let x = get_bits(w0, 9, 3);
        let y = get_bits(w0, 0, 3);
        let sz = 4;
        let src = ARDEC(cs.address_reg(y));
        let dst = ARDEC(cs.address_reg(x));
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADDX,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1101000000000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1101000001000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1101000010000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.ea(r, m, sz);
        let dst = cs.data_reg_op(d);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1101000100000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1101000101000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADD,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111000000) == 0b1101000110000000 {
        let d = get_bits(w0, 9, 3);
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(d);
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ADD,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_1110(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = NoOperand;
            let dst = cs.ea(r, m, 0);
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = NoOperand;
            let dst = cs.ea(r, m, 0);
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = cs.ea(r, m, 0);
            let dst = cs.data_reg_op(d);
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = cs.ea(r, m, 0);
            let dst = cs.data_reg_op(d);
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = cs.ea(r, m, 0);
            let dst = cs.data_reg_op(d);
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = cs.data_reg_op(d);
            let dst = cs.ea(r, m, 0);
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = NoOperand;
            let dst = cs.ea(r, m, 0);
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
            let extra = cs.bitfield(O, o, W, w);
            let sz = 0;
            let src = NoOperand;
            let dst = cs.ea(r, m, 0);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: BFTST,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111000111111000) == 0b1110000100000000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101000000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110000000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000000000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001000000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010000000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000100100000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101100000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110100000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000100000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001100000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010100000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000100001000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101001000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110001000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000001000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001001000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010001000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000100101000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101101000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110101000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000101000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001101000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010101000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000100010000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101010000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110010000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000010000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001010000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010010000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000100110000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101110000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110110000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000110000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001110000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010110000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000100011000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101011000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110011000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000011000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001011000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010011000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = IMM8(c as u8);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000100111000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000101111000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000110111000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000000111000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 1;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000001111000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111000111111000) == 0b1110000010111000 {
        let c = get_bits(w0, 9, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 4;
        let src = cs.data_reg_op(c);
        let dst = cs.data_reg_op(r);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110000111000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110000011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ASR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110001111000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110001011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: LSR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110010111000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110010011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROXR,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110011111000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROL,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1110011011000000 {
        let m = get_bits(w0, 3, 3);
        let r = get_bits(w0, 0, 3);
        let sz = 2;
        let src = Implied;
        let dst = cs.ea(r, m, sz);
        let extra = NoExtra;
        return cs.check_overflow(Instruction {
            size: sz,
            operation: ROR,
            operands: [src, dst],
            extra: extra,
        });
    }
    return Err(NotImplemented);
}
#[allow(non_snake_case)]
#[allow(unused_mut)]
pub fn decode_group_1111(
    w0: u16,
    cs: &mut CodeStream,
) -> Result<DecodedInstruction, DecodingError> {
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000011000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FABS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000001011000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FSABS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000001011100 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FDABS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000011100 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FACOS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000100010 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FADD,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000001100010 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FSADD,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000001100110 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FDADD,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000001100 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FASIN,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000001010 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FATAN,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000001101 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FATANH,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001010000000 {
        let c = get_bits(w0, 0, 6);
        let sz = 2;
        let src = PCDISP(2, simple_disp(cs.pull16() as i16 as i32));
        let dst = NoOperand;
        let extra = cs.fpcc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: FBCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1111001011000000 {
        let c = get_bits(w0, 0, 6);
        let sz = 4;
        let src = PCDISP(2, simple_disp(cs.pull32() as i32));
        let dst = NoOperand;
        let extra = cs.fpcc(c);
        return cs.check_overflow(Instruction {
            size: sz,
            operation: FBCC,
            operands: [src, dst],
            extra: extra,
        });
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000111000 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FCMP,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000011101 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FCOS,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000011001 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FCOSH,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111111000) == 0b1111001001001000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1111111111100000) == 0b0000000000000000 {
            let r = get_bits(w0, 0, 3);
            let c = get_bits(w1, 0, 5);
            cs.skip_words(1);
            let sz = 2;
            let src = cs.data_reg_op(r);
            let dst = PCDISP(4, simple_disp(cs.pull16() as i16 as i32));
            let extra = cs.fpcc(c);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FDBCC,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    if (w0 & 0b1111111111000000) == 0b1111001000000000 && cs.has_words(1) {
        let w1 = cs.peek_word(0);
        if (w1 & 0b1010000001111111) == 0b0000000000001110 {
            let m = get_bits(w0, 3, 3);
            let r = get_bits(w0, 0, 3);
            let R = get_bits(w1, 14, 1);
            let s = get_bits(w1, 10, 3);
            let d = get_bits(w1, 7, 3);
            cs.skip_words(1);
            let (sz, src, dst, extra) = cs.decode_fp(r, m, R, s, d);
            return cs.check_overflow(Instruction {
                size: sz,
                operation: FSIN,
                operands: [src, dst],
                extra: extra,
            });
        }
    }
    return Err(NotImplemented);
}
pub fn decode_instruction(code: &[u8]) -> Result<DecodedInstruction, DecodingError> {
    let mut cs = CodeStream::new(code);
    let w0 = cs.pull16();
    match w0 >> 12 {
        0b0000 => decode_group_0000(w0, &mut cs),
        0b0001 => decode_group_0001(w0, &mut cs),
        0b0010 => decode_group_0010(w0, &mut cs),
        0b0011 => decode_group_0011(w0, &mut cs),
        0b0100 => decode_group_0100(w0, &mut cs),
        0b0101 => decode_group_0101(w0, &mut cs),
        0b0110 => decode_group_0110(w0, &mut cs),
        0b0111 => decode_group_0111(w0, &mut cs),
        0b1000 => decode_group_1000(w0, &mut cs),
        0b1001 => decode_group_1001(w0, &mut cs),
        0b1011 => decode_group_1011(w0, &mut cs),
        0b1100 => decode_group_1100(w0, &mut cs),
        0b1101 => decode_group_1101(w0, &mut cs),
        0b1110 => decode_group_1110(w0, &mut cs),
        0b1111 => decode_group_1111(w0, &mut cs),
        _ => Err(NotImplemented),
    }
}
