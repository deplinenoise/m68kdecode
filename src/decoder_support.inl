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

static m68k_operand op_imm8(uint16_t val)
{
  return (m68k_operand) { .kind = M68K_IMM8, .data = { .u8 = (uint8_t) val } };
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
  return op_imm8((uint8_t) pull16(cs));
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
  return (m68k_operand) { .kind = M68K_DR, .data = { .regno = (uint8_t) reg } };
}

static m68k_operand op_ar(int reg)
{
  return (m68k_operand) { .kind = M68K_AR, .data = { .regno = (uint8_t) reg } };
}

static m68k_operand op_fr(int reg)
{
  return (m68k_operand) { .kind = M68K_FR, .data = { .regno = (uint8_t) reg } };
}

static m68k_operand op_arind(int reg)
{
  return (m68k_operand) { .kind = M68K_ARIND, .data = { .regno = (uint8_t) reg } };
}

static m68k_operand op_arinc(int reg)
{
  return (m68k_operand) { .kind = M68K_ARINC, .data = { .regno = (uint8_t) reg } };
}

static m68k_operand op_ardec(int reg)
{
  return (m68k_operand) { .kind = M68K_ARDEC, .data = { .regno = (uint8_t) reg } };
}

static m68k_operand op_ardisp(int reg, m68k_disp disp)
{
  return (m68k_operand) { .kind = M68K_ARDISP, .data = { .ar_disp = { .regno = (uint8_t) reg, .disp = disp } } };
}

static m68k_operand op_implied(void)
{
  return (m68k_operand) { .kind = M68K_IMPLIED };
}

static m68k_operand op_none(void)
{
  return (m68k_operand) { .kind = M68K_NO_OPERAND };
}

static m68k_operand op_controlreg(uint16_t r)
{
  return (m68k_operand) { .kind = M68K_CONTROLREG, .data = { .control_reg = r } };
}

static m68k_operand op_reglist(uint16_t r)
{
  return (m68k_operand) { .kind = M68K_REGLIST, .data = { .reglist = r } };
}

static m68k_operand op_dpair(uint16_t x, uint16_t y)
{
  return (m68k_operand) { .kind = M68K_DPAIR, .data = { .dr_pair = { (int8_t)x, (int8_t)y } } };
}

static m68k_operand op_fpair(uint16_t x, uint16_t y)
{
  return (m68k_operand) { .kind = M68K_FPAIR, .data = { .fp_pair = { (int8_t)x, (int8_t)y } } };
}

static m68k_operand op_abs16(uint16_t val)
{
  return (m68k_operand) { .kind = M68K_ABS16, .data = { .abs16 = val } };
}

static m68k_operand op_abs32(uint32_t val)
{
  return (m68k_operand) { .kind = M68K_ABS32, .data = { .abs32 = val } };
}

static m68k_disp simple_disp(int32_t disp)
{
  return (m68k_disp) { .base_displacement = disp, .indexer_scale = 1 };
}

static m68k_operand op_pcdisp(int pc_off, m68k_disp disp)
{
  return (m68k_operand) { .kind = M68K_PCDISP, .data = { .pc_disp = { .pc_offset = (uint8_t) pc_off, .disp = disp } } };
}

static m68k_operand op_quick_const(int val)
{
  return (m68k_operand) { .kind = M68K_IMM8, .data = { .u8 = val ? (uint8_t) val : 8 } };
}

static m68k_extra extra_cc(int code)
{
  return (m68k_extra) { .kind = M68K_EXTRA_CC, .data = { .cc = code } };
}

static m68k_extra extra_fpcc(int code)
{
  return (m68k_extra) { .kind = M68K_EXTRA_FPCC, .data = { .fp_cc = code } };
}

static m68k_extra extra_packadj(uint16_t adjust)
{
  return (m68k_extra) { .kind = M68K_EXTRA_PACKADJ, .data = { .pack_adjustment = adjust } };
}

static m68k_extra extra_bitfield(uint16_t dyn_off, uint16_t off, uint16_t dyn_width, uint16_t width)
{
  uint8_t flags = 0;
  uint8_t r_off = (uint8_t) off;
  uint8_t r_width = (uint8_t) width;

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
  uint8_t pc_off = (uint8_t) cs->pos;
  uint16_t ext = pull16(cs);
  uint16_t scale = get_bits(ext, 9, 2);
  uint16_t idx = get_bits(ext, 12, 3);
  int idx_is_a = get_bits(ext, 15, 1);

  if (ext & (1 << 8)) {
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

    int suppress_base = get_bits(ext, 7, 1);
    int suppress_indexer = get_bits(ext, 6, 1);

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
      indexer_reg = (int8_t) idx;
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
        .data = { .ar_disp = { .disp = displacement, .regno = (int8_t) src_reg } }
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
      .indexer_reg = (int8_t) idx,
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
        .data = { .ar_disp = { .disp = displacement, .regno = (int8_t) src_reg } }
      };
    }
  }
}

static m68k_operand op_dar(int d_or_a, int regno)
{
  if (d_or_a == 0) {
    return (m68k_operand) { .kind = M68K_DR, .data = { .regno = (int8_t) regno } };
  } else {
    return (m68k_operand) { .kind = M68K_AR, .data = { .regno = (int8_t) regno } };
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
    case 5: /* 101 */ return op_ardisp(src_reg, simple_disp((int16_t)pull16(cs)));
    case 6: /* 110 */ return decode_extended_ea(cs, src_reg);
    case 7: /* 111 */ {
      switch (src_reg) {
        case 0: /* 000 */ return op_abs16(pull16(cs));
        case 1: /* 001 */ return op_abs32(pull32(cs));
        case 2: /* 010 */ {
          uint8_t pc_offset = (uint8_t) cs->pos;
          return (m68k_operand) {
            .kind = M68K_PCDISP,
            .data = {
              .pc_disp = {
                .pc_offset = pc_offset,
                .disp = simple_disp((int16_t)pull16(cs)),
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
  if (m_r) {
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
