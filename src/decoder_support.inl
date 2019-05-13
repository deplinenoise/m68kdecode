#pragma once

#include "lib.h"

static inline uint16_t get_bits(uint16_t word, int first, int length)
{
  uint16_t s = word >> first;
  uint16_t mask = (1 << length) - 1;
  return s & mask;
}

typedef struct m68k_code_stream {
  const uint8_t *bytes;
  uint32_t len;
  uint32_t pos;
  m68k_decoding_error error;
} m68k_code_stream;

static void m68k_code_stream_init(m68k_code_stream *cs, const uint8_t *bytes, uint32_t len)
{
  cs->bytes = bytes;
  cs->len = len;
  cs->pos = 0;
  cs->error = M68K_NO_ERROR;
}

static uint16_t pull16(m68k_code_stream *cs)
{
  uint32_t pos = cs->pos;
  uint16_t result = 0xffff;

  if (cs->len >= pos + 2) {
    uint16_t hi = cs->bytes[pos];
    uint16_t lo = cs->bytes[pos + 1];
    result = (hi << 8) | lo;
  }

  cs->pos = pos + 2;
  return result;
}

static uint32_t pull32(m68k_code_stream *cs)
{
  uint32_t pos = cs->pos;
  uint32_t result = ~0u;

  if (cs->len >= pos + 4) {
    uint32_t b0 = cs->bytes[pos];
    uint32_t b1 = cs->bytes[pos + 1];
    uint32_t b2 = cs->bytes[pos + 2];
    uint32_t b3 = cs->bytes[pos + 3];
    result = (b0 << 24) | (b1 << 16) | (b2 << 8) | b3;
  }

  cs->pos = pos + 4;
  return result;
}

static m68k_decoding_error check_overflow(m68k_code_stream *cs, m68k_instruction insn, m68k_decoded_instruction *result)
{
  if (cs->error != M68K_NO_ERROR) {
    return cs->error;
  }

  if (cs->pos > cs->len) {
    return M68K_OUT_OF_SPACE;
  }
  
  result->bytes_used = cs->pos;
  result->instruction = insn;
  return M68K_NO_ERROR;
}

static int has_words(m68k_code_stream *cs, int count)
{
  return (int)cs->len - (int)cs->pos >= 2 * count;
}

static uint16_t peek_word(m68k_code_stream *cs, int offset)
{
  uint32_t p = cs->pos + offset;
  if (p + 2 <= cs->len) {
    uint16_t hi = cs->bytes[p];
    uint16_t lo = cs->bytes[p + 1];
    return (hi << 8) | lo;
  }
  cs->error = M68K_OUT_OF_SPACE;
  return 0;
}

static void skip_words(m68k_code_stream *cs, int count)
{
  cs->pos += count * 2;
}

