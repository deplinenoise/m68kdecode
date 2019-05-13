//! \file lib.h
//! \brief An instruction decoder for M680x0 family instructions.
#pragma once

#include <stdint.h>
#include <stdio.h>
#include "decoder.h"

/// Indicates how memory indexing is to be performed for 68020+ double memory operands.
typedef enum m68k_memind {
  /// No memory indirection.
  M68K_NO_INDIRECTION,
  /// Regular memory indirection.
  M68K_INDIRECT,
  /// Memory pre-indexed indirection (indexer applies to inner array).
  M68K_INDIRECT_PRE_INDEXED,
  /// Memory post-indexed indirection (indexer applies to outer array).
  M68K_INDIRECT_POST_INDEXED,
} m68k_memind;

/// Indicates how indexing is to be performed.
typedef enum m68k_indexer_type {
  /// Indexing suppressed
  M68K_NO_INDEXER,
  /// Index using data register
  M68K_INDEXER_DR,
  /// Index using address register
  M68K_INDEXER_AR
} m68k_indexer_type;

/// Indicates how a memory operand effective address is to be computed using displacement.
typedef struct m68k_disp {
    /// Base displacement. This is always sign-extended to 32-bit by the instruction decoder.
    int base_displacement;
    /// Outer displacement (only used for 68020+ double indirection modes, zero otherwise)
    int outer_displacement;

    /// Indicates how indexing is to be performed.
    m68k_indexer_type indexer_type;

    /// If indexing is to be performed, specifies address or data register to use.
    int8_t indexer_reg;

    /// Scale, 1, 2, 4 or 8
    int8_t indexer_scale;

    /// Indicates what type of memory indirection is to be performed.
    m68k_memind indirection_type;
} m68k_disp;

/// Describes one of the two operands in an instruction
typedef enum m68k_operand_kind {
    /// No operand present, used for instructions that have fewer than two operands.
    M68K_NO_OPERAND,
    /// The operand is implied, for example when the CCR is being moved to or from.
    ///
    /// When the operand is implied, the instruction documentation must be consulted to see what
    /// data is being affected.
    M68K_IMPLIED,
    /// An immediate 8-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    M68K_IMM8,
    /// An immediate 16-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    M68K_IMM16,
    /// An immediate 32-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    M68K_IMM32,
    /// An absolute address, given as a sign-extended 16-bit value.
    M68K_ABS16,
    /// An absolute address, given as a full 32-bit value.
    M68K_ABS32,
    /// A data register
    M68K_DR,
    /// An address register
    M68K_AR,
    /// A floating point register
    M68K_FR,
    /// A vanilla indrection without displacement, e.g. `(a0)`
    M68K_ARIND,
    /// Address register indirect with post-increment, e.g. `(a0)+`
    M68K_ARINC,
    /// Address register indirect with pre-decrement, e.g. `-(a0)`
    M68K_ARDEC,
    /// Address register indirect with displacement, e.g. `123(pc,d0)`
    M68K_ARDISP,
    /// Program counter indirect with displacement, e.g. `123(pc,d0)`
    M68K_PCDISP,
    /// Just a displacement (skipping the base register), e.g. `123(d0)`
    M68K_DISP,
    /// A data register pair, used for 64-bit multiply/divide results.
    M68K_DPAIR,
    /// A floating point register pair, used for `FSINCOS`. First register receives the sine, the
    /// other the cosine.
    M68K_FPAIR,
    /// A register bitmask for `MOVEM` or `FMOVEM`. The order of registers is reversed depending on
    /// whether the address register is pre-decremented or post-incremented.
    M68K_REGLIST,
    /// A control register
    M68K_CONTROLREG,
} m68k_operand_kind;

/// Describes one of the two operands in an instruction
typedef union m68k_operand_data {
    /// An immediate 8-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    uint8_t u8;
    /// An immediate 16-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    uint16_t u16;
    /// An immediate 32-bit value.
    ///
    /// Stored as unsigned, but it often needs to be interpreted as signed.
    uint32_t u32;
    /// An absolute address, given as a sign-extended 16-bit value.
    int16_t abs16;
    /// An absolute address, given as a full 32-bit value.
    uint32_t abs32;
    /// Number of data, address or fp register
    int8_t regno;
    /// Address register indirect with displacement, e.g. `123(a2,d3)`
    struct {
      int8_t regno;
      m68k_disp disp;
    } ar_disp;
    /// Program counter indirect with displacement, e.g. `123(pc,d0)`
    struct {
      /// Offset from instruction PC (in bytes) reference is relative to.
      uint8_t pc_offset;
      m68k_disp disp;
    } pc_disp;

    /// Just a displacement (skipping the base register), e.g. `123(d0)`
    m68k_disp plain_disp;

    /// A data register pair, used for 64-bit multiply/divide results.
    struct {
      int8_t dx;
      int8_t dy;
    } dr_pair;

    /// A floating point register pair, used for `FSINCOS`. First register receives the sine, the
    /// other the cosine.
    struct {
      int8_t dx;
      int8_t dy;
    } fp_pair;

    /// A register bitmask for `MOVEM` or `FMOVEM`. The order of registers is reversed depending on
    /// whether the address register is pre-decremented or post-incremented.
    uint16_t reglist;

    /// A control register
    uint16_t control_reg;
} m68k_operand_data;

