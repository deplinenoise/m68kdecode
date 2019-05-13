#include "lib.h"

#define CMP2(expr) do { int diff = (int) (lhs-> expr) - (int) (rhs -> expr); if (0 != diff) return diff; } while(0)

static const char* cc_labels[] = {
   "T ",
   "F ",
   "HI",
   "LS",
   "CC",
   "CS",
   "NE",
   "EQ",
   "VC",
   "VS",
   "PL",
   "MI",
   "GE",
   "LT",
   "GT",
   "LE",
};

static const char* fpcc_labels[] = {
    "F  ",
    "EQ ",
    "OGT",
    "OGE",
    "OLT",
    "OLE",
    "OGL",
    "OR ",
    "UN ",
    "UEQ",
    "UGT",
    "UGE",
    "ULT",
    "ULE",
    "NE ",
    "T  ",
    "SF ",
    "SEQ",
    "GT ",
    "GE ",
    "LT ",
    "LE ",
    "GL ",
    "GLE",
    "NGL",
    "NGL",
    "NLE",
    "NLT",
    "NGE",
    "NGT",
    "SNE",
    "ST ",
};


static const char* fpformat_labels[] = {
    "LONG_INT",
    "SINGLE",
    "EXTENDED_REAL",
    "PACKED_DECIMAL_REAL_STATIC",
    "PACKED_DECIMAL_REAL_DYNAMIC",
    "WORD_INT",
    "DOUBLE",
    "BYTE_INT",
};

static const char* opkind_labels[] = {
    "NO_OPERAND",
    "IMPLIED",
    "IMM8",
    "IMM16",
    "IMM32",
    "ABS16",
    "ABS32",
    "DR",
    "AR",
    "FR",
    "ARIND",
    "ARINC",
    "ARDEC",
    "ARDISP",
    "PCDISP",
    "DISP",
    "DPAIR",
    "FPAIR",
    "REGLIST",
    "CONTROLREG",
};

static const char* indexer_labels[] = {
  "N/A",
  "DR",
  "AR",
};

static const char* indirection_labels[] = {
  "N/A",
  "INDIRECT",
  "INDIRECT_PRE_INDEXED",
  "INDIRECT_POST_INDEXED",
};

static int compare_disp(const m68k_disp* lhs, const m68k_disp* rhs)
{
  CMP2(base_displacement);
  CMP2(outer_displacement);
  CMP2(indexer_type);
  CMP2(indexer_reg);
  CMP2(indexer_scale);
  CMP2(indirection_type);
  return 0;
}

static int compare_extra(const m68k_extra* lhs, const m68k_extra* rhs)
{
  CMP2(kind);

  switch (lhs->kind) {
    case M68K_NO_EXTRA:
      break;
    case M68K_EXTRA_BITFIELD:
      CMP2(data.bitfield.flags);
      CMP2(data.bitfield.offset);
      CMP2(data.bitfield.width);
      break;
    case M68K_EXTRA_CC:
      CMP2(data.cc);
      break;
    case M68K_EXTRA_FPCC:
      CMP2(data.fp_cc);
      break;
    case M68K_EXTRA_PACKADJ:
      CMP2(data.pack_adjustment);
      break;
    case M68K_EXTRA_FPFORMAT:
      CMP2(data.fp_data.fp_format);
      CMP2(data.fp_data.k_factor);
      break;
  }

  return 0;
}

/// Compare two instructions, equivalent to memcmp()
static int compare_operands(const m68k_operand* lhs, const m68k_operand *rhs)
{
  CMP2(kind);

  switch (lhs->kind) {
    case M68K_NO_OPERAND:
    case M68K_IMPLIED:
      break;

    case M68K_IMM8:  CMP2(data.u8); break;
    case M68K_IMM16: CMP2(data.u16); break;
    case M68K_IMM32: CMP2(data.u32); break;
    case M68K_ABS16: CMP2(data.abs16); break;
    case M68K_ABS32: CMP2(data.abs32); break;
    case M68K_DR:
    case M68K_AR:
    case M68K_FR:
    case M68K_ARIND:
    case M68K_ARINC:
    case M68K_ARDEC: CMP2(data.regno); break;
    case M68K_ARDISP:
                     CMP2(data.ar_disp.regno);
                     return compare_disp(&lhs->data.ar_disp.disp, &rhs->data.ar_disp.disp);
    case M68K_PCDISP:
                     CMP2(data.pc_disp.pc_offset);
                     return compare_disp(&lhs->data.pc_disp.disp, &rhs->data.pc_disp.disp);
    case M68K_DISP:
                     return compare_disp(&lhs->data.plain_disp, &rhs->data.plain_disp);
    case M68K_DPAIR:
                     CMP2(data.dr_pair.dx);
                     CMP2(data.dr_pair.dy);
                     break;
    case M68K_FPAIR:
                     CMP2(data.fp_pair.dx);
                     CMP2(data.fp_pair.dy);
                     break;
    case M68K_REGLIST:
                     CMP2(data.reglist);
                     break;
    case M68K_CONTROLREG:
                     CMP2(data.control_reg);
                     break;
  }

  return 0;
}