/*
    pub fn has_words(&self, count: usize) -> bool {
        self.pos + count * 2 <= self.bytes.len()
    }

    pub fn peek_word(&mut self, offset: i32) -> u16 {
        let p = self.pos + offset as usize;
        if p + 2 <= self.bytes.len() {
            ((self.bytes[p] as u16) << 8) | (self.bytes[p + 1] as u16)
        } else {
            self.error = Some(OutOfSpace);
            0
        }
    }

    pub fn skip_words(&mut self, count: usize) {
        self.pos += 2 * count;
    }

    pub fn pull16(&mut self) -> u16 {
        let result = self.peek_word(0);
        self.pos += 2;
        result
    }

    pub fn pull32(&mut self) -> u32 {
        let a = self.pull16() as u32;
        let b = self.pull16() as u32;
        (a << 16) | b
    }

    pub fn data_reg(&mut self, r: u16) -> DataRegister {
        match r {
            0 => D0,
            1 => D1,
            2 => D2,
            3 => D3,
            4 => D4,
            5 => D5,
            6 => D6,
            7 => D7,
            _ => {
                self.error = Some(BadRegister);
                D0
            }
        }
    }

    pub fn address_reg(&mut self, r: u16) -> AddressRegister {
        match r {
            0 => A0,
            1 => A1,
            2 => A2,
            3 => A3,
            4 => A4,
            5 => A5,
            6 => A6,
            7 => A7,
            _ => {
                self.error = Some(BadRegister);
                A0
            }
        }
    }

    pub fn float_reg(&mut self, r: u16) -> FloatingRegister {
        match r {
            0 => FP0,
            1 => FP1,
            2 => FP2,
            3 => FP3,
            4 => FP4,
            5 => FP5,
            6 => FP6,
            7 => FP7,
            _ => {
                self.error = Some(BadRegister);
                FP0
            }
        }
    }

    pub fn data_reg_op(&mut self, r: u16) -> Operand {
        DR(self.data_reg(r))
    }

    pub fn address_reg_op(&mut self, r: u16) -> Operand {
        AR(self.address_reg(r))
    }

    pub fn float_reg_op(&mut self, r: u16) -> Operand {
        FR(self.float_reg(r))
    }


    fn decode_extended_ea(&mut self, src_reg: Option<AddressRegister>) -> Operand {
        let pc_off = self.pos as u8;
        println!("decode_extended_ea: pc_off={}", pc_off);
        let ext = self.pull16();
        let scale = get_bits(ext, 9, 2) as u8;
        let idx = get_bits(ext, 12, 3);
        let idx_is_a = get_bits(ext, 15, 1) == 1;

        if 0 != (ext & (1 << 8)) {
            // Handle full extension word.
            let bd = get_bits(ext, 4, 2);
            let od = get_bits(ext, 0, 2);
            let disp = match bd {
                0 => { self.error = Some(Reserved); 0 },
                1 => 0u32,
                2 => self.pull16() as i16 as i32 as u32,
                3 => self.pull32(),
                _ => { self.error = Some(NotImplemented); 0 },
            };
            let odisp = match od {
                0 => 0u32,
                1 => 0u32,
                2 => self.pull16() as i16 as i32 as u32,
                3 => self.pull32(),
                _ => { self.error = Some(NotImplemented); 0 },
            };

            let suppress_base = get_bits(ext, 7, 1) == 1;
            let suppress_indexer = get_bits(ext, 6, 1) == 1;

            let indirection_mode = match suppress_indexer {
                false => {
                    match get_bits(ext, 0, 3) {
                        0b000 => NoIndirection,
                        0b001 => IndirectPreIndexed,
                        0b010 => IndirectPreIndexed,
                        0b011 => IndirectPreIndexed,
                        0b100 => { self.error = Some(Reserved); NoIndirection },
                        0b101 => IndirectPostIndexed,
                        0b110 => IndirectPostIndexed,
                        0b111 => IndirectPostIndexed,
                        _ => { self.error = Some(NotImplemented); NoIndirection },
                    }
                },
                true => {
                    match get_bits(ext, 0, 3) {
                        0b000 => NoIndirection,
                        0b001 => Indirect,
                        0b010 => Indirect,
                        0b011 => Indirect,
                        _ => { self.error = Some(Reserved); NoIndirection },
                    }
                },
            };

            let indexer = match suppress_indexer {
                true => Indexer::NoIndexer,
                false => if idx_is_a {
                    Indexer::AR(self.address_reg(idx), scale)
                } else {
                    Indexer::DR(self.data_reg(idx), scale)
                },
            };

            if suppress_base {
                DISP(Displacement {
                    base_displacement: disp as i32,
                    outer_displacement: odisp as i32,
                    indexer: indexer,
                    indirection: indirection_mode,
                })
            } else {
                match src_reg {
                    None => PCDISP(pc_off, Displacement {
                        base_displacement: disp as i32,
                        outer_displacement: odisp as i32,
                        indexer: indexer,
                        indirection: indirection_mode,
                    }),
                    Some(reg) => ARDISP(reg, Displacement {
                        base_displacement: disp as i32,
                        outer_displacement: odisp as i32,
                        indexer: indexer,
                        indirection: indirection_mode,
                    }),
                }
            }
        } else {
            // Handle brief extension word
            let disp = ((ext & 0xff) as i8) as i32;
            let indexer =  if idx_is_a {
                Indexer::AR(self.address_reg(idx), scale)
            } else {
                Indexer::DR(self.data_reg(idx), scale)
            };

            let displacement = Displacement { 
                base_displacement: disp,
                outer_displacement: 0,
                indexer: indexer,
                indirection: NoIndirection,
            };

            match src_reg {
                None => PCDISP(pc_off, displacement),
                Some(r) => ARDISP(r, displacement),
            }
        }
    }

    pub fn imm8(&mut self) -> Operand {
        IMM8(self.pull16() as u8)
    }

    pub fn imm16(&mut self) -> Operand {
        IMM16(self.pull16())
    }

    pub fn imm32(&mut self) -> Operand {
        IMM32(self.pull32())
    }

    pub fn bitfield(&mut self, dyn_off: u16, off: u16, dyn_width: u16, width: u16) -> InstructionExtra {
        let bf_offset = match dyn_off {
            0 => STATIC(if off & 31 == 0 { 32 } else { (off & 31) as u8 }),
            _ => DYNAMIC(self.data_reg(off)),
        };
        let bf_width = match dyn_width {
            0 => STATIC(if width & 31 == 0 { 32 } else { (width & 31) as u8 }),
            _ => DYNAMIC(self.data_reg(width)),
        };
        Bitfield(bf_offset, bf_width)
    }

    pub fn cc(&self, c: u16) -> InstructionExtra {
        Condition(match c {
            0b0000 => CC_T  ,         // Always True
            0b0001 => CC_F  ,         // Always False
            0b0010 => CC_HI ,         // High
            0b0011 => CC_LS ,         // Low or Same
            0b0100 => CC_CC ,         // Carry Clear
            0b0101 => CC_CS ,         // Carry Set
            0b0110 => CC_NE ,         // Not Equal
            0b0111 => CC_EQ ,         // Equal
            0b1000 => CC_VC ,         // Overflow Clear
            0b1001 => CC_VS ,         // Overflow Set
            0b1010 => CC_PL ,         // Plus
            0b1011 => CC_MI ,         // Negative
            0b1100 => CC_GE ,         // Greater or Equal
            0b1101 => CC_LT ,         // Less
            0b1110 => CC_GT ,         // Greater
            _      => CC_LE ,         // Less or Equal
        })
    }

    pub fn quick_const(&self, i: u16) -> Operand {
        IMM8(if i == 0 { 8 } else { i as u8 })
    }

    pub fn fpcc(&self, c: u16) -> InstructionExtra {
        FPCondition(match c {
            0b000000 => FPCC_F     , // False
            0b000001 => FPCC_EQ    , // Equal
            0b000010 => FPCC_OGT   , // Ordered Greater Than
            0b000011 => FPCC_OGE   , // Ordered Greater Than or Equal
            0b000100 => FPCC_OLT   , // Ordered Less Than
            0b000101 => FPCC_OLE   , // Ordered Less Than or Equal
            0b000110 => FPCC_OGL   , // Ordered Greater Than or Less Than
            0b000111 => FPCC_OR    , // Ordered
            0b001000 => FPCC_UN    , // Unordered
            0b001001 => FPCC_UEQ   , // Unordered or Equal
            0b001010 => FPCC_UGT   , // Unordered or Greater Than
            0b001011 => FPCC_UGE   , // Unordered or Greater Than or Equal
            0b001100 => FPCC_ULT   , // Unordered or Less Than
            0b001101 => FPCC_ULE   , // Unordered or Less Than or Equal
            0b001110 => FPCC_NE    , // Not Equal
            0b001111 => FPCC_T     , // True
            0b010000 => FPCC_SF    , // Signaling False
            0b010001 => FPCC_SEQ   , // Signaling Equal
            0b010010 => FPCC_GT    , // Greater Than
            0b010011 => FPCC_GE    , // Greater Than or Equal
            0b010100 => FPCC_LT    , // Less Than
            0b010101 => FPCC_LE    , // Less Than or Equal
            0b010110 => FPCC_GL    , // Greater Than or Less Than
            0b010111 => FPCC_GLE   , // Greater Than or Less Than or Equal
            0b011000 => FPCC_NGLE  , // Not (Greater Than or Less Than or Equal)
            0b011001 => FPCC_NGL   , // Not (Greater Than or Less Than)
            0b011010 => FPCC_NLE   , // Not (Less Than or Equal)
            0b011011 => FPCC_NLT   , // Not (Less Than)
            0b011100 => FPCC_NGE   , // Not (Greater Than or Equal)
            0b011101 => FPCC_NGT   , // Not (Greater Than)
            0b011110 => FPCC_SNE   , // Signaling Not Equal
            _        => FPCC_ST    , // Signaling True
        })
    }

}
*/

