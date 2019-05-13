#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include "lib.h"
#include "decoder_support.inl"

static int test_decoding_result_ok(const char* test_name, const uint8_t *bytes, size_t len, m68k_instruction expected, const char* text)
{
  m68k_decoded_instruction result;
  m68k_decoding_error err = m68k_decode(bytes, (uint32_t) len, &result);
  if (err != M68K_NO_ERROR) {
    fprintf(stderr, "%s failed; err=%d\n", test_name, (int) err);
    return 0;
  }

  if (result.bytes_used != len) {
    fprintf(stderr, "%s failed; incorrect byte count; expected=%d but got %d\n", test_name, (int) len, (int) result.bytes_used);
    return 0;
  }

  if (0 != m68k_compare_instructions(&expected, &result.instruction)) {
    fprintf(stderr, "asm:\n%s\n", text);

    fprintf(stderr, "%s failed; comparison\n", test_name);
    fprintf(stderr, "expected: ");
    m68k_print_instruction(stderr, &expected);
    fprintf(stderr, "\ngot     : ");
    m68k_print_instruction(stderr, &result.instruction);
    fprintf(stderr, "\n");
    return 0;
  }

  return 1;
} 

static int test_decoding_result_err(const char* test_name, const uint8_t *bytes, size_t len, m68k_decoding_error expected_error, const char* text)
{
  m68k_decoded_instruction result;
  m68k_decoding_error err = m68k_decode(bytes, (uint32_t) len, &result);
  if (err != expected_error) {
    fprintf(stderr, "%s: failed\n", test_name);
    return 0;
  }
  return 1;
}

#define DR(n) ((m68k_operand) { .kind = M68K_DR, .data = { .regno = (n) } })
#define AR(n) ((m68k_operand) { .kind = M68K_AR, .data = { .regno = (n) } })
#define FR(n) ((m68k_operand) { .kind = M68K_FR, .data = { .regno = (n) } })
#define ARIND(r) ((m68k_operand) { .kind = M68K_ARIND, .data = { .regno = (r) } })
#define ARINC(r) ((m68k_operand) { .kind = M68K_ARINC, .data = { .regno = (r) } })
#define ARDEC(r) ((m68k_operand) { .kind = M68K_ARDEC, .data = { .regno = (r) } })
#define ARDISP(r, dsp) ((m68k_operand) { .kind = M68K_ARDISP, .data = { .ar_disp = { .regno = (r), .disp = (dsp) } } })
#define DR_DISP_SCALE(r, dsp, scl) ((m68k_disp) { .base_displacement = (dsp), .indexer_type = M68K_INDEXER_DR, .indexer_reg = (r), .indexer_scale = (scl) })
#define DR_DISP(r, dsp) DR_DISP_SCALE(r, dsp, 1)

#define IMM8(n) ((m68k_operand) { .kind = M68K_IMM8, .data = { .u8 = (n) } })
#define IMM16(n) ((m68k_operand) { .kind = M68K_IMM16, .data = { .u16 = (n) } })
#define IMM32(n) ((m68k_operand) { .kind = M68K_IMM32, .data = { .u32 = (n) } })

#define ABS16(n) ((m68k_operand) { .kind = M68K_ABS16, .data = { .abs16 = (n) } })
#define ABS32(n) ((m68k_operand) { .kind = M68K_ABS32, .data = { .abs32 = (n) } })

#define IMPLIED ((m68k_operand) { .kind = M68K_IMPLIED })
#define NO_OPERAND ((m68k_operand) { .kind = M68K_NO_OPERAND })

#define DPAIR(a, b) ((m68k_operand) { .kind = M68K_DPAIR, .data = { .dr_pair = { (a), (b) } } })
#define FPAIR(a, b) ((m68k_operand) { .kind = M68K_FPAIR, .data = { .fp_pair = { (a), (b) } } })
#define REGLIST(n) ((m68k_operand) { .kind = M68K_REGLIST, .data = { .reglist = (n) } })

#define PCDISP(pc_off, dsp) ((m68k_operand) { .kind = M68K_PCDISP, .data = { .pc_disp = { .pc_offset = (pc_off), .disp = (dsp) } } })

#define CONTROLREG(r) ((m68k_operand) { .kind = M68K_CONTROLREG, .data = { .control_reg = (r) } })


#define BF_SS(o, w) ((m68k_extra) { .kind = M68K_EXTRA_BITFIELD, .data = { .bitfield = { 0, o, w } } })
#define BF_DS(o, w) ((m68k_extra) { .kind = M68K_EXTRA_BITFIELD, .data = { .bitfield = { M68K_BITFIELD_DYNAMIC_OFFSET, o, w } } })
#define BF_DD(o, w) ((m68k_extra) { .kind = M68K_EXTRA_BITFIELD, .data = { .bitfield = { M68K_BITFIELD_DYNAMIC_OFFSET|M68K_BITFIELD_DYNAMIC_WIDTH, o, w } } })

#define FPFORMAT(f) extra_floatformat(f, 0)
#define FPFORMAT_K(f,k) extra_floatformat(f, k)
#define CC(c) extra_cc(c)
#define FPCC(c) extra_fpcc(c)
#define PACKADJ(c) extra_packadj(c)