typedef struct m68k_operand {
  m68k_operand_kind kind;
  m68k_operand_data data;
} m68k_operand;

/// A CPU condition code from the CCR
typedef enum m68k_condition_code {
    /// Always True
    M68K_CC_T  = 0,  // 0000
    /// Always False
    M68K_CC_F  = 1,  // 0001
    /// High
    M68K_CC_HI = 2,  // 0010
    /// Low or Same
    M68K_CC_LS = 3,  // 0011
    /// Carry Clear
    M68K_CC_CC = 4,  // 0100
    /// Carry Set
    M68K_CC_CS = 5,  // 0101
    /// Not Equal
    M68K_CC_NE = 6,  // 0110
    /// Equal
    M68K_CC_EQ = 7,  // 0111
    /// Overflow Clear
    M68K_CC_VC = 8,  // 1000
    /// Overflow Set
    M68K_CC_VS = 9,  // 1001
    /// Plus
    M68K_CC_PL = 10, // 1010
    /// Negative
    M68K_CC_MI = 11, // 1011
    /// Greater or Equal
    M68K_CC_GE = 12, // 1100
    /// Less
    M68K_CC_LT = 13, // 1101
    /// Greater
    M68K_CC_GT = 14, // 1110
    /// Less or Equal
    M68K_CC_LE = 15, // 1111
} m68k_condition_code;

/// A FPU condition code
typedef enum m68k_fp_condition_code {
    /// False
    M68K_FPCC_F    = 0,  // 000000
    /// Equal
    M68K_FPCC_EQ   = 1,  // 000001
    /// Ordered Greater Than
    M68K_FPCC_OGT  = 2,  // 000010
    /// Ordered Greater Than or Equal
    M68K_FPCC_OGE  = 3,  // 000011
    /// Ordered Less Than
    M68K_FPCC_OLT  = 4,  // 000100
    /// Ordered Less Than or Equal
    M68K_FPCC_OLE  = 5,  // 000101
    /// Ordered Greater Than or Less Than
    M68K_FPCC_OGL  = 6,  // 000110,
    /// Ordered
    M68K_FPCC_OR   = 7,  // 000111,
    /// Unordered
    M68K_FPCC_UN   = 8,  // 001000,
    /// Unordered or Equal
    M68K_FPCC_UEQ  = 9,  // 001001,
    /// Unordered or Greater Than
    M68K_FPCC_UGT  = 10, // 001010,
    /// Unordered or Greater Than or Equal
    M68K_FPCC_UGE  = 11, // 001011,
    /// Unordered or Less Than
    M68K_FPCC_ULT  = 12, // 001100,
    /// Unordered or Less Than or Equal
    M68K_FPCC_ULE  = 13, // 001101,
    /// Not Equal
    M68K_FPCC_NE   = 14, // 001110,
    /// True
    M68K_FPCC_T    = 15, // 001111,
    /// Signaling False
    M68K_FPCC_SF   = 16, // 010000,
    /// Signaling Equal
    M68K_FPCC_SEQ  = 17, // 010001,
    /// Greater Than
    M68K_FPCC_GT   = 18, // 010010,
    /// Greater Than or Equal
    M68K_FPCC_GE   = 19, // 010011,
    /// Less Than
    M68K_FPCC_LT   = 20, // 010100,
    /// Less Than or Equal
    M68K_FPCC_LE   = 21, // 010101,
    /// Greater Than or Less Than
    M68K_FPCC_GL   = 22, // 010110,
    /// Greater Than or Less Than or Equal
    M68K_FPCC_GLE  = 23, // 010111,
    /// Not (Greater Than or Less Than or Equal)
    M68K_FPCC_NGLE = 24, // 011000,
    /// Not (Greater Than or Less Than)
    M68K_FPCC_NGL  = 25, // 011001,
    /// Not (Less Than or Equal)
    M68K_FPCC_NLE  = 26, // 011010,
    /// Not (Less Than)
    M68K_FPCC_NLT  = 27, // 011011,
    /// Not (Greater Than or Equal)
    M68K_FPCC_NGE  = 28, // 011100,
    /// Not (Greater Than)
    M68K_FPCC_NGT  = 29, // 011101,
    /// Signaling Not Equal
    M68K_FPCC_SNE  = 30, // 011110,
    /// Signaling True
    M68K_FPCC_ST   = 31, // 011111,
} m68k_fp_condition_code;