static m68k_operand op_imm8(uint8_t val)
{
  return (m68k_operand) { .kind = M68K_IMM8, .data = { .u8 = val } };
}

static m68k_operand op_imm16(uint16_t val)
{
  return (m68k_operand) { .kind = M68K_IMM16, .data = { .u16 = val } };
}

static m68k_operand op_imm32(uint32_t val)
{
  return (m68k_operand) { .kind = M68K_IMM32, .data = { .u32 = val } };
}

static m68k_operand op_imm8p(m68k_code_stream *cs)
{
  return op_imm8(pull16(cs));
}

static m68k_operand op_imm16p(m68k_code_stream *cs)
{
  return op_imm16(pull16(cs));
}

static m68k_operand op_imm32p(m68k_code_stream *cs)
{
  return op_imm32(pull32(cs));
}

static m68k_operand op_dr(int reg)
{
  return (m68k_operand) { .kind = M68K_DR, .data = { .regno = reg } };
}

static m68k_operand op_ar(int reg)
{
  return (m68k_operand) { .kind = M68K_AR, .data = { .regno = reg } };
}

static m68k_operand op_fr(int reg)
{
  return (m68k_operand) { .kind = M68K_FR, .data = { .regno = reg } };
}

static m68k_operand op_arind(int reg)
{
  return (m68k_operand) { .kind = M68K_ARIND, .data = { .regno = reg } };
}