/// Compare two instructions, equivalent to memcmp()
int m68k_compare_instructions(const m68k_instruction* lhs, const m68k_instruction *rhs)
{
  CMP2(size);
  CMP2(operation);
  for (int i = 0; i < 2; ++i)
  {
    int diff;
    if (0 != (diff = compare_operands(&lhs->operands[i], &rhs->operands[i])))
      return diff;
  }

  return compare_extra(&lhs->extra, &rhs->extra);
}

static void print_disp(FILE *stream, const m68k_disp *d)
{
  fprintf(stream, "base=%d outer=%d indexer=%s reg=%d scale=%d indirection=%s",
      d->base_displacement, d->outer_displacement,
      indexer_labels[d->indexer_type],
      d->indexer_reg,
      d->indexer_scale,
      indirection_labels[d->indirection_type]);
}

static void print_operand(FILE *stream, const m68k_operand *i)
{
  fprintf(stream, "%s(", opkind_labels[i->kind]);

  switch (i->kind) {
    case M68K_NO_OPERAND:
    case M68K_IMPLIED:
      break;

    case M68K_IMM8:  fprintf(stream, "%d", i->data.u8); break;
    case M68K_IMM16: fprintf(stream, "%d", i->data.u16); break;
    case M68K_IMM32: fprintf(stream, "%u", i->data.u32); break;
    case M68K_ABS16: fprintf(stream, "%u", i->data.abs16); break;
    case M68K_ABS32: fprintf(stream, "%u", i->data.abs32); break;
    case M68K_DR: fprintf(stream, "D%d", i->data.regno); break;
    case M68K_AR: fprintf(stream, "A%d", i->data.regno); break;
    case M68K_FR: fprintf(stream, "FP%d", i->data.regno); break;
    case M68K_ARIND:
    case M68K_ARINC:
    case M68K_ARDEC:
                     fprintf(stream, "A%d", i->data.regno); break;
    case M68K_ARDISP:
                     fprintf(stream, "A%d -> ", i->data.regno);
                     print_disp(stream, &i->data.ar_disp.disp);
                     break;
    case M68K_PCDISP:
                     fprintf(stream, "PC -> ");
                     print_disp(stream, &i->data.pc_disp.disp);
                     break;
    case M68K_DISP:
                     print_disp(stream, &i->data.plain_disp);
                     break;
    case M68K_DPAIR:
                     fprintf(stream, "D%d, D%d", i->data.dr_pair.dx, i->data.dr_pair.dy); break;
                     break;
    case M68K_FPAIR:
                     fprintf(stream, "FP%d, FP%d", i->data.fp_pair.dx, i->data.fp_pair.dy); break;
                     break;
    case M68K_REGLIST:
                     fprintf(stream, "%02x", i->data.reglist); break;
                     break;
    case M68K_CONTROLREG:
                     fprintf(stream, "%02x", i->data.control_reg); break;
                     break;
  }

  fprintf(stream, ")");
}

void m68k_print_instruction(FILE *stream, const m68k_instruction *i)
{
  fprintf(stream, "%s ", m68k_operation_names[i->operation]);

  for (int x = 0; x < 2; ++x) {
    if (x != 0)
      fprintf(stream, ", ");
    print_operand(stream, &i->operands[x]);
  }

  switch (i->extra.kind) {
    case M68K_NO_EXTRA:
      break;
    case M68K_EXTRA_BITFIELD:
      fprintf(stream, "[bitfield width=");
      if (i->extra.data.bitfield.flags & M68K_BITFIELD_DYNAMIC_WIDTH) {
        fprintf(stream, "%d", i->extra.data.bitfield.width);
      } else {
        fprintf(stream, "D%d", i->extra.data.bitfield.width);
      }
      fprintf(stream, " offset=");
      if (i->extra.data.bitfield.flags & M68K_BITFIELD_DYNAMIC_WIDTH) {
        fprintf(stream, "%d", i->extra.data.bitfield.width);
      } else {
        fprintf(stream, "D%d", i->extra.data.bitfield.width);
      }
      fprintf(stream, "]");
      break;
    case M68K_EXTRA_CC:
      fprintf(stream, "[cc %s]", cc_labels[i->extra.data.cc]);
      break;
    case M68K_EXTRA_FPCC:
      fprintf(stream, "[fpcc %s]", fpcc_labels[i->extra.data.fp_cc]);
      break;
    case M68K_EXTRA_PACKADJ:
      fprintf(stream, "[packadj %d]", i->extra.data.pack_adjustment);
      break;
    case M68K_EXTRA_FPFORMAT:
      fprintf(stream, "[fpformat %s, k=%d]", fpformat_labels[i->extra.data.fp_data.fp_format], i->extra.data.fp_data.k_factor);
      break;
  }
}