/// Indicates the floating point format in memory for a FPU operation
typedef enum m68k_fpformat {
    /// The memory operand is a 32-bit integer.
    M68K_FPF_LONG_INT,
    /// The memory operand is a IEEE 32-bit single.
    M68K_FPF_SINGLE,
    /// The memory operand is a 68k extended-precision real (10 bytes).
    M68K_FPF_EXTENDED_REAL,
    /// The memory operand is a 68k packed decimal real (BCD encoded) (12 bytes).
    /// For `FMOVE`, this also includes a static _K-factor_ to be applied to the result.
    M68K_FPF_PACKED_DECIMAL_REAL_STATIC,
    /// The memory operand is a 68k packed decimal real (BCD encoded) (12 bytes).
    /// For `FMOVE`, this also includes a dynamic _K-factor_ to be applied to the result.
    M68K_FPF_PACKED_DECIMAL_REAL_DYNAMIC,
    /// The memory operand is a 16-bit integer.
    M68K_FPF_WORD_INT,
    /// The memory operand is a IEEE 64-bit double.
    M68K_FPF_DOUBLE,
    /// The memory operand is an 8-bit integer.
    M68K_FPF_BYTE_INT
} m68k_fpformat;

enum m68k_bitfield_flags {
  /// If set, bitfield offset specifies a data register and not a static value.
  M68K_BITFIELD_DYNAMIC_OFFSET = 1 << 0,
  /// If set, bitfield width specifies a data register and not a static value.
  M68K_BITFIELD_DYNAMIC_WIDTH = 1 << 1,
};

/// Additional attributes for an instruction that don't fit anywhere else.
typedef union m68k_extra_data {
  /// For 68020+ bitfield instructions, specifies the bitfield offset and width of the EA.
  struct {
    /// See m68k_bitfield_flags
    int8_t flags;
    int8_t offset;
    int8_t width;
  } bitfield;

  /// For instructions involving a CPU condition code (e.g. `DBcc`), specifies the condition code
  /// tested.
  m68k_condition_code cc;

  /// For instructions involving a FPU condition code (e.g. `FDBcc`), specifies the condition code
  /// tested.
  m68k_fp_condition_code fp_cc;

  /// Specifies the pack adjustment for BCD packing.
  uint16_t pack_adjustment;

  /// Specifies the float format of a memory operand for FPU instructions that use a memory EA.
  struct {
    m68k_fpformat fp_format;
    int k_factor; // register index for some formats, static bias for others
  } fp_data;
} m68k_extra_data;

typedef enum m68k_extra_kind {
  M68K_NO_EXTRA,
  M68K_EXTRA_BITFIELD,
  M68K_EXTRA_CC,
  M68K_EXTRA_FPCC,
  M68K_EXTRA_PACKADJ,
  M68K_EXTRA_FPFORMAT,
} m68k_extra_kind;

typedef struct m68k_extra {
  m68k_extra_kind kind;
  m68k_extra_data data;
} m68k_extra;

/// Represents an instruction.
typedef struct m68k_instruction {
    /// The size of any data movement (0 for unsized, 1, 2, 4).
    int size;

    /// The instruction itself.
    m68k_operation operation;

    /// The two operands involved, source and destination.
    m68k_operand operands[2];

    /// Any additional data required to make sense of the operation.
    m68k_extra extra;
} m68k_instruction;

/// Represents the result of decoding an instruction from a byte stream.
typedef struct m68k_decoded_instruction {
    /// The number of bytes that were consumed decoding the instruction.
    int bytes_used;
    /// The decoded instruction.
    m68k_instruction instruction;
} m68k_decoded_instruction;

/// Enumerates errors that can happen while decoding instructions.
typedef enum m68k_decoding_error {
  /// Everything OK
  M68K_NO_ERROR = 0,
  /// The instruction is not implemented in the decoder.
  M68K_NOT_IMPLEMENTED = -10000,
  /// The code stream doesn't contain enough data to decode the instruction.
  M68K_OUT_OF_SPACE,
  /// An illegal register was specified in the instruction encoding.
  M68K_BAD_REGISTER,
  /// An illegal size was specified in the instruction encoding.
  M68K_BAD_SIZE,
  /// A reserved case was hit in the instruction encoding.
  M68K_RESERVED,
} m68k_decoding_error;


/// Attempt to decode a single M68000 instruction starting at the specified byte
m68k_decoding_error m68k_decode(const uint8_t *code, uint32_t max_bytes, m68k_decoded_instruction *result);

/// Compare two instructions, equivalent to memcmp()
int m68k_compare_instructions(const m68k_instruction* lhs, const m68k_instruction *rhs);

/// Print an instruction.
void m68k_print_instruction(FILE *stream, const m68k_instruction *i);