static m68k_operand op_arinc(int reg)
{
  return (m68k_operand) { .kind = M68K_ARINC, .data = { .regno = reg } };
}

static m68k_operand op_ardec(int reg)
{
  return (m68k_operand) { .kind = M68K_ARDEC, .data = { .regno = reg } };
}

static m68k_operand op_ardisp(int reg, m68k_disp disp)
{
  return (m68k_operand) { .kind = M68K_ARDISP, .data = { .ar_disp = { .regno = reg, .disp = disp } } };
}

static m68k_operand op_implied(void)
{
  return (m68k_operand) { .kind = M68K_IMPLIED };
}

static m68k_operand op_none(void)
{
  return (m68k_operand) { .kind = M68K_NO_OPERAND };
}

static m68k_operand op_controlreg(int r)
{
  return (m68k_operand) { .kind = M68K_CONTROLREG, .data = { .control_reg = r } };
}

static m68k_operand op_reglist(int r)
{
  return (m68k_operand) { .kind = M68K_REGLIST, .data = { .reglist = r } };
}

static m68k_operand op_dpair(int x, int y)
{
  return (m68k_operand) { .kind = M68K_DPAIR, .data = { .dr_pair = { x, y } } };
}

static m68k_operand op_fpair(int x, int y)
{
  return (m68k_operand) { .kind = M68K_FPAIR, .data = { .fp_pair = { x, y } } };
}

static m68k_operand op_abs16(uint16_t val)
{
  return (m68k_operand) { .kind = M68K_ABS16, .data = { .abs16 = val } };
}

static m68k_operand op_abs32(uint32_t val)
{
  return (m68k_operand) { .kind = M68K_ABS32, .data = { .abs32 = val } };
}

static m68k_disp simple_disp(uint16_t disp)
{
  return (m68k_disp) { .base_displacement = (int16_t) disp, .indexer_scale = 1 };
}

static m68k_operand op_pcdisp(int pc_off, m68k_disp disp)
{
  return (m68k_operand) { .kind = M68K_PCDISP, .data = { .pc_disp = { .pc_offset = pc_off, .disp = disp } } };
}

static m68k_operand op_quick_const(int val)
{
  return (m68k_operand) { .kind = M68K_IMM8, .data = { .u8 = val ? val : 8 } };
}


static m68k_extra extra_cc(int code)
{
  return (m68k_extra) { .kind = M68K_EXTRA_CC, .data = { .cc = code } };
}

static m68k_extra extra_fpcc(int code)
{
  return (m68k_extra) { .kind = M68K_EXTRA_FPCC, .data = { .fp_cc = code } };
}

static m68k_extra extra_packadj(int adjust)
{
  return (m68k_extra) { .kind = M68K_EXTRA_PACKADJ, .data = { .pack_adjustment = adjust } };
}

static m68k_extra extra_bitfield(uint16_t dyn_off, uint16_t off, uint16_t dyn_width, uint16_t width)
{
  uint8_t flags = 0;
  uint8_t r_off = off;
  uint8_t r_width = width;

  if (dyn_off) {
    flags |= M68K_BITFIELD_DYNAMIC_OFFSET;
  } else {
    r_off &= 31;
    if (r_off == 0) {
      r_off = 32;
    }
  }

  if (dyn_width) {
    flags |= M68K_BITFIELD_DYNAMIC_WIDTH;
  } else {
    r_width &= 31;
    if (r_width == 0) {
      r_width = 32;
    }
  }
  return (m68k_extra) { .kind = M68K_EXTRA_BITFIELD, .data = { .bitfield = { .flags = flags, .offset = r_off, .width = r_width } } };
}

static m68k_extra extra_floatformat(m68k_fpformat format, int k_factor)
{
  return (m68k_extra) { .kind = M68K_EXTRA_FPFORMAT, .data = { .fp_data = { .fp_format = format, .k_factor = k_factor } } };
}


static m68k_operand decode_extended_ea(m68k_code_stream *cs, int src_reg)
{
  uint8_t pc_off = cs->pos;
  uint16_t ext = pull16(cs);
  uint16_t scale = get_bits(ext, 9, 2);
  uint16_t idx = get_bits(ext, 12, 3);
  int idx_is_a = get_bits(ext, 15, 1) == 1;

  if (0 != (ext & (1 << 8))) {
    // Handle full extension word.
    uint16_t bd = get_bits(ext, 4, 2);
    uint16_t od = get_bits(ext, 0, 2);

    uint32_t disp = 0;
    switch (bd) {
      case 0: cs->error = M68K_RESERVED; break;
      case 1: disp = 0; break;
      case 2: disp = (int16_t) pull16(cs); break;
      case 3: disp = pull32(cs); break;
    }

    uint32_t odisp = 0;
    switch (od) {
      case 0: odisp = 0; break;
      case 1: odisp = 0; break;
      case 2: odisp = (int16_t) pull16(cs); break;
      case 3: odisp = pull32(cs); break;
    }

    int suppress_base = get_bits(ext, 7, 1) == 1;
    int suppress_indexer = get_bits(ext, 6, 1) == 1;

    m68k_memind indirection_mode = M68K_NO_INDIRECTION;

    if (suppress_indexer) {
      switch (get_bits(ext, 0, 3)) {
        case 0: indirection_mode = M68K_NO_INDIRECTION; break;
        case 1: indirection_mode = M68K_INDIRECT; break;
        case 2: indirection_mode = M68K_INDIRECT; break;
        case 3: indirection_mode = M68K_INDIRECT; break;
        default: cs->error = M68K_RESERVED; break;
      }
    } else {
      switch (get_bits(ext, 0, 3)) {
        case 0: indirection_mode = M68K_NO_INDIRECTION; break;
        case 1: indirection_mode = M68K_INDIRECT_PRE_INDEXED; break;
        case 2: indirection_mode = M68K_INDIRECT_PRE_INDEXED; break;
        case 3: indirection_mode = M68K_INDIRECT_PRE_INDEXED; break;
        case 4: indirection_mode = cs->error = M68K_RESERVED; break;
        case 5: indirection_mode = M68K_INDIRECT_POST_INDEXED; break;
        case 6: indirection_mode = M68K_INDIRECT_POST_INDEXED; break;
        case 7: indirection_mode = M68K_INDIRECT_POST_INDEXED; break;
        default: {
          cs->error = M68K_NOT_IMPLEMENTED;
          break;
        }
      }
    }

    m68k_indexer_type indexer_type = M68K_NO_INDEXER;
    int8_t indexer_reg = 0;

    if (!suppress_indexer) {
      indexer_reg = idx;
      if (idx_is_a) {
        indexer_type = M68K_INDEXER_AR;
      } else {
        indexer_type = M68K_INDEXER_DR;
      }
    }

    const m68k_disp displacement = {
      .base_displacement = disp,
      .outer_displacement = odisp,
      .indexer_type = indexer_type,
      .indexer_reg = indexer_reg,
      .indirection_type = indirection_mode,
      .indexer_scale = 1 << scale,
    };

    if (suppress_base) {
      return (m68k_operand) {
        .kind = M68K_DISP,
        .data = { .plain_disp = displacement }
      };
    } else if (src_reg < 0) {
      return (m68k_operand) {
        .kind = M68K_PCDISP,
        .data = { .pc_disp = { .disp = displacement, .pc_offset = pc_off } }
      };
    } else {
      return (m68k_operand) {
        .kind = M68K_ARDISP,
        .data = { .ar_disp = { .disp = displacement, .regno = src_reg } }
      };
    }
  }

  else {
    // Handle brief extension word
    int disp = (int8_t) (ext & 0xff);
    m68k_disp displacement = { 
      .base_displacement = disp,
      .outer_displacement = 0,
      .indexer_type = idx_is_a ? M68K_INDEXER_AR : M68K_INDEXER_DR,
      .indexer_reg = idx,
      .indirection_type = M68K_NO_INDIRECTION,
      .indexer_scale = 1 << scale,
    };

    if (src_reg < 0) {
      return (m68k_operand) {
        .kind = M68K_PCDISP,
        .data = { .pc_disp = { .disp = displacement, .pc_offset = pc_off } }
      };
    } else {
      return (m68k_operand) {
        .kind = M68K_ARDISP,
        .data = { .ar_disp = { .disp = displacement, .regno = src_reg } }
      };
    }
  }
}

static m68k_operand op_dar(int d_or_a, int regno)
{
  if (d_or_a == 0) {
    return (m68k_operand) { .kind = M68K_DR, .data = { .regno = regno } };
  } else {
    return (m68k_operand) { .kind = M68K_AR, .data = { .regno = regno } };
  }
}

static m68k_operand op_ea(m68k_code_stream *cs, uint16_t src_reg, uint16_t src_mod, int size)
{
  switch (src_mod) {
    case 0: /* 000 */ return op_dr(src_reg);
    case 1: /* 001 */ return op_ar(src_reg);
    case 2: /* 010 */ return op_arind(src_reg);
    case 3: /* 011 */ return op_arinc(src_reg);
    case 4: /* 100 */ return op_ardec(src_reg);
    case 5: /* 101 */ return op_ardisp(src_reg, simple_disp(pull16(cs)));
    case 6: /* 110 */ return decode_extended_ea(cs, src_reg);
    case 7: /* 111 */ {
      switch (src_reg) {
        case 0: /* 000 */ return op_abs16(pull16(cs));
        case 1: /* 001 */ return op_abs32(pull32(cs));
        case 2: /* 010 */ {
          uint8_t pc_offset = cs->pos;
          return (m68k_operand) {
            .kind = M68K_PCDISP,
            .data = {
              .pc_disp = {
                .pc_offset = pc_offset,
                .disp = simple_disp(pull16(cs)),
              },
            }
          };
        }
        case 3: /* 011 */ return decode_extended_ea(cs, -1);
        case 4: /* 100 */ {
          switch (size) {
            case 1: return op_imm8p(cs);
            case 2: return op_imm16p(cs);
            case 4: return op_imm32p(cs);
            default: {
              cs->error = M68K_BAD_SIZE;
              break;
            }
          }
          break;
        }
        default: {
          cs->error = M68K_NOT_IMPLEMENTED;
          break;
        }
      }
    }
    default: {
      cs->error = M68K_NOT_IMPLEMENTED;
      break;
    }
  }

  return (m68k_operand) { .kind = M68K_NO_OPERAND };
}

static void decode_fp(
    int *out_sz, m68k_operand *out_src, m68k_operand *out_dst, m68k_extra *out_extra,
    m68k_code_stream *cs, uint16_t rg, uint16_t md, uint16_t m_r, uint16_t s, uint16_t d, uint16_t k)
{
  if (m_r == 1) {
    int k_factor = 0;
    int sz;
    m68k_fpformat format;

    switch (s) {
      case 0: /* 000 */ sz =  4; format = M68K_FPF_LONG_INT; break;
      case 1: /* 001 */ sz =  4; format = M68K_FPF_SINGLE; break;
      case 2: /* 010 */ sz = 10; format = M68K_FPF_EXTENDED_REAL; break;
      case 3: /* 011 */ sz = 12; format = M68K_FPF_PACKED_DECIMAL_REAL_STATIC; k_factor = ((int8_t)(k << 1)) >> 1; break;
      case 4: /* 100 */ sz =  2; format = M68K_FPF_WORD_INT; break;
      case 5: /* 101 */ sz =  8; format = M68K_FPF_DOUBLE; break;
      case 6: /* 110 */ sz =  1; format = M68K_FPF_BYTE_INT; break;
      case 7: /* 111 */ sz = 12; format = M68K_FPF_PACKED_DECIMAL_REAL_DYNAMIC; k_factor = k >> 4; break;
      default: cs->error = M68K_RESERVED; sz = 1; format = M68K_FPF_BYTE_INT; break;
    };

    *out_sz = sz;
    *out_src = op_ea(cs, rg, md, sz);
    *out_dst = op_fr(d);
    *out_extra = extra_floatformat(format, k_factor);
  } else {
    *out_sz = 10;
    *out_src = op_fr(s);
    *out_dst = op_fr(d);
    *out_extra = extra_floatformat(M68K_FPF_EXTENDED_REAL, 0);
  }
}

static void decode_fp_movem(
    int *out_sz, m68k_operand *out_src, m68k_operand *out_dst, m68k_extra *out_extra,
    m68k_code_stream *cs, uint16_t r, uint16_t m, uint16_t direction, uint16_t mask, uint16_t mode)
{
  m68k_operand ea = op_ea(cs, r, m, 10);

  m68k_operand rl;

  if (0 == (mode & 1)) {
    rl = op_reglist(mask);
  } else {
    rl = op_dr(mask >> 4);
  }

  *out_sz = 10;
  *out_extra = extra_floatformat(M68K_FPF_EXTENDED_REAL, 0);

  if (0 == direction) {
    // move from memory to float registers
    *out_src = ea;
    *out_dst = rl;
  } else {
    // move from registers to memory
    *out_src = rl;
    *out_dst = ea;
  }
}
