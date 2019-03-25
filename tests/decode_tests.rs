// auto-generated from decoding_tests.txt by gen_decoding_tests.py
mod tests {
  use m68kdecode::*;
  mod support;
  use support::*;
//  move.b d0,d1
#[test]
fn test_decode_0001_move_b_d0_d1() {
test_decoding_result_ok(&[0x12, 0x00],  Instruction { size: 1, operation: MOVE, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" move.b d0,d1",
]);}
//  move.b d2,d3
#[test]
fn test_decode_0002_move_b_d2_d3() {
test_decoding_result_ok(&[0x16, 0x02],  Instruction { size: 1, operation: MOVE, operands: [ DR(D2), DR(D3) ], extra: NoExtra }
, &[" move.b d2,d3",
]);}
//  move.b d4,d5
#[test]
fn test_decode_0003_move_b_d4_d5() {
test_decoding_result_ok(&[0x1a, 0x04],  Instruction { size: 1, operation: MOVE, operands: [ DR(D4), DR(D5) ], extra: NoExtra }
, &[" move.b d4,d5",
]);}
//  move.b d6,d7
#[test]
fn test_decode_0004_move_b_d6_d7() {
test_decoding_result_ok(&[0x1e, 0x06],  Instruction { size: 1, operation: MOVE, operands: [ DR(D6), DR(D7) ], extra: NoExtra }
, &[" move.b d6,d7",
]);}
//  move.w a0,a1
#[test]
fn test_decode_0005_move_w_a0_a1() {
test_decoding_result_ok(&[0x32, 0x48],  Instruction { size: 2, operation: MOVEA, operands: [ AR(A0), AR(A1) ], extra: NoExtra }
, &[" move.w a0,a1",
]);}
//  move.w a2,a3
#[test]
fn test_decode_0006_move_w_a2_a3() {
test_decoding_result_ok(&[0x36, 0x4a],  Instruction { size: 2, operation: MOVEA, operands: [ AR(A2), AR(A3) ], extra: NoExtra }
, &[" move.w a2,a3",
]);}
//  move.w a4,a5
#[test]
fn test_decode_0007_move_w_a4_a5() {
test_decoding_result_ok(&[0x3a, 0x4c],  Instruction { size: 2, operation: MOVEA, operands: [ AR(A4), AR(A5) ], extra: NoExtra }
, &[" move.w a4,a5",
]);}
//  move.w a6,a7
#[test]
fn test_decode_0008_move_w_a6_a7() {
test_decoding_result_ok(&[0x3e, 0x4e],  Instruction { size: 2, operation: MOVEA, operands: [ AR(A6), AR(A7) ], extra: NoExtra }
, &[" move.w a6,a7",
]);}
//  move.b 123(a0,d0),d3
#[test]
fn test_decode_0009_move_b_123_a0_d0_d3() {
test_decoding_result_ok(&[0x16, 0x30, 0x00, 0x7b],  Instruction { size: 1, operation: MOVE, operands: [ ARDISP(A0, dr_disp(D0, 123)), DR(D3) ], extra: NoExtra }
, &[" move.b 123(a0,d0),d3",
]);}
//  move.w 123(a0,d0),d3
#[test]
fn test_decode_0010_move_w_123_a0_d0_d3() {
test_decoding_result_ok(&[0x36, 0x30, 0x00, 0x7b],  Instruction { size: 2, operation: MOVE, operands: [ ARDISP(A0, dr_disp(D0, 123)), DR(D3) ], extra: NoExtra }
, &[" move.w 123(a0,d0),d3",
]);}
//  move.l 123(a0,d0),d3
#[test]
fn test_decode_0011_move_l_123_a0_d0_d3() {
test_decoding_result_ok(&[0x26, 0x30, 0x00, 0x7b],  Instruction { size: 4, operation: MOVE, operands: [ ARDISP(A0, dr_disp(D0, 123)), DR(D3) ], extra: NoExtra }
, &[" move.l 123(a0,d0),d3",
]);}
//  move.l 123(a0,d0),a1
#[test]
fn test_decode_0012_move_l_123_a0_d0_a1() {
test_decoding_result_ok(&[0x22, 0x70, 0x00, 0x7b],  Instruction { size: 4, operation: MOVEA, operands: [ ARDISP(A0, dr_disp(D0, 123)), AR(A1) ], extra: NoExtra }
, &[" move.l 123(a0,d0),a1",
]);}
//  move.w 123(a0,d0),a1
#[test]
fn test_decode_0013_move_w_123_a0_d0_a1() {
test_decoding_result_ok(&[0x32, 0x70, 0x00, 0x7b],  Instruction { size: 2, operation: MOVEA, operands: [ ARDISP(A0, dr_disp(D0, 123)), AR(A1) ], extra: NoExtra }
, &[" move.w 123(a0,d0),a1",
]);}
//  move.b #$12,d7
#[test]
fn test_decode_0014_move_b_12_d7() {
test_decoding_result_ok(&[0x1e, 0x3c, 0x00, 0x12],  Instruction { size: 1, operation: MOVE, operands: [ IMM8(0x12), DR(D7) ], extra: NoExtra }
, &[" move.b #$12,d7",
]);}
//  move.w #$1234,d7
#[test]
fn test_decode_0015_move_w_1234_d7() {
test_decoding_result_ok(&[0x3e, 0x3c, 0x12, 0x34],  Instruction { size: 2, operation: MOVE, operands: [ IMM16(0x1234), DR(D7) ], extra: NoExtra }
, &[" move.w #$1234,d7",
]);}
//  move.l #$12345678,d7
#[test]
fn test_decode_0016_move_l_12345678_d7() {
test_decoding_result_ok(&[0x2e, 0x3c, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: MOVE, operands: [ IMM32(0x12345678), DR(D7) ], extra: NoExtra }
, &[" move.l #$12345678,d7",
]);}
//  move.l D1,-(A2)
#[test]
fn test_decode_0017_move_l_d1_a2_() {
test_decoding_result_ok(&[0x25, 0x01],  Instruction { size: 4, operation: MOVE, operands: [ DR(D1), ARDEC(A2) ], extra: NoExtra }
, &[" move.l D1,-(A2)",
]);}
//  move.l D1,(A2)+
#[test]
fn test_decode_0018_move_l_d1_a2_() {
test_decoding_result_ok(&[0x24, 0xc1],  Instruction { size: 4, operation: MOVE, operands: [ DR(D1), ARINC(A2) ], extra: NoExtra }
, &[" move.l D1,(A2)+",
]);}
//  move.l -(A4),(A2)+
#[test]
fn test_decode_0019_move_l_a4_a2_() {
test_decoding_result_ok(&[0x24, 0xe4],  Instruction { size: 4, operation: MOVE, operands: [ ARDEC(A4), ARINC(A2) ], extra: NoExtra }
, &[" move.l -(A4),(A2)+",
]);}
//  move.l 4.w,A0
#[test]
fn test_decode_0020_move_l_4_w_a0() {
test_decoding_result_ok(&[0x20, 0x78, 0x00, 0x04],  Instruction { size: 4, operation: MOVEA, operands: [ ABS16(4), AR(A0) ], extra: NoExtra }
, &[" move.l 4.w,A0",
]);}
//  move.l $11223344,A0
#[test]
fn test_decode_0021_move_l_11223344_a0() {
test_decoding_result_ok(&[0x20, 0x79, 0x11, 0x22, 0x33, 0x44],  Instruction { size: 4, operation: MOVEA, operands: [ ABS32(0x11223344), AR(A0) ], extra: NoExtra }
, &[" move.l $11223344,A0",
]);}
//  move.w #$1234,123(d0)
#[test]
fn test_decode_0022_move_w_1234_123_d0_() {
test_decoding_result_ok(&[0x31, 0xbc, 0x12, 0x34, 0x01, 0xa0, 0x00, 0x7b],  Instruction {
   size: 2, operation: MOVE, operands: [
     IMM16(0x1234),
     DISP(Displacement {
       base_displacement: 123,
       outer_displacement: 0,
       indexer: Indexer::DR(D0, 0),
       indirection: NoIndirection,
      })
    ],
    extra: NoExtra
  }
, &[" move.w #$1234,123(d0)",
]);}
//  move.w -8(pc),d3
#[test]
fn test_decode_0023_move_w_8_pc_d3() {
test_decoding_result_ok(&[0x36, 0x3a, 0xff, 0xf8],  Instruction {
   size: 2, operation: MOVE, operands: [
     PCDISP(2, Displacement {
       base_displacement: -8,
       outer_displacement: 0,
       indexer: Indexer::NoIndexer,
       indirection: NoIndirection,
     }),
     DR(D3),
   ],
   extra: NoExtra
 }
, &[" move.w -8(pc),d3",
]);}
//  move.w -8(pc,d2*8),d3
#[test]
fn test_decode_0024_move_w_8_pc_d2_8_d3() {
test_decoding_result_ok(&[0x36, 0x3b, 0x26, 0xf8],  Instruction {
   size: 2, operation: MOVE, operands: [
     PCDISP(2, Displacement {
       base_displacement: -8,
       outer_displacement: 0,
       indexer: Indexer::DR(D2, 3),
       indirection: NoIndirection,
     }),
     DR(D3),
   ],
   extra: NoExtra
 }
, &[" move.w -8(pc,d2*8),d3",
]);}
//  move.w 123(a1,d2*4),9876(a2,d3*2)
#[test]
fn test_decode_0025_move_w_123_a1_d2_4_9876_a2_d3_2_() {
test_decoding_result_ok(&[0x35, 0xb1, 0x24, 0x7b, 0x33, 0x20, 0x26, 0x94],  Instruction {
   size: 2,
   operation: MOVE,
   operands: [
     ARDISP(A1, dr_disp_scale(D2, 123, 2)),
     ARDISP(A2, dr_disp_scale(D3, 9876, 1))
   ],
   extra: NoExtra
 }
, &[" move.w 123(a1,d2*4),9876(a2,d3*2)",
]);}
//  move.w d0,12345(a0,a1*2)
#[test]
fn test_decode_0026_move_w_d0_12345_a0_a1_2_() {
test_decoding_result_ok(&[0x31, 0x80, 0x93, 0x20, 0x30, 0x39],  Instruction {
   size: 2,
   operation: MOVE,
   operands: [
     DR(D0),
     ARDISP(A0, Displacement {
       base_displacement: 12345,
       outer_displacement: 0,
       indexer: Indexer::AR(A1, 1),
       indirection: NoIndirection,
     })
   ],
   extra: NoExtra
 }
, &[" move.w d0,12345(a0,a1*2)",
]);}
//  lea (a0),a1
#[test]
fn test_decode_0027_lea_a0_a1() {
test_decoding_result_ok(&[0x43, 0xd0],  Instruction { size: 4, operation: LEA, operands: [ ARIND(A0), AR(A1) ], extra: NoExtra }
, &[" lea (a0),a1",
]);}
//  lea 8(a0),a1
#[test]
fn test_decode_0028_lea_8_a0_a1() {
test_decoding_result_ok(&[0x43, 0xe8, 0x00, 0x08],  Instruction { size: 4, operation: LEA, operands: [ ARDISP(A0, simple_disp(8)), AR(A1) ], extra: NoExtra  }
, &[" lea 8(a0),a1",
]);}
//  ori #17,ccr
#[test]
fn test_decode_0029_ori_17_ccr() {
test_decoding_result_ok(&[0x00, 0x3c, 0x00, 0x11],  Instruction { size: 1, operation: ORITOCCR, operands: [ IMM8(17), Implied ], extra: NoExtra  }
, &[" ori #17,ccr",
]);}
//  ori #$1234,sr
#[test]
fn test_decode_0030_ori_1234_sr() {
test_decoding_result_ok(&[0x00, 0x7c, 0x12, 0x34],  Instruction { size: 2, operation: ORITOSR, operands: [ IMM16(0x1234), Implied ], extra: NoExtra  }
, &[" ori #$1234,sr",
]);}
//  ori.w #$1234,d0
#[test]
fn test_decode_0031_ori_w_1234_d0() {
test_decoding_result_ok(&[0x00, 0x40, 0x12, 0x34],  Instruction { size: 2, operation: ORI, operands: [ IMM16(0x1234), DR(D0) ], extra: NoExtra  }
, &[" ori.w #$1234,d0",
]);}
//  ori.b #$12,d2
#[test]
fn test_decode_0032_ori_b_12_d2() {
test_decoding_result_ok(&[0x00, 0x02, 0x00, 0x12],  Instruction { size: 1, operation: ORI, operands: [ IMM8(0x12), DR(D2) ], extra: NoExtra  }
, &[" ori.b #$12,d2",
]);}
//  ori.w #$1234,123(a0,d0)
#[test]
fn test_decode_0033_ori_w_1234_123_a0_d0_() {
test_decoding_result_ok(&[0x00, 0x70, 0x12, 0x34, 0x00, 0x7b],  Instruction { size: 2, operation: ORI, operands: [ IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123)) ], extra: NoExtra  }
, &[" ori.w #$1234,123(a0,d0)",
]);}
//  ori.l #$12345678,-(a0)
#[test]
fn test_decode_0034_ori_l_12345678_a0_() {
test_decoding_result_ok(&[0x00, 0xa0, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: ORI, operands: [ IMM32(0x12345678), ARDEC(A0) ], extra: NoExtra  }
, &[" ori.l #$12345678,-(a0)",
]);}
//  andi #17,ccr
#[test]
fn test_decode_0035_andi_17_ccr() {
test_decoding_result_ok(&[0x02, 0x3c, 0x00, 0x11],  Instruction { size: 1, operation: ANDITOCCR, operands: [ IMM8(17), Implied ], extra: NoExtra  }
, &[" andi #17,ccr",
]);}
//  andi #$1234,sr
#[test]
fn test_decode_0036_andi_1234_sr() {
test_decoding_result_ok(&[0x02, 0x7c, 0x12, 0x34],  Instruction { size: 2, operation: ANDITOSR, operands: [ IMM16(0x1234), Implied ], extra: NoExtra  }
, &[" andi #$1234,sr",
]);}
//  andi.w #$1234,d0
#[test]
fn test_decode_0037_andi_w_1234_d0() {
test_decoding_result_ok(&[0x02, 0x40, 0x12, 0x34],  Instruction { size: 2, operation: ANDI, operands: [ IMM16(0x1234), DR(D0) ], extra: NoExtra  }
, &[" andi.w #$1234,d0",
]);}
//  andi.b #$12,d2
#[test]
fn test_decode_0038_andi_b_12_d2() {
test_decoding_result_ok(&[0x02, 0x02, 0x00, 0x12],  Instruction { size: 1, operation: ANDI, operands: [ IMM8(0x12), DR(D2) ], extra: NoExtra  }
, &[" andi.b #$12,d2",
]);}
//  andi.w #$1234,123(a0,d0)
#[test]
fn test_decode_0039_andi_w_1234_123_a0_d0_() {
test_decoding_result_ok(&[0x02, 0x70, 0x12, 0x34, 0x00, 0x7b],  Instruction { size: 2, operation: ANDI, operands: [ IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123)) ], extra: NoExtra  }
, &[" andi.w #$1234,123(a0,d0)",
]);}
//  andi.l #$12345678,-(a0)
#[test]
fn test_decode_0040_andi_l_12345678_a0_() {
test_decoding_result_ok(&[0x02, 0xa0, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: ANDI, operands: [ IMM32(0x12345678), ARDEC(A0) ], extra: NoExtra  }
, &[" andi.l #$12345678,-(a0)",
]);}
//  eori #17,ccr
#[test]
fn test_decode_0041_eori_17_ccr() {
test_decoding_result_ok(&[0x0a, 0x3c, 0x00, 0x11],  Instruction { size: 1, operation: EORITOCCR, operands: [ IMM8(17), Implied ], extra: NoExtra  }
, &[" eori #17,ccr",
]);}
//  eori #$1234,sr
#[test]
fn test_decode_0042_eori_1234_sr() {
test_decoding_result_ok(&[0x0a, 0x7c, 0x12, 0x34],  Instruction { size: 2, operation: EORITOSR, operands: [ IMM16(0x1234), Implied ], extra: NoExtra  }
, &[" eori #$1234,sr",
]);}
//  eori.w #$1234,d0
#[test]
fn test_decode_0043_eori_w_1234_d0() {
test_decoding_result_ok(&[0x0a, 0x40, 0x12, 0x34],  Instruction { size: 2, operation: EORI, operands: [ IMM16(0x1234), DR(D0) ], extra: NoExtra  }
, &[" eori.w #$1234,d0",
]);}
//  eori.b #$12,d2
#[test]
fn test_decode_0044_eori_b_12_d2() {
test_decoding_result_ok(&[0x0a, 0x02, 0x00, 0x12],  Instruction { size: 1, operation: EORI, operands: [ IMM8(0x12), DR(D2) ], extra: NoExtra  }
, &[" eori.b #$12,d2",
]);}
//  eori.w #$1234,123(a0,d0)
#[test]
fn test_decode_0045_eori_w_1234_123_a0_d0_() {
test_decoding_result_ok(&[0x0a, 0x70, 0x12, 0x34, 0x00, 0x7b],  Instruction { size: 2, operation: EORI, operands: [ IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123)) ], extra: NoExtra  }
, &[" eori.w #$1234,123(a0,d0)",
]);}
//  eori.l #$12345678,-(a0)
#[test]
fn test_decode_0046_eori_l_12345678_a0_() {
test_decoding_result_ok(&[0x0a, 0xa0, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: EORI, operands: [ IMM32(0x12345678), ARDEC(A0) ], extra: NoExtra }
, &[" eori.l #$12345678,-(a0)",
]);}
//  addi.l #$12345678,-(a0)
#[test]
fn test_decode_0047_addi_l_12345678_a0_() {
test_decoding_result_ok(&[0x06, 0xa0, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: ADDI, operands: [ IMM32(0x12345678), ARDEC(A0) ], extra: NoExtra }
, &[" addi.l #$12345678,-(a0)",
]);}
//  subi.l #$12345678,-(a0)
#[test]
fn test_decode_0048_subi_l_12345678_a0_() {
test_decoding_result_ok(&[0x04, 0xa0, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: SUBI, operands: [ IMM32(0x12345678), ARDEC(A0) ], extra: NoExtra }
, &[" subi.l #$12345678,-(a0)",
]);}
//  rtm d3
#[test]
fn test_decode_0049_rtm_d3() {
test_decoding_result_ok(&[0x06, 0xc3],  Instruction { size: 0, operation: RTM, operands: [ DR(D3), NoOperand ], extra: NoExtra }
, &[" rtm d3",
]);}
//  rtm a1
#[test]
fn test_decode_0050_rtm_a1() {
test_decoding_result_ok(&[0x06, 0xc9],  Instruction { size: 0, operation: RTM, operands: [ AR(A1), NoOperand ], extra: NoExtra }
, &[" rtm a1",
]);}
//  callm #3,(a1)
#[test]
fn test_decode_0051_callm_3_a1_() {
test_decoding_result_ok(&[0x06, 0xd1, 0x00, 0x03],  Instruction { size: 0, operation: CALLM, operands: [ IMM8(3), ARIND(A1) ], extra: NoExtra }
, &[" callm #3,(a1)",
]);}
//  callm #99,$12345678
#[test]
fn test_decode_0052_callm_99_12345678() {
test_decoding_result_ok(&[0x06, 0xf9, 0x00, 0x63, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 0, operation: CALLM, operands: [ IMM8(99), ABS32(0x12345678) ], extra: NoExtra }
, &[" callm #99,$12345678",
]);}
//  cmp2.l (a0),d3
#[test]
fn test_decode_0053_cmp2_l_a0_d3() {
test_decoding_result_ok(&[0x04, 0xd0, 0x30, 0x00],  Instruction { size: 4, operation: CMP2, operands: [ ARIND(A0), DR(D3) ], extra: NoExtra }
, &[" cmp2.l (a0),d3",
]);}
//  cmp2.b 90(a0,d2),a6
#[test]
fn test_decode_0054_cmp2_b_90_a0_d2_a6() {
test_decoding_result_ok(&[0x00, 0xf0, 0xe0, 0x00, 0x20, 0x5a],  Instruction { size: 1, operation: CMP2, operands: [ ARDISP(A0, dr_disp(D2, 90)), AR(A6) ], extra: NoExtra }
, &[" cmp2.b 90(a0,d2),a6",
]);}
//  chk2.w 90(a0,d2),a6
#[test]
fn test_decode_0055_chk2_w_90_a0_d2_a6() {
test_decoding_result_ok(&[0x02, 0xf0, 0xe8, 0x00, 0x20, 0x5a],  Instruction { size: 2, operation: CHK2, operands: [ ARDISP(A0, dr_disp(D2, 90)), AR(A6) ], extra: NoExtra }
, &[" chk2.w 90(a0,d2),a6",
]);}
//  cmpi.b #$a5,90(a0,d2*4)
#[test]
fn test_decode_0056_cmpi_b_a5_90_a0_d2_4_() {
test_decoding_result_ok(&[0x0c, 0x30, 0x00, 0xa5, 0x24, 0x5a],  Instruction { size: 1, operation: CMPI, operands: [ IMM8(0xa5), ARDISP(A0, dr_disp_scale(D2, 90, 2)) ], extra: NoExtra }
, &[" cmpi.b #$a5,90(a0,d2*4)",
]);}
//  cmpi.w #$a512,90(a0,d2*4)
#[test]
fn test_decode_0057_cmpi_w_a512_90_a0_d2_4_() {
test_decoding_result_ok(&[0x0c, 0x70, 0xa5, 0x12, 0x24, 0x5a],  Instruction { size: 2, operation: CMPI, operands: [ IMM16(0xa512), ARDISP(A0, dr_disp_scale(D2, 90, 2)) ], extra: NoExtra }
, &[" cmpi.w #$a512,90(a0,d2*4)",
]);}
//  cmpi.l #$12345678,90(a0,d2*4)
#[test]
fn test_decode_0058_cmpi_l_12345678_90_a0_d2_4_() {
test_decoding_result_ok(&[0x0c, 0xb0, 0x12, 0x34, 0x56, 0x78, 0x24, 0x5a],  Instruction { size: 4, operation: CMPI, operands: [ IMM32(0x12345678), ARDISP(A0, dr_disp_scale(D2, 90, 2)) ], extra: NoExtra }
, &[" cmpi.l #$12345678,90(a0,d2*4)",
]);}
//  btst #18,d0
#[test]
fn test_decode_0059_btst_18_d0() {
test_decoding_result_ok(&[0x08, 0x00, 0x00, 0x12],  Instruction { size: 1, operation: BTST, operands: [ IMM16(18), DR(D0) ], extra: NoExtra }
, &[" btst #18,d0",
]);}
//  btst #18,(a0)+
#[test]
fn test_decode_0060_btst_18_a0_() {
test_decoding_result_ok(&[0x08, 0x18, 0x00, 0x12],  Instruction { size: 1, operation: BTST, operands: [ IMM16(18), ARINC(A0) ], extra: NoExtra }
, &[" btst #18,(a0)+",
]);}
//  bclr #18,(a0)+
#[test]
fn test_decode_0061_bclr_18_a0_() {
test_decoding_result_ok(&[0x08, 0x98, 0x00, 0x12],  Instruction { size: 1, operation: BCLR, operands: [ IMM16(18), ARINC(A0) ], extra: NoExtra }
, &[" bclr #18,(a0)+",
]);}
//  bchg #18,(a0)+
#[test]
fn test_decode_0062_bchg_18_a0_() {
test_decoding_result_ok(&[0x08, 0x58, 0x00, 0x12],  Instruction { size: 1, operation: BCHG, operands: [ IMM16(18), ARINC(A0) ], extra: NoExtra }
, &[" bchg #18,(a0)+",
]);}
//  bset #18,(a0)+
#[test]
fn test_decode_0063_bset_18_a0_() {
test_decoding_result_ok(&[0x08, 0xd8, 0x00, 0x12],  Instruction { size: 1, operation: BSET, operands: [ IMM16(18), ARINC(A0) ], extra: NoExtra }
, &[" bset #18,(a0)+",
]);}
//  moves.l a0,(a1)
#[test]
fn test_decode_0064_moves_l_a0_a1_() {
test_decoding_result_ok(&[0x0e, 0x91, 0x88, 0x00],  Instruction { size: 4, operation: MOVES, operands: [ AR(A0), ARIND(A1) ], extra: NoExtra }
, &[" moves.l a0,(a1)",
]);}
//  moves.b d0,(a1)
#[test]
fn test_decode_0065_moves_b_d0_a1_() {
test_decoding_result_ok(&[0x0e, 0x11, 0x08, 0x00],  Instruction { size: 1, operation: MOVES, operands: [ DR(D0), ARIND(A1) ], extra: NoExtra }
, &[" moves.b d0,(a1)",
]);}
//  cas d0,d1,(a0)
#[test]
fn test_decode_0066_cas_d0_d1_a0_() {
test_decoding_result_err(&[0x0c, 0xd0, 0x00, 0x40],  NotImplemented
, &[" cas d0,d1,(a0)",
]);}
//  cas2 d0:d1,d2:d3,(a0):(a1)
#[test]
fn test_decode_0067_cas2_d0_d1_d2_d3_a0_a1_() {
test_decoding_result_err(&[0x0c, 0xfc, 0x80, 0x80, 0x90, 0xc1],  NotImplemented
, &[" cas2 d0:d1,d2:d3,(a0):(a1)",
]);}
//  illegal
#[test]
fn test_decode_0068_illegal() {
test_decoding_result_ok(&[0x4a, 0xfc],  Instruction { size: 0, operation: ILLEGAL, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" illegal",
]);}
//  nop
#[test]
fn test_decode_0069_nop() {
test_decoding_result_ok(&[0x4e, 0x71],  Instruction { size: 0, operation: NOP, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" nop",
]);}
//  reset
#[test]
fn test_decode_0070_reset() {
test_decoding_result_ok(&[0x4e, 0x70],  Instruction { size: 0, operation: RESET, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" reset",
]);}
//  rtd #578
#[test]
fn test_decode_0071_rtd_578() {
test_decoding_result_ok(&[0x4e, 0x74, 0x02, 0x42],  Instruction { size: 0, operation: RTD, operands: [ IMM16(578), NoOperand ], extra: NoExtra }
, &[" rtd #578",
]);}
//  rte
#[test]
fn test_decode_0072_rte() {
test_decoding_result_ok(&[0x4e, 0x73],  Instruction { size: 0, operation: RTE, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" rte",
]);}
//  rtr
#[test]
fn test_decode_0073_rtr() {
test_decoding_result_ok(&[0x4e, 0x77],  Instruction { size: 0, operation: RTR, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" rtr",
]);}
//  rts
#[test]
fn test_decode_0074_rts() {
test_decoding_result_ok(&[0x4e, 0x75],  Instruction { size: 0, operation: RTS, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" rts",
]);}
//  stop #123
#[test]
fn test_decode_0075_stop_123() {
test_decoding_result_ok(&[0x4e, 0x72, 0x00, 0x7b],  Instruction { size: 0, operation: STOP, operands: [ IMM16(123), NoOperand ], extra: NoExtra }
, &[" stop #123",
]);}
//  trapv
#[test]
fn test_decode_0076_trapv() {
test_decoding_result_ok(&[0x4e, 0x76],  Instruction { size: 0, operation: TRAPV, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" trapv",
]);}
//  swap d7
#[test]
fn test_decode_0077_swap_d7() {
test_decoding_result_ok(&[0x48, 0x47],  Instruction { size: 0, operation: SWAP, operands: [ DR(D7), NoOperand ], extra: NoExtra }
, &[" swap d7",
]);}
//  bkpt #3
#[test]
fn test_decode_0078_bkpt_3() {
test_decoding_result_ok(&[0x48, 0x4b],  Instruction { size: 0, operation: BKPT, operands: [ IMM8(3), NoOperand ], extra: NoExtra }
, &[" bkpt #3",
]);}
//  ext.w d6
#[test]
fn test_decode_0079_ext_w_d6() {
test_decoding_result_ok(&[0x48, 0x86],  Instruction { size: 2, operation: EXTW, operands: [ DR(D6), NoOperand ], extra: NoExtra }
, &[" ext.w d6",
]);}
//  ext.l d6
#[test]
fn test_decode_0080_ext_l_d6() {
test_decoding_result_ok(&[0x48, 0xc6],  Instruction { size: 4, operation: EXTL, operands: [ DR(D6), NoOperand ], extra: NoExtra }
, &[" ext.l d6",
]);}
//  extb.l d6
#[test]
fn test_decode_0081_extb_l_d6() {
test_decoding_result_ok(&[0x49, 0xc6],  Instruction { size: 4, operation: EXTBL, operands: [ DR(D6), NoOperand ], extra: NoExtra }
, &[" extb.l d6",
]);}
//  link.w a0,#1234
#[test]
fn test_decode_0082_link_w_a0_1234() {
test_decoding_result_ok(&[0x4e, 0x50, 0x04, 0xd2],  Instruction { size: 2, operation: LINK, operands: [ AR(A0), IMM16(1234) ], extra: NoExtra }
, &[" link.w a0,#1234",
]);}
//  link.l a5,#$12345678
#[test]
fn test_decode_0083_link_l_a5_12345678() {
test_decoding_result_ok(&[0x48, 0x0d, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: LINK, operands: [ AR(A5), IMM32(0x12345678) ], extra: NoExtra }
, &[" link.l a5,#$12345678",
]);}
//  unlk a2
#[test]
fn test_decode_0084_unlk_a2() {
test_decoding_result_ok(&[0x4e, 0x5a],  Instruction { size: 0, operation: UNLK, operands: [ AR(A2), NoOperand ], extra: NoExtra }
, &[" unlk a2",
]);}
//  trap #15
#[test]
fn test_decode_0085_trap_15() {
test_decoding_result_ok(&[0x4e, 0x4f],  Instruction { size: 0, operation: TRAP, operands: [ IMM8(15), NoOperand ], extra: NoExtra }
, &[" trap #15",
]);}
//  divs.w (a1)+,d2
#[test]
fn test_decode_0086_divs_w_a1_d2() {
test_decoding_result_ok(&[0x85, 0xd9],  Instruction { size: 2, operation: DIVS, operands: [ ARINC(A1), DR(D2) ], extra: NoExtra }
, &[" divs.w (a1)+,d2",
]);}
//  divs.l d0,d2
#[test]
fn test_decode_0087_divs_l_d0_d2() {
test_decoding_result_ok(&[0x4c, 0x40, 0x28, 0x02],  Instruction { size: 4, operation: DIVSL, operands: [ DR(D0), DR(D2) ], extra: NoExtra }
, &[" divs.l d0,d2",
]);}
//  divs.l d0,d3:d2
#[test]
fn test_decode_0088_divs_l_d0_d3_d2() {
test_decoding_result_ok(&[0x4c, 0x40, 0x2c, 0x03],  Instruction { size: 4, operation: DIVSL, operands: [ DR(D0), DPAIR(D2, D3) ], extra: NoExtra }
, &[" divs.l d0,d3:d2",
]);}
//  divsl.l d0,d3:d2
#[test]
fn test_decode_0089_divsl_l_d0_d3_d2() {
test_decoding_result_ok(&[0x4c, 0x40, 0x28, 0x03],  Instruction { size: 4, operation: DIVSLL, operands: [ DR(D0), DPAIR(D2, D3) ], extra: NoExtra }
, &[" divsl.l d0,d3:d2",
]);}
//  divu.w (a1)+,d2
#[test]
fn test_decode_0090_divu_w_a1_d2() {
test_decoding_result_ok(&[0x84, 0xd9],  Instruction { size: 2, operation: DIVU, operands: [ ARINC(A1), DR(D2) ], extra: NoExtra }
, &[" divu.w (a1)+,d2",
]);}
//  divu.l d0,d2
#[test]
fn test_decode_0091_divu_l_d0_d2() {
test_decoding_result_ok(&[0x4c, 0x40, 0x20, 0x02],  Instruction { size: 4, operation: DIVUL, operands: [ DR(D0), DR(D2) ], extra: NoExtra }
, &[" divu.l d0,d2",
]);}
//  divu.l d0,d3:d2
#[test]
fn test_decode_0092_divu_l_d0_d3_d2() {
test_decoding_result_ok(&[0x4c, 0x40, 0x24, 0x03],  Instruction { size: 4, operation: DIVUL, operands: [ DR(D0), DPAIR(D2, D3) ], extra: NoExtra }
, &[" divu.l d0,d3:d2",
]);}
//  divul.l d0,d3:d2
#[test]
fn test_decode_0093_divul_l_d0_d3_d2() {
test_decoding_result_ok(&[0x4c, 0x40, 0x20, 0x03],  Instruction { size: 4, operation: DIVULL, operands: [ DR(D0), DPAIR(D2, D3) ], extra: NoExtra }
, &[" divul.l d0,d3:d2",
]);}
//  jmp (a0)
#[test]
fn test_decode_0094_jmp_a0_() {
test_decoding_result_ok(&[0x4e, 0xd0],  Instruction { size: 0, operation: JMP, operands: [ ARIND(A0), NoOperand ], extra: NoExtra }
, &[" jmp (a0)",
]);}
//  jmp $12345678
#[test]
fn test_decode_0095_jmp_12345678() {
test_decoding_result_ok(&[0x4e, 0xf9, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 0, operation: JMP, operands: [ ABS32(0x12345678), NoOperand ], extra: NoExtra }
, &[" jmp $12345678",
]);}
//  jmp 123(pc)
#[test]
fn test_decode_0096_jmp_123_pc_() {
test_decoding_result_ok(&[0x4e, 0xfa, 0x00, 0x7b],  Instruction {
   size: 0, operation: JMP, operands: [
     PCDISP(2, Displacement {
       base_displacement: 123,
       outer_displacement: 0,
       indexer: Indexer::NoIndexer,
       indirection: NoIndirection,
     }),
     NoOperand,
   ],
   extra: NoExtra,
 }
, &[" jmp 123(pc)",
]);}
//  jsr (a0)
#[test]
fn test_decode_0097_jsr_a0_() {
test_decoding_result_ok(&[0x4e, 0x90],  Instruction { size: 0, operation: JSR, operands: [ ARIND(A0), NoOperand ], extra: NoExtra }
, &[" jsr (a0)",
]);}
//  jsr $12345678
#[test]
fn test_decode_0098_jsr_12345678() {
test_decoding_result_ok(&[0x4e, 0xb9, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 0, operation: JSR, operands: [ ABS32(0x12345678), NoOperand ], extra: NoExtra }
, &[" jsr $12345678",
]);}
//  jsr 123(pc)
#[test]
fn test_decode_0099_jsr_123_pc_() {
test_decoding_result_ok(&[0x4e, 0xba, 0x00, 0x7b],  Instruction {
   size: 0, operation: JSR, operands: [
     PCDISP(2, Displacement {
       base_displacement: 123,
       outer_displacement: 0,
       indexer: Indexer::NoIndexer,
       indirection: NoIndirection,
     }),
     NoOperand,
   ],
   extra: NoExtra,
 }
, &[" jsr 123(pc)",
]);}
//  muls.w  d0,d1
#[test]
fn test_decode_0100_muls_w_d0_d1() {
test_decoding_result_ok(&[0xc3, 0xc0],  Instruction { size: 2, operation: MULS, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" muls.w  d0,d1",
]);}
//  muls.l  d0,d1
#[test]
fn test_decode_0101_muls_l_d0_d1() {
test_decoding_result_ok(&[0x4c, 0x00, 0x18, 0x01],  Instruction { size: 4, operation: MULS, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" muls.l  d0,d1",
]);}
//  muls.l  d0,d2:d1
#[test]
fn test_decode_0102_muls_l_d0_d2_d1() {
test_decoding_result_ok(&[0x4c, 0x00, 0x1c, 0x02],  Instruction { size: 4, operation: MULS, operands: [ DR(D0), DPAIR(D1, D2) ], extra: NoExtra }
, &[" muls.l  d0,d2:d1",
]);}
//  mulu.w  d0,d1
#[test]
fn test_decode_0103_mulu_w_d0_d1() {
test_decoding_result_ok(&[0xc2, 0xc0],  Instruction { size: 2, operation: MULU, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" mulu.w  d0,d1",
]);}
//  mulu.l  d0,d1
#[test]
fn test_decode_0104_mulu_l_d0_d1() {
test_decoding_result_ok(&[0x4c, 0x00, 0x10, 0x01],  Instruction { size: 4, operation: MULU, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" mulu.l  d0,d1",
]);}
//  mulu.l  d0,d2:d1
#[test]
fn test_decode_0105_mulu_l_d0_d2_d1() {
test_decoding_result_ok(&[0x4c, 0x00, 0x14, 0x02],  Instruction { size: 4, operation: MULU, operands: [ DR(D0), DPAIR(D1, D2) ], extra: NoExtra }
, &[" mulu.l  d0,d2:d1",
]);}
//  nbcd  (a0)+
#[test]
fn test_decode_0106_nbcd_a0_() {
test_decoding_result_ok(&[0x48, 0x18],  Instruction { size: 1, operation: NBCD, operands: [ ARINC(A0), NoOperand ], extra: NoExtra }
, &[" nbcd  (a0)+",
]);}
//  move sr,d0
#[test]
fn test_decode_0107_move_sr_d0() {
test_decoding_result_ok(&[0x40, 0xc0],  Instruction { size: 2, operation: MOVEFROMSR, operands: [ Implied, DR(D0) ], extra: NoExtra }
, &[" move sr,d0",
]);}
//  move d0,sr
#[test]
fn test_decode_0108_move_d0_sr() {
test_decoding_result_ok(&[0x46, 0xc0],  Instruction { size: 2, operation: MOVETOSR, operands: [ DR(D0), Implied ], extra: NoExtra }
, &[" move d0,sr",
]);}
//  move a0,usp
#[test]
fn test_decode_0109_move_a0_usp() {
test_decoding_result_ok(&[0x4e, 0x60],  Instruction { size: 4, operation: MOVETOUSP, operands: [ AR(A0), Implied ], extra: NoExtra }
, &[" move a0,usp",
]);}
//  move usp,a3
#[test]
fn test_decode_0110_move_usp_a3() {
test_decoding_result_ok(&[0x4e, 0x6b],  Instruction { size: 4, operation: MOVEFROMUSP, operands: [ Implied, AR(A3) ], extra: NoExtra }
, &[" move usp,a3",
]);}
//  move d0,ccr
#[test]
fn test_decode_0111_move_d0_ccr() {
test_decoding_result_ok(&[0x44, 0xc0],  Instruction { size: 2, operation: MOVETOCCR, operands: [ DR(D0), Implied ], extra: NoExtra }
, &[" move d0,ccr",
]);}
//  move ccr,d0
#[test]
fn test_decode_0112_move_ccr_d0() {
test_decoding_result_ok(&[0x42, 0xc0],  Instruction { size: 2, operation: MOVEFROMCCR, operands: [ Implied, DR(D0) ], extra: NoExtra }
, &[" move ccr,d0",
]);}
//  pea (a0)
#[test]
fn test_decode_0113_pea_a0_() {
test_decoding_result_ok(&[0x48, 0x50],  Instruction { size: 4, operation: PEA, operands: [ ARIND(A0), Implied ], extra: NoExtra }
, &[" pea (a0)",
]);}
//  movem.w d0-d4/a0-a2,-(a4)
#[test]
fn test_decode_0114_movem_w_d0_d4_a0_a2_a4_() {
test_decoding_result_ok(&[0x48, 0xa4, 0xf8, 0xe0],  Instruction { size: 2, operation: MOVEM, operands: [ REGLIST(0b1111100011100000), ARDEC(A4) ], extra: NoExtra }
, &[" movem.w d0-d4/a0-a2,-(a4)",
]);}
//  movem.l (a4)+,d0-d4/a0-a2
#[test]
fn test_decode_0115_movem_l_a4_d0_d4_a0_a2() {
test_decoding_result_ok(&[0x4c, 0xdc, 0x07, 0x1f],  Instruction { size: 4, operation: MOVEM, operands: [ ARINC(A4), REGLIST(0b0000011100011111) ], extra: NoExtra }
, &[" movem.l (a4)+,d0-d4/a0-a2",
]);}
//  clr.b d0
#[test]
fn test_decode_0116_clr_b_d0() {
test_decoding_result_ok(&[0x42, 0x00],  Instruction { size: 1, operation: CLR, operands: [ Implied, DR(D0) ], extra: NoExtra }
, &[" clr.b d0",
]);}
//  clr.w (a0)+
#[test]
fn test_decode_0117_clr_w_a0_() {
test_decoding_result_ok(&[0x42, 0x58],  Instruction { size: 2, operation: CLR, operands: [ Implied, ARINC(A0) ], extra: NoExtra }
, &[" clr.w (a0)+",
]);}
//  clr.l (a4)
#[test]
fn test_decode_0118_clr_l_a4_() {
test_decoding_result_ok(&[0x42, 0x94],  Instruction { size: 4, operation: CLR, operands: [ Implied, ARIND(A4) ], extra: NoExtra }
, &[" clr.l (a4)",
]);}
//  neg.b d0
#[test]
fn test_decode_0119_neg_b_d0() {
test_decoding_result_ok(&[0x44, 0x00],  Instruction { size: 1, operation: NEG, operands: [ Implied, DR(D0) ], extra: NoExtra }
, &[" neg.b d0",
]);}
//  neg.w (a0)+
#[test]
fn test_decode_0120_neg_w_a0_() {
test_decoding_result_ok(&[0x44, 0x58],  Instruction { size: 2, operation: NEG, operands: [ Implied, ARINC(A0) ], extra: NoExtra }
, &[" neg.w (a0)+",
]);}
//  neg.l (a4)
#[test]
fn test_decode_0121_neg_l_a4_() {
test_decoding_result_ok(&[0x44, 0x94],  Instruction { size: 4, operation: NEG, operands: [ Implied, ARIND(A4) ], extra: NoExtra }
, &[" neg.l (a4)",
]);}
//  negx.b d0
#[test]
fn test_decode_0122_negx_b_d0() {
test_decoding_result_ok(&[0x40, 0x00],  Instruction { size: 1, operation: NEGX, operands: [ Implied, DR(D0) ], extra: NoExtra }
, &[" negx.b d0",
]);}
//  negx.w (a0)+
#[test]
fn test_decode_0123_negx_w_a0_() {
test_decoding_result_ok(&[0x40, 0x58],  Instruction { size: 2, operation: NEGX, operands: [ Implied, ARINC(A0) ], extra: NoExtra }
, &[" negx.w (a0)+",
]);}
//  negx.l (a4)
#[test]
fn test_decode_0124_negx_l_a4_() {
test_decoding_result_ok(&[0x40, 0x94],  Instruction { size: 4, operation: NEGX, operands: [ Implied, ARIND(A4) ], extra: NoExtra }
, &[" negx.l (a4)",
]);}
//  not.b d0
#[test]
fn test_decode_0125_not_b_d0() {
test_decoding_result_ok(&[0x46, 0x00],  Instruction { size: 1, operation: NOT, operands: [ Implied, DR(D0) ], extra: NoExtra }
, &[" not.b d0",
]);}
//  not.w (a0)+
#[test]
fn test_decode_0126_not_w_a0_() {
test_decoding_result_ok(&[0x46, 0x58],  Instruction { size: 2, operation: NOT, operands: [ Implied, ARINC(A0) ], extra: NoExtra }
, &[" not.w (a0)+",
]);}
//  not.l (a4)
#[test]
fn test_decode_0127_not_l_a4_() {
test_decoding_result_ok(&[0x46, 0x94],  Instruction { size: 4, operation: NOT, operands: [ Implied, ARIND(A4) ], extra: NoExtra }
, &[" not.l (a4)",
]);}
//  tst.b d0
#[test]
fn test_decode_0128_tst_b_d0() {
test_decoding_result_ok(&[0x4a, 0x00],  Instruction { size: 1, operation: TST, operands: [ Implied, DR(D0) ], extra: NoExtra }
, &[" tst.b d0",
]);}
//  tst.w (a0)+
#[test]
fn test_decode_0129_tst_w_a0_() {
test_decoding_result_ok(&[0x4a, 0x58],  Instruction { size: 2, operation: TST, operands: [ Implied, ARINC(A0) ], extra: NoExtra }
, &[" tst.w (a0)+",
]);}
//  tst.l (a4)
#[test]
fn test_decode_0130_tst_l_a4_() {
test_decoding_result_ok(&[0x4a, 0x94],  Instruction { size: 4, operation: TST, operands: [ Implied, ARIND(A4) ], extra: NoExtra }
, &[" tst.l (a4)",
]);}
//  chk.w (a4),d2
#[test]
fn test_decode_0131_chk_w_a4_d2() {
test_decoding_result_ok(&[0x45, 0x94],  Instruction { size: 2, operation: CHK, operands: [ ARIND(A4), DR(D2) ], extra: NoExtra }
, &[" chk.w (a4),d2",
]);}
//  chk.l (a4),d2
#[test]
fn test_decode_0132_chk_l_a4_d2() {
test_decoding_result_ok(&[0x45, 0x14],  Instruction { size: 4, operation: CHK, operands: [ ARIND(A4), DR(D2) ], extra: NoExtra }
, &[" chk.l (a4),d2",
]);}
//  bfchg (a4){12:7}
#[test]
fn test_decode_0133_bfchg_a4_12_7_() {
test_decoding_result_ok(&[0xea, 0xd4, 0x03, 0x07],  Instruction { size: 0, operation: BFCHG, operands: [ NoOperand, ARIND(A4) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bfchg (a4){12:7}",
]);}
//  bfchg (a4){d2:7}
#[test]
fn test_decode_0134_bfchg_a4_d2_7_() {
test_decoding_result_ok(&[0xea, 0xd4, 0x08, 0x87],  Instruction { size: 0, operation: BFCHG, operands: [ NoOperand, ARIND(A4) ], extra: Bitfield(DYNAMIC(D2), STATIC(7)) }
, &[" bfchg (a4){d2:7}",
]);}
//  bfchg (a4){d2:d3}
#[test]
fn test_decode_0135_bfchg_a4_d2_d3_() {
test_decoding_result_ok(&[0xea, 0xd4, 0x08, 0xa3],  Instruction { size: 0, operation: BFCHG, operands: [ NoOperand, ARIND(A4) ], extra: Bitfield(DYNAMIC(D2), DYNAMIC(D3)) }
, &[" bfchg (a4){d2:d3}",
]);}
//  bfclr (a4){12:7}
#[test]
fn test_decode_0136_bfclr_a4_12_7_() {
test_decoding_result_ok(&[0xec, 0xd4, 0x03, 0x07],  Instruction { size: 0, operation: BFCLR, operands: [ NoOperand, ARIND(A4) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bfclr (a4){12:7}",
]);}
//  bfexts (a4){12:7},d1
#[test]
fn test_decode_0137_bfexts_a4_12_7_d1() {
test_decoding_result_ok(&[0xeb, 0xd4, 0x13, 0x07],  Instruction { size: 0, operation: BFEXTS, operands: [ ARIND(A4), DR(D1) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bfexts (a4){12:7},d1",
]);}
//  bfextu (a4){12:7},d1
#[test]
fn test_decode_0138_bfextu_a4_12_7_d1() {
test_decoding_result_ok(&[0xe9, 0xd4, 0x13, 0x07],  Instruction { size: 0, operation: BFEXTU, operands: [ ARIND(A4), DR(D1) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bfextu (a4){12:7},d1",
]);}
//  bfffo (a4){12:7},d1
#[test]
fn test_decode_0139_bfffo_a4_12_7_d1() {
test_decoding_result_ok(&[0xed, 0xd4, 0x13, 0x07],  Instruction { size: 0, operation: BFFFO, operands: [ ARIND(A4), DR(D1) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bfffo (a4){12:7},d1",
]);}
//  bfins d1,(a4){12:7}
#[test]
fn test_decode_0140_bfins_d1_a4_12_7_() {
test_decoding_result_ok(&[0xef, 0xd4, 0x13, 0x07],  Instruction { size: 0, operation: BFINS, operands: [ DR(D1), ARIND(A4) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bfins d1,(a4){12:7}",
]);}
//  bfset (a4){12:7}
#[test]
fn test_decode_0141_bfset_a4_12_7_() {
test_decoding_result_ok(&[0xee, 0xd4, 0x03, 0x07],  Instruction { size: 0, operation: BFSET, operands: [ NoOperand, ARIND(A4) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bfset (a4){12:7}",
]);}
//  bftst (a4){12:7}
#[test]
fn test_decode_0142_bftst_a4_12_7_() {
test_decoding_result_ok(&[0xe8, 0xd4, 0x03, 0x07],  Instruction { size: 0, operation: BFTST, operands: [ NoOperand, ARIND(A4) ], extra: Bitfield(STATIC(12), STATIC(7)) }
, &[" bftst (a4){12:7}",
]);}
//  .self: dbf d3,.self
#[test]
fn test_decode_0143_self_dbf_d3_self() {
test_decoding_result_ok(&[0x51, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_F) }
, &[" .self: dbf d3,.self",
]);}
//  .self: dbhi d3,.self
#[test]
fn test_decode_0144_self_dbhi_d3_self() {
test_decoding_result_ok(&[0x52, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_HI) }
, &[" .self: dbhi d3,.self",
]);}
//  .self: dbls d3,.self
#[test]
fn test_decode_0145_self_dbls_d3_self() {
test_decoding_result_ok(&[0x53, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_LS) }
, &[" .self: dbls d3,.self",
]);}
//  .self: dbcc d3,.self
#[test]
fn test_decode_0146_self_dbcc_d3_self() {
test_decoding_result_ok(&[0x54, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_CC) }
, &[" .self: dbcc d3,.self",
]);}
//  .self: dbhs d3,.self
#[test]
fn test_decode_0147_self_dbhs_d3_self() {
test_decoding_result_ok(&[0x54, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_CC) }
, &[" .self: dbhs d3,.self",
]);}
//  .self: dbcs d3,.self
#[test]
fn test_decode_0148_self_dbcs_d3_self() {
test_decoding_result_ok(&[0x55, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_CS) }
, &[" .self: dbcs d3,.self",
]);}
//  .self: dblo d3,.self
#[test]
fn test_decode_0149_self_dblo_d3_self() {
test_decoding_result_ok(&[0x55, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_CS) }
, &[" .self: dblo d3,.self",
]);}
//  .self: dbne d3,.self
#[test]
fn test_decode_0150_self_dbne_d3_self() {
test_decoding_result_ok(&[0x56, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_NE) }
, &[" .self: dbne d3,.self",
]);}
//  .self: dbeq d3,.self
#[test]
fn test_decode_0151_self_dbeq_d3_self() {
test_decoding_result_ok(&[0x57, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_EQ) }
, &[" .self: dbeq d3,.self",
]);}
//  .self: dbvc d3,.self
#[test]
fn test_decode_0152_self_dbvc_d3_self() {
test_decoding_result_ok(&[0x58, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_VC) }
, &[" .self: dbvc d3,.self",
]);}
//  .self: dbvs d3,.self
#[test]
fn test_decode_0153_self_dbvs_d3_self() {
test_decoding_result_ok(&[0x59, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_VS) }
, &[" .self: dbvs d3,.self",
]);}
//  .self: dbpl d3,.self
#[test]
fn test_decode_0154_self_dbpl_d3_self() {
test_decoding_result_ok(&[0x5a, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_PL) }
, &[" .self: dbpl d3,.self",
]);}
//  .self: dbmi d3,.self
#[test]
fn test_decode_0155_self_dbmi_d3_self() {
test_decoding_result_ok(&[0x5b, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_MI) }
, &[" .self: dbmi d3,.self",
]);}
//  .self: dbge d3,.self
#[test]
fn test_decode_0156_self_dbge_d3_self() {
test_decoding_result_ok(&[0x5c, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_GE) }
, &[" .self: dbge d3,.self",
]);}
//  .self: dblt d3,.self
#[test]
fn test_decode_0157_self_dblt_d3_self() {
test_decoding_result_ok(&[0x5d, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_LT) }
, &[" .self: dblt d3,.self",
]);}
//  .self: dbgt d3,.self
#[test]
fn test_decode_0158_self_dbgt_d3_self() {
test_decoding_result_ok(&[0x5e, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_GT) }
, &[" .self: dbgt d3,.self",
]);}
//  .self: dble d3,.self
#[test]
fn test_decode_0159_self_dble_d3_self() {
test_decoding_result_ok(&[0x5f, 0xcb, 0xff, 0xfe],  Instruction { size: 2, operation: DBCC, operands: [ DR(D3), PCDISP(2, simple_disp(-2)) ], extra: Condition(CC_LE) }
, &[" .self: dble d3,.self",
]);}
//  addq.b #1,d0
#[test]
fn test_decode_0160_addq_b_1_d0() {
test_decoding_result_ok(&[0x52, 0x00],  Instruction { size: 1, operation: ADDQ, operands: [ IMM8(1), DR(D0) ], extra: NoExtra }
, &[" addq.b #1,d0",
]);}
//  addq.w #7,a0
#[test]
fn test_decode_0161_addq_w_7_a0() {
test_decoding_result_ok(&[0x5e, 0x48],  Instruction { size: 2, operation: ADDQ, operands: [ IMM8(7), AR(A0) ], extra: NoExtra }
, &[" addq.w #7,a0",
]);}
//  addq.l #8,d0
#[test]
fn test_decode_0162_addq_l_8_d0() {
test_decoding_result_ok(&[0x50, 0x80],  Instruction { size: 4, operation: ADDQ, operands: [ IMM8(8), DR(D0) ], extra: NoExtra }
, &[" addq.l #8,d0",
]);}
//  subq.b #1,d0
#[test]
fn test_decode_0163_subq_b_1_d0() {
test_decoding_result_ok(&[0x53, 0x00],  Instruction { size: 1, operation: SUBQ, operands: [ IMM8(1), DR(D0) ], extra: NoExtra }
, &[" subq.b #1,d0",
]);}
//  subq.w #7,a0
#[test]
fn test_decode_0164_subq_w_7_a0() {
test_decoding_result_ok(&[0x5f, 0x48],  Instruction { size: 2, operation: SUBQ, operands: [ IMM8(7), AR(A0) ], extra: NoExtra }
, &[" subq.w #7,a0",
]);}
//  subq.l #8,d0
#[test]
fn test_decode_0165_subq_l_8_d0() {
test_decoding_result_ok(&[0x51, 0x80],  Instruction { size: 4, operation: SUBQ, operands: [ IMM8(8), DR(D0) ], extra: NoExtra }
, &[" subq.l #8,d0",
]);}
//  trapne
#[test]
fn test_decode_0166_trapne() {
test_decoding_result_ok(&[0x56, 0xfc],  Instruction { size: 0, operation: TRAPCC, operands: [ NoOperand, NoOperand ], extra: Condition(CC_NE) }
, &[" trapne",
]);}
//  trapne.w #1234
#[test]
fn test_decode_0167_trapne_w_1234() {
test_decoding_result_ok(&[0x56, 0xfa, 0x04, 0xd2],  Instruction { size: 2, operation: TRAPCC, operands: [ IMM16(1234), NoOperand ], extra: Condition(CC_NE) }
, &[" trapne.w #1234",
]);}
//  trapne.l #$12345678
#[test]
fn test_decode_0168_trapne_l_12345678() {
test_decoding_result_ok(&[0x56, 0xfb, 0x12, 0x34, 0x56, 0x78],  Instruction { size: 4, operation: TRAPCC, operands: [ IMM32(0x12345678), NoOperand ], extra: Condition(CC_NE) }
, &[" trapne.l #$12345678",
]);}
//  sne (a0)
#[test]
fn test_decode_0169_sne_a0_() {
test_decoding_result_ok(&[0x56, 0xd0],  Instruction { size: 1, operation: SCC, operands: [ Implied, ARIND(A0) ], extra: Condition(CC_NE) }
, &[" sne (a0)",
]);}
//  addx.b d0,d1
#[test]
fn test_decode_0170_addx_b_d0_d1() {
test_decoding_result_ok(&[0xd3, 0x00],  Instruction { size: 1, operation: ADDX, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" addx.b d0,d1",
]);}
//  addx.w d0,d1
#[test]
fn test_decode_0171_addx_w_d0_d1() {
test_decoding_result_ok(&[0xd3, 0x40],  Instruction { size: 2, operation: ADDX, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" addx.w d0,d1",
]);}
//  addx.l d0,d1
#[test]
fn test_decode_0172_addx_l_d0_d1() {
test_decoding_result_ok(&[0xd3, 0x80],  Instruction { size: 4, operation: ADDX, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" addx.l d0,d1",
]);}
//  addx.b -(a2),-(a3)
#[test]
fn test_decode_0173_addx_b_a2_a3_() {
test_decoding_result_ok(&[0xd7, 0x0a],  Instruction { size: 1, operation: ADDX, operands: [ ARDEC(A2), ARDEC(A3) ], extra: NoExtra }
, &[" addx.b -(a2),-(a3)",
]);}
//  addx.w -(a2),-(a3)
#[test]
fn test_decode_0174_addx_w_a2_a3_() {
test_decoding_result_ok(&[0xd7, 0x4a],  Instruction { size: 2, operation: ADDX, operands: [ ARDEC(A2), ARDEC(A3) ], extra: NoExtra }
, &[" addx.w -(a2),-(a3)",
]);}
//  addx.l -(a2),-(a3)
#[test]
fn test_decode_0175_addx_l_a2_a3_() {
test_decoding_result_ok(&[0xd7, 0x8a],  Instruction { size: 4, operation: ADDX, operands: [ ARDEC(A2), ARDEC(A3) ], extra: NoExtra }
, &[" addx.l -(a2),-(a3)",
]);}
//  add.b (a2),d0
#[test]
fn test_decode_0176_add_b_a2_d0() {
test_decoding_result_ok(&[0xd0, 0x12],  Instruction { size: 1, operation: ADD, operands: [ ARIND(A2), DR(D0) ], extra: NoExtra }
, &[" add.b (a2),d0",
]);}
//  add.w (a2),d0
#[test]
fn test_decode_0177_add_w_a2_d0() {
test_decoding_result_ok(&[0xd0, 0x52],  Instruction { size: 2, operation: ADD, operands: [ ARIND(A2), DR(D0) ], extra: NoExtra }
, &[" add.w (a2),d0",
]);}
//  add.l (a2),d0
#[test]
fn test_decode_0178_add_l_a2_d0() {
test_decoding_result_ok(&[0xd0, 0x92],  Instruction { size: 4, operation: ADD, operands: [ ARIND(A2), DR(D0) ], extra: NoExtra }
, &[" add.l (a2),d0",
]);}
//  add.b d3,(a2)
#[test]
fn test_decode_0179_add_b_d3_a2_() {
test_decoding_result_ok(&[0xd7, 0x12],  Instruction { size: 1, operation: ADD, operands: [ DR(D3), ARIND(A2) ], extra: NoExtra }
, &[" add.b d3,(a2)",
]);}
//  add.w d3,(a2)
#[test]
fn test_decode_0180_add_w_d3_a2_() {
test_decoding_result_ok(&[0xd7, 0x52],  Instruction { size: 2, operation: ADD, operands: [ DR(D3), ARIND(A2) ], extra: NoExtra }
, &[" add.w d3,(a2)",
]);}
//  add.l d3,(a2)
#[test]
fn test_decode_0181_add_l_d3_a2_() {
test_decoding_result_ok(&[0xd7, 0x92],  Instruction { size: 4, operation: ADD, operands: [ DR(D3), ARIND(A2) ], extra: NoExtra }
, &[" add.l d3,(a2)",
]);}
//  subx.b d0,d1
#[test]
fn test_decode_0182_subx_b_d0_d1() {
test_decoding_result_ok(&[0x93, 0x00],  Instruction { size: 1, operation: SUBX, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" subx.b d0,d1",
]);}
//  subx.w d0,d1
#[test]
fn test_decode_0183_subx_w_d0_d1() {
test_decoding_result_ok(&[0x93, 0x40],  Instruction { size: 2, operation: SUBX, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" subx.w d0,d1",
]);}
//  subx.l d0,d1
#[test]
fn test_decode_0184_subx_l_d0_d1() {
test_decoding_result_ok(&[0x93, 0x80],  Instruction { size: 4, operation: SUBX, operands: [ DR(D0), DR(D1) ], extra: NoExtra }
, &[" subx.l d0,d1",
]);}
//  subx.b -(a2),-(a3)
#[test]
fn test_decode_0185_subx_b_a2_a3_() {
test_decoding_result_ok(&[0x97, 0x0a],  Instruction { size: 1, operation: SUBX, operands: [ ARDEC(A2), ARDEC(A3) ], extra: NoExtra }
, &[" subx.b -(a2),-(a3)",
]);}
//  subx.w -(a2),-(a3)
#[test]
fn test_decode_0186_subx_w_a2_a3_() {
test_decoding_result_ok(&[0x97, 0x4a],  Instruction { size: 2, operation: SUBX, operands: [ ARDEC(A2), ARDEC(A3) ], extra: NoExtra }
, &[" subx.w -(a2),-(a3)",
]);}
//  subx.l -(a2),-(a3)
#[test]
fn test_decode_0187_subx_l_a2_a3_() {
test_decoding_result_ok(&[0x97, 0x8a],  Instruction { size: 4, operation: SUBX, operands: [ ARDEC(A2), ARDEC(A3) ], extra: NoExtra }
, &[" subx.l -(a2),-(a3)",
]);}
//  sub.b (a2),d0
#[test]
fn test_decode_0188_sub_b_a2_d0() {
test_decoding_result_ok(&[0x90, 0x12],  Instruction { size: 1, operation: SUB, operands: [ ARIND(A2), DR(D0) ], extra: NoExtra }
, &[" sub.b (a2),d0",
]);}
//  sub.w (a2),d0
#[test]
fn test_decode_0189_sub_w_a2_d0() {
test_decoding_result_ok(&[0x90, 0x52],  Instruction { size: 2, operation: SUB, operands: [ ARIND(A2), DR(D0) ], extra: NoExtra }
, &[" sub.w (a2),d0",
]);}
//  sub.l (a2),d0
#[test]
fn test_decode_0190_sub_l_a2_d0() {
test_decoding_result_ok(&[0x90, 0x92],  Instruction { size: 4, operation: SUB, operands: [ ARIND(A2), DR(D0) ], extra: NoExtra }
, &[" sub.l (a2),d0",
]);}
//  sub.b d3,(a2)
#[test]
fn test_decode_0191_sub_b_d3_a2_() {
test_decoding_result_ok(&[0x97, 0x12],  Instruction { size: 1, operation: SUB, operands: [ DR(D3), ARIND(A2) ], extra: NoExtra }
, &[" sub.b d3,(a2)",
]);}
//  sub.w d3,(a2)
#[test]
fn test_decode_0192_sub_w_d3_a2_() {
test_decoding_result_ok(&[0x97, 0x52],  Instruction { size: 2, operation: SUB, operands: [ DR(D3), ARIND(A2) ], extra: NoExtra }
, &[" sub.w d3,(a2)",
]);}
//  sub.l d3,(a2)
#[test]
fn test_decode_0193_sub_l_d3_a2_() {
test_decoding_result_ok(&[0x97, 0x92],  Instruction { size: 4, operation: SUB, operands: [ DR(D3), ARIND(A2) ], extra: NoExtra }
, &[" sub.l d3,(a2)",
]);}
//  cmpa.w (a1),a2
#[test]
fn test_decode_0194_cmpa_w_a1_a2() {
test_decoding_result_ok(&[0xb4, 0xd1],  Instruction { size: 2, operation: CMPA, operands: [ ARIND(A1), AR(A2) ], extra: NoExtra }
, &[" cmpa.w (a1),a2",
]);}
//  cmpa.l (a1),a2
#[test]
fn test_decode_0195_cmpa_l_a1_a2() {
test_decoding_result_ok(&[0xb5, 0xd1],  Instruction { size: 4, operation: CMPA, operands: [ ARIND(A1), AR(A2) ], extra: NoExtra }
, &[" cmpa.l (a1),a2",
]);}
//  cmpm.b (a0)+,(a2)+
#[test]
fn test_decode_0196_cmpm_b_a0_a2_() {
test_decoding_result_ok(&[0xb5, 0x08],  Instruction { size: 1, operation: CMPM, operands: [ ARINC(A0), ARINC(A2) ], extra: NoExtra }
, &[" cmpm.b (a0)+,(a2)+",
]);}
//  cmpm.w (a0)+,(a2)+
#[test]
fn test_decode_0197_cmpm_w_a0_a2_() {
test_decoding_result_ok(&[0xb5, 0x48],  Instruction { size: 2, operation: CMPM, operands: [ ARINC(A0), ARINC(A2) ], extra: NoExtra }
, &[" cmpm.w (a0)+,(a2)+",
]);}
//  cmpm.l (a0)+,(a2)+
#[test]
fn test_decode_0198_cmpm_l_a0_a2_() {
test_decoding_result_ok(&[0xb5, 0x88],  Instruction { size: 4, operation: CMPM, operands: [ ARINC(A0), ARINC(A2) ], extra: NoExtra }
, &[" cmpm.l (a0)+,(a2)+",
]);}
//  cmp.b (a0)+,d7
#[test]
fn test_decode_0199_cmp_b_a0_d7() {
test_decoding_result_ok(&[0xbe, 0x18],  Instruction { size: 1, operation: CMP, operands: [ ARINC(A0), DR(D7) ], extra: NoExtra }
, &[" cmp.b (a0)+,d7",
]);}
//  cmp.w (a0)+,d7
#[test]
fn test_decode_0200_cmp_w_a0_d7() {
test_decoding_result_ok(&[0xbe, 0x58],  Instruction { size: 2, operation: CMP, operands: [ ARINC(A0), DR(D7) ], extra: NoExtra }
, &[" cmp.w (a0)+,d7",
]);}
//  cmp.l (a0)+,d7
#[test]
fn test_decode_0201_cmp_l_a0_d7() {
test_decoding_result_ok(&[0xbe, 0x98],  Instruction { size: 4, operation: CMP, operands: [ ARINC(A0), DR(D7) ], extra: NoExtra }
, &[" cmp.l (a0)+,d7",
]);}
//  eor.b d7,(a0)+
#[test]
fn test_decode_0202_eor_b_d7_a0_() {
test_decoding_result_ok(&[0xbf, 0x18],  Instruction { size: 1, operation: EOR, operands: [ DR(D7), ARINC(A0) ], extra: NoExtra }
, &[" eor.b d7,(a0)+",
]);}
//  eor.w d7,(a0)+
#[test]
fn test_decode_0203_eor_w_d7_a0_() {
test_decoding_result_ok(&[0xbf, 0x58],  Instruction { size: 2, operation: EOR, operands: [ DR(D7), ARINC(A0) ], extra: NoExtra }
, &[" eor.w d7,(a0)+",
]);}
//  eor.l d7,-(a0)
#[test]
fn test_decode_0204_eor_l_d7_a0_() {
test_decoding_result_ok(&[0xbf, 0xa0],  Instruction { size: 4, operation: EOR, operands: [ DR(D7), ARDEC(A0) ], extra: NoExtra }
, &[" eor.l d7,-(a0)",
]);}
//  lab:
//    bra.s lab
#[test]
fn test_decode_0205_lab_() {
test_decoding_result_ok(&[0x60, 0xfe],  Instruction { size: 1, operation: BRA, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: NoExtra }
, &[" lab:",
"   bra.s lab",
]);}
//  lab:
//    bra.w lab
#[test]
fn test_decode_0206_lab_() {
test_decoding_result_ok(&[0x60, 0x00, 0xff, 0xfe],  Instruction { size: 2, operation: BRA, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: NoExtra }
, &[" lab:",
"   bra.w lab",
]);}
//  lab:
//    bra.l lab
#[test]
fn test_decode_0207_lab_() {
test_decoding_result_ok(&[0x60, 0xff, 0xff, 0xff, 0xff, 0xfe],  Instruction { size: 4, operation: BRA, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: NoExtra }
, &[" lab:",
"   bra.l lab",
]);}
//  lab:
//    bsr.s lab
#[test]
fn test_decode_0208_lab_() {
test_decoding_result_ok(&[0x61, 0xfe],  Instruction { size: 1, operation: BSR, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: NoExtra }
, &[" lab:",
"   bsr.s lab",
]);}
//  lab:
//    bsr.w lab
#[test]
fn test_decode_0209_lab_() {
test_decoding_result_ok(&[0x61, 0x00, 0xff, 0xfe],  Instruction { size: 2, operation: BSR, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: NoExtra }
, &[" lab:",
"   bsr.w lab",
]);}
//  lab:
//    bsr.l lab
#[test]
fn test_decode_0210_lab_() {
test_decoding_result_ok(&[0x61, 0xff, 0xff, 0xff, 0xff, 0xfe],  Instruction { size: 4, operation: BSR, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: NoExtra }
, &[" lab:",
"   bsr.l lab",
]);}
//  lab:
//    bne.s lab
#[test]
fn test_decode_0211_lab_() {
test_decoding_result_ok(&[0x66, 0xfe],  Instruction { size: 1, operation: BCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: Condition(CC_NE), }
, &[" lab:",
"   bne.s lab",
]);}
//  lab:
//    beq.w lab
#[test]
fn test_decode_0212_lab_() {
test_decoding_result_ok(&[0x67, 0x00, 0xff, 0xfe],  Instruction { size: 2, operation: BCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: Condition(CC_EQ) }
, &[" lab:",
"   beq.w lab",
]);}
//  lab:
//    bcs.l lab
#[test]
fn test_decode_0213_lab_() {
test_decoding_result_ok(&[0x65, 0xff, 0xff, 0xff, 0xff, 0xfe],  Instruction { size: 4, operation: BCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: Condition(CC_CS) }
, &[" lab:",
"   bcs.l lab",
]);}
//  pack d0,d1,#12
#[test]
fn test_decode_0214_pack_d0_d1_12() {
test_decoding_result_ok(&[0x83, 0x40, 0x00, 0x0c],  Instruction { size: 0, operation: PACK, operands: [ DR(D0), DR(D1) ], extra: PackAdjustment(12) }
, &[" pack d0,d1,#12",
]);}
//  unpk d0,d1,#12
#[test]
fn test_decode_0215_unpk_d0_d1_12() {
test_decoding_result_ok(&[0x83, 0x80, 0x00, 0x0c],  Instruction { size: 0, operation: UNPK, operands: [ DR(D0), DR(D1) ], extra: PackAdjustment(12) }
, &[" unpk d0,d1,#12",
]);}
//  pack -(a0),-(a1),#37
#[test]
fn test_decode_0216_pack_a0_a1_37() {
test_decoding_result_ok(&[0x83, 0x48, 0x00, 0x25],  Instruction { size: 0, operation: PACK, operands: [ ARDEC(A0), ARDEC(A1) ], extra: PackAdjustment(37) }
, &[" pack -(a0),-(a1),#37",
]);}
//  unpk -(a0),-(a1),#37
#[test]
fn test_decode_0217_unpk_a0_a1_37() {
test_decoding_result_ok(&[0x83, 0x88, 0x00, 0x25],  Instruction { size: 0, operation: UNPK, operands: [ ARDEC(A0), ARDEC(A1) ], extra: PackAdjustment(37) }
, &[" unpk -(a0),-(a1),#37",
]);}
//  sbcd -(a0),-(a1)
#[test]
fn test_decode_0218_sbcd_a0_a1_() {
test_decoding_result_ok(&[0x83, 0x08],  Instruction { size: 1, operation: SBCD, operands: [ ARDEC(A0), ARDEC(A1) ], extra: NoExtra }
, &[" sbcd -(a0),-(a1)",
]);}
//  sbcd d3,d4
#[test]
fn test_decode_0219_sbcd_d3_d4() {
test_decoding_result_ok(&[0x89, 0x03],  Instruction { size: 1, operation: SBCD, operands: [ DR(D3), DR(D4) ], extra: NoExtra }
, &[" sbcd d3,d4",
]);}
//  or.b (a0)+,d0
#[test]
fn test_decode_0220_or_b_a0_d0() {
test_decoding_result_ok(&[0x80, 0x18],  Instruction { size: 1, operation: OR, operands: [ ARINC(A0), DR(D0) ], extra: NoExtra }
, &[" or.b (a0)+,d0",
]);}
//  or.w (a0)+,d0
#[test]
fn test_decode_0221_or_w_a0_d0() {
test_decoding_result_ok(&[0x80, 0x58],  Instruction { size: 2, operation: OR, operands: [ ARINC(A0), DR(D0) ], extra: NoExtra }
, &[" or.w (a0)+,d0",
]);}
//  or.l (a0)+,d0
#[test]
fn test_decode_0222_or_l_a0_d0() {
test_decoding_result_ok(&[0x80, 0x98],  Instruction { size: 4, operation: OR, operands: [ ARINC(A0), DR(D0) ], extra: NoExtra }
, &[" or.l (a0)+,d0",
]);}
//  or.b d0,(a0)+
#[test]
fn test_decode_0223_or_b_d0_a0_() {
test_decoding_result_ok(&[0x81, 0x18],  Instruction { size: 1, operation: OR, operands: [ DR(D0), ARINC(A0) ], extra: NoExtra }
, &[" or.b d0,(a0)+",
]);}
//  or.w d0,(a0)+
#[test]
fn test_decode_0224_or_w_d0_a0_() {
test_decoding_result_ok(&[0x81, 0x58],  Instruction { size: 2, operation: OR, operands: [ DR(D0), ARINC(A0) ], extra: NoExtra }
, &[" or.w d0,(a0)+",
]);}
//  or.l d0,(a0)+
#[test]
fn test_decode_0225_or_l_d0_a0_() {
test_decoding_result_ok(&[0x81, 0x98],  Instruction { size: 4, operation: OR, operands: [ DR(D0), ARINC(A0) ], extra: NoExtra }
, &[" or.l d0,(a0)+",
]);}
//  exg d0,d5
#[test]
fn test_decode_0226_exg_d0_d5() {
test_decoding_result_ok(&[0xc1, 0x45],  Instruction { size: 4, operation: EXG, operands: [ DR(D0), DR(D5) ], extra: NoExtra }
, &[" exg d0,d5",
]);}
//  exg a0,a5
#[test]
fn test_decode_0227_exg_a0_a5() {
test_decoding_result_ok(&[0xc1, 0x4d],  Instruction { size: 4, operation: EXG, operands: [ AR(A0), AR(A5) ], extra: NoExtra }
, &[" exg a0,a5",
]);}
//  exg d3,a5
#[test]
fn test_decode_0228_exg_d3_a5() {
test_decoding_result_ok(&[0xc7, 0x8d],  Instruction { size: 4, operation: EXG, operands: [ DR(D3), AR(A5) ], extra: NoExtra }
, &[" exg d3,a5",
]);}
//  and.b (a0)+,d0
#[test]
fn test_decode_0229_and_b_a0_d0() {
test_decoding_result_ok(&[0xc0, 0x18],  Instruction { size: 1, operation: AND, operands: [ ARINC(A0), DR(D0) ], extra: NoExtra }
, &[" and.b (a0)+,d0",
]);}
//  and.w (a0)+,d0
#[test]
fn test_decode_0230_and_w_a0_d0() {
test_decoding_result_ok(&[0xc0, 0x58],  Instruction { size: 2, operation: AND, operands: [ ARINC(A0), DR(D0) ], extra: NoExtra }
, &[" and.w (a0)+,d0",
]);}
//  and.l (a0)+,d0
#[test]
fn test_decode_0231_and_l_a0_d0() {
test_decoding_result_ok(&[0xc0, 0x98],  Instruction { size: 4, operation: AND, operands: [ ARINC(A0), DR(D0) ], extra: NoExtra }
, &[" and.l (a0)+,d0",
]);}
//  and.b d0,(a0)+
#[test]
fn test_decode_0232_and_b_d0_a0_() {
test_decoding_result_ok(&[0xc1, 0x18],  Instruction { size: 1, operation: AND, operands: [ DR(D0), ARINC(A0) ], extra: NoExtra }
, &[" and.b d0,(a0)+",
]);}
//  and.w d0,(a0)+
#[test]
fn test_decode_0233_and_w_d0_a0_() {
test_decoding_result_ok(&[0xc1, 0x58],  Instruction { size: 2, operation: AND, operands: [ DR(D0), ARINC(A0) ], extra: NoExtra }
, &[" and.w d0,(a0)+",
]);}
//  and.l d0,(a0)+
#[test]
fn test_decode_0234_and_l_d0_a0_() {
test_decoding_result_ok(&[0xc1, 0x98],  Instruction { size: 4, operation: AND, operands: [ DR(D0), ARINC(A0) ], extra: NoExtra }
, &[" and.l d0,(a0)+",
]);}
//  asl.b #3,d7
#[test]
fn test_decode_0235_asl_b_3_d7() {
test_decoding_result_ok(&[0xe7, 0x07],  Instruction { size: 1, operation: ASL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" asl.b #3,d7",
]);}
//  asl.w #3,d7
#[test]
fn test_decode_0236_asl_w_3_d7() {
test_decoding_result_ok(&[0xe7, 0x47],  Instruction { size: 2, operation: ASL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" asl.w #3,d7",
]);}
//  asl.l #3,d7
#[test]
fn test_decode_0237_asl_l_3_d7() {
test_decoding_result_ok(&[0xe7, 0x87],  Instruction { size: 4, operation: ASL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" asl.l #3,d7",
]);}
//  asr.b #3,d7
#[test]
fn test_decode_0238_asr_b_3_d7() {
test_decoding_result_ok(&[0xe6, 0x07],  Instruction { size: 1, operation: ASR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" asr.b #3,d7",
]);}
//  asr.w #3,d7
#[test]
fn test_decode_0239_asr_w_3_d7() {
test_decoding_result_ok(&[0xe6, 0x47],  Instruction { size: 2, operation: ASR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" asr.w #3,d7",
]);}
//  asr.l #3,d7
#[test]
fn test_decode_0240_asr_l_3_d7() {
test_decoding_result_ok(&[0xe6, 0x87],  Instruction { size: 4, operation: ASR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" asr.l #3,d7",
]);}
//  asl.b d1,d7
#[test]
fn test_decode_0241_asl_b_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x27],  Instruction { size: 1, operation: ASL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" asl.b d1,d7",
]);}
//  asl.w d1,d7
#[test]
fn test_decode_0242_asl_w_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x67],  Instruction { size: 2, operation: ASL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" asl.w d1,d7",
]);}
//  asl.l d1,d7
#[test]
fn test_decode_0243_asl_l_d1_d7() {
test_decoding_result_ok(&[0xe3, 0xa7],  Instruction { size: 4, operation: ASL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" asl.l d1,d7",
]);}
//  asr.b d1,d7
#[test]
fn test_decode_0244_asr_b_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x27],  Instruction { size: 1, operation: ASR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" asr.b d1,d7",
]);}
//  asr.w d1,d7
#[test]
fn test_decode_0245_asr_w_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x67],  Instruction { size: 2, operation: ASR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" asr.w d1,d7",
]);}
//  asr.l d1,d7
#[test]
fn test_decode_0246_asr_l_d1_d7() {
test_decoding_result_ok(&[0xe2, 0xa7],  Instruction { size: 4, operation: ASR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" asr.l d1,d7",
]);}
//  asl.w (a0)
#[test]
fn test_decode_0247_asl_w_a0_() {
test_decoding_result_ok(&[0xe1, 0xd0],  Instruction { size: 2, operation: ASL, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" asl.w (a0)",
]);}
//  asr.w (a0)
#[test]
fn test_decode_0248_asr_w_a0_() {
test_decoding_result_ok(&[0xe0, 0xd0],  Instruction { size: 2, operation: ASR, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" asr.w (a0)",
]);}
//  lsl.b #3,d7
#[test]
fn test_decode_0249_lsl_b_3_d7() {
test_decoding_result_ok(&[0xe7, 0x0f],  Instruction { size: 1, operation: LSL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" lsl.b #3,d7",
]);}
//  lsl.w #3,d7
#[test]
fn test_decode_0250_lsl_w_3_d7() {
test_decoding_result_ok(&[0xe7, 0x4f],  Instruction { size: 2, operation: LSL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" lsl.w #3,d7",
]);}
//  lsl.l #3,d7
#[test]
fn test_decode_0251_lsl_l_3_d7() {
test_decoding_result_ok(&[0xe7, 0x8f],  Instruction { size: 4, operation: LSL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" lsl.l #3,d7",
]);}
//  lsr.b #3,d7
#[test]
fn test_decode_0252_lsr_b_3_d7() {
test_decoding_result_ok(&[0xe6, 0x0f],  Instruction { size: 1, operation: LSR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" lsr.b #3,d7",
]);}
//  lsr.w #3,d7
#[test]
fn test_decode_0253_lsr_w_3_d7() {
test_decoding_result_ok(&[0xe6, 0x4f],  Instruction { size: 2, operation: LSR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" lsr.w #3,d7",
]);}
//  lsr.l #3,d7
#[test]
fn test_decode_0254_lsr_l_3_d7() {
test_decoding_result_ok(&[0xe6, 0x8f],  Instruction { size: 4, operation: LSR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" lsr.l #3,d7",
]);}
//  lsl.b d1,d7
#[test]
fn test_decode_0255_lsl_b_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x2f],  Instruction { size: 1, operation: LSL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" lsl.b d1,d7",
]);}
//  lsl.w d1,d7
#[test]
fn test_decode_0256_lsl_w_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x6f],  Instruction { size: 2, operation: LSL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" lsl.w d1,d7",
]);}
//  lsl.l d1,d7
#[test]
fn test_decode_0257_lsl_l_d1_d7() {
test_decoding_result_ok(&[0xe3, 0xaf],  Instruction { size: 4, operation: LSL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" lsl.l d1,d7",
]);}
//  lsr.b d1,d7
#[test]
fn test_decode_0258_lsr_b_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x2f],  Instruction { size: 1, operation: LSR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" lsr.b d1,d7",
]);}
//  lsr.w d1,d7
#[test]
fn test_decode_0259_lsr_w_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x6f],  Instruction { size: 2, operation: LSR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" lsr.w d1,d7",
]);}
//  lsr.l d1,d7
#[test]
fn test_decode_0260_lsr_l_d1_d7() {
test_decoding_result_ok(&[0xe2, 0xaf],  Instruction { size: 4, operation: LSR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" lsr.l d1,d7",
]);}
//  lsl.w (a0)
#[test]
fn test_decode_0261_lsl_w_a0_() {
test_decoding_result_ok(&[0xe3, 0xd0],  Instruction { size: 2, operation: LSL, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" lsl.w (a0)",
]);}
//  lsr.w (a0)
#[test]
fn test_decode_0262_lsr_w_a0_() {
test_decoding_result_ok(&[0xe2, 0xd0],  Instruction { size: 2, operation: LSR, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" lsr.w (a0)",
]);}
//  roxl.b #3,d7
#[test]
fn test_decode_0263_roxl_b_3_d7() {
test_decoding_result_ok(&[0xe7, 0x17],  Instruction { size: 1, operation: ROXL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" roxl.b #3,d7",
]);}
//  roxl.w #3,d7
#[test]
fn test_decode_0264_roxl_w_3_d7() {
test_decoding_result_ok(&[0xe7, 0x57],  Instruction { size: 2, operation: ROXL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" roxl.w #3,d7",
]);}
//  roxl.l #3,d7
#[test]
fn test_decode_0265_roxl_l_3_d7() {
test_decoding_result_ok(&[0xe7, 0x97],  Instruction { size: 4, operation: ROXL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" roxl.l #3,d7",
]);}
//  roxr.b #3,d7
#[test]
fn test_decode_0266_roxr_b_3_d7() {
test_decoding_result_ok(&[0xe6, 0x17],  Instruction { size: 1, operation: ROXR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" roxr.b #3,d7",
]);}
//  roxr.w #3,d7
#[test]
fn test_decode_0267_roxr_w_3_d7() {
test_decoding_result_ok(&[0xe6, 0x57],  Instruction { size: 2, operation: ROXR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" roxr.w #3,d7",
]);}
//  roxr.l #3,d7
#[test]
fn test_decode_0268_roxr_l_3_d7() {
test_decoding_result_ok(&[0xe6, 0x97],  Instruction { size: 4, operation: ROXR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" roxr.l #3,d7",
]);}
//  roxl.b d1,d7
#[test]
fn test_decode_0269_roxl_b_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x37],  Instruction { size: 1, operation: ROXL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" roxl.b d1,d7",
]);}
//  roxl.w d1,d7
#[test]
fn test_decode_0270_roxl_w_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x77],  Instruction { size: 2, operation: ROXL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" roxl.w d1,d7",
]);}
//  roxl.l d1,d7
#[test]
fn test_decode_0271_roxl_l_d1_d7() {
test_decoding_result_ok(&[0xe3, 0xb7],  Instruction { size: 4, operation: ROXL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" roxl.l d1,d7",
]);}
//  roxr.b d1,d7
#[test]
fn test_decode_0272_roxr_b_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x37],  Instruction { size: 1, operation: ROXR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" roxr.b d1,d7",
]);}
//  roxr.w d1,d7
#[test]
fn test_decode_0273_roxr_w_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x77],  Instruction { size: 2, operation: ROXR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" roxr.w d1,d7",
]);}
//  roxr.l d1,d7
#[test]
fn test_decode_0274_roxr_l_d1_d7() {
test_decoding_result_ok(&[0xe2, 0xb7],  Instruction { size: 4, operation: ROXR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" roxr.l d1,d7",
]);}
//  roxl.w (a0)
#[test]
fn test_decode_0275_roxl_w_a0_() {
test_decoding_result_ok(&[0xe5, 0xd0],  Instruction { size: 2, operation: ROXL, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" roxl.w (a0)",
]);}
//  roxr.w (a0)
#[test]
fn test_decode_0276_roxr_w_a0_() {
test_decoding_result_ok(&[0xe4, 0xd0],  Instruction { size: 2, operation: ROXR, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" roxr.w (a0)",
]);}
//  rol.b #3,d7
#[test]
fn test_decode_0277_rol_b_3_d7() {
test_decoding_result_ok(&[0xe7, 0x1f],  Instruction { size: 1, operation: ROL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" rol.b #3,d7",
]);}
//  rol.w #3,d7
#[test]
fn test_decode_0278_rol_w_3_d7() {
test_decoding_result_ok(&[0xe7, 0x5f],  Instruction { size: 2, operation: ROL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" rol.w #3,d7",
]);}
//  rol.l #3,d7
#[test]
fn test_decode_0279_rol_l_3_d7() {
test_decoding_result_ok(&[0xe7, 0x9f],  Instruction { size: 4, operation: ROL, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" rol.l #3,d7",
]);}
//  ror.b #3,d7
#[test]
fn test_decode_0280_ror_b_3_d7() {
test_decoding_result_ok(&[0xe6, 0x1f],  Instruction { size: 1, operation: ROR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" ror.b #3,d7",
]);}
//  ror.w #3,d7
#[test]
fn test_decode_0281_ror_w_3_d7() {
test_decoding_result_ok(&[0xe6, 0x5f],  Instruction { size: 2, operation: ROR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" ror.w #3,d7",
]);}
//  ror.l #3,d7
#[test]
fn test_decode_0282_ror_l_3_d7() {
test_decoding_result_ok(&[0xe6, 0x9f],  Instruction { size: 4, operation: ROR, operands: [ IMM8(3), DR(D7) ], extra: NoExtra }
, &[" ror.l #3,d7",
]);}
//  rol.b d1,d7
#[test]
fn test_decode_0283_rol_b_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x3f],  Instruction { size: 1, operation: ROL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" rol.b d1,d7",
]);}
//  rol.w d1,d7
#[test]
fn test_decode_0284_rol_w_d1_d7() {
test_decoding_result_ok(&[0xe3, 0x7f],  Instruction { size: 2, operation: ROL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" rol.w d1,d7",
]);}
//  rol.l d1,d7
#[test]
fn test_decode_0285_rol_l_d1_d7() {
test_decoding_result_ok(&[0xe3, 0xbf],  Instruction { size: 4, operation: ROL, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" rol.l d1,d7",
]);}
//  ror.b d1,d7
#[test]
fn test_decode_0286_ror_b_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x3f],  Instruction { size: 1, operation: ROR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" ror.b d1,d7",
]);}
//  ror.w d1,d7
#[test]
fn test_decode_0287_ror_w_d1_d7() {
test_decoding_result_ok(&[0xe2, 0x7f],  Instruction { size: 2, operation: ROR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" ror.w d1,d7",
]);}
//  ror.l d1,d7
#[test]
fn test_decode_0288_ror_l_d1_d7() {
test_decoding_result_ok(&[0xe2, 0xbf],  Instruction { size: 4, operation: ROR, operands: [ DR(D1), DR(D7) ], extra: NoExtra }
, &[" ror.l d1,d7",
]);}
//  rol.w (a0)
#[test]
fn test_decode_0289_rol_w_a0_() {
test_decoding_result_ok(&[0xe7, 0xd0],  Instruction { size: 2, operation: ROL, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" rol.w (a0)",
]);}
//  ror.w (a0)
#[test]
fn test_decode_0290_ror_w_a0_() {
test_decoding_result_ok(&[0xe6, 0xd0],  Instruction { size: 2, operation: ROR, operands: [ Implied, ARIND(A0) ], extra: NoExtra }
, &[" ror.w (a0)",
]);}
//  moveq #-1,d2
#[test]
fn test_decode_0291_moveq_1_d2() {
test_decoding_result_ok(&[0x74, 0xff],  Instruction { size: 4, operation: MOVEQ, operands: [ IMM8(0xff), DR(D2) ], extra: NoExtra }
, &[" moveq #-1,d2",
]);}
//  moveq #127,d5
#[test]
fn test_decode_0292_moveq_127_d5() {
test_decoding_result_ok(&[0x7a, 0x7f],  Instruction { size: 4, operation: MOVEQ, operands: [ IMM8(0x7f), DR(D5) ], extra: NoExtra }
, &[" moveq #127,d5",
]);}
//  fabs fp1
#[test]
fn test_decode_0293_fabs_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x04, 0x98],  Instruction { size: 10, operation: FABS, operands: [ FR(FP1), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fabs fp1",
]);}
//  fsabs fp1
#[test]
fn test_decode_0294_fsabs_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x04, 0xd8],  Instruction { size: 10, operation: FSABS, operands: [ FR(FP1), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsabs fp1",
]);}
//  fdabs fp1
#[test]
fn test_decode_0295_fdabs_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x04, 0xdc],  Instruction { size: 10, operation: FDABS, operands: [ FR(FP1), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fdabs fp1",
]);}
//  fabs.l (a0),fp1
#[test]
fn test_decode_0296_fabs_l_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x40, 0x98],  Instruction { size: 4, operation: FABS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_LONG_INT) }
, &[" fabs.l (a0),fp1",
]);}
//  fabs.s (a0),fp1
#[test]
fn test_decode_0297_fabs_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x98],  Instruction { size: 4, operation: FABS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fabs.s (a0),fp1",
]);}
//  fabs.d (a0),fp1
#[test]
fn test_decode_0298_fabs_d_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x54, 0x98],  Instruction { size: 8, operation: FABS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_DOUBLE) }
, &[" fabs.d (a0),fp1",
]);}
//  fabs.w (a0),fp1
#[test]
fn test_decode_0299_fabs_w_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x50, 0x98],  Instruction { size: 2, operation: FABS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_WORD_INT) }
, &[" fabs.w (a0),fp1",
]);}
//  fabs.b (a0),fp1
#[test]
fn test_decode_0300_fabs_b_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x58, 0x98],  Instruction { size: 1, operation: FABS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_BYTE_INT) }
, &[" fabs.b (a0),fp1",
]);}
//  fabs.x (a0),fp1
#[test]
fn test_decode_0301_fabs_x_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x48, 0x98],  Instruction { size: 10, operation: FABS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fabs.x (a0),fp1",
]);}
//  fabs.p (a0),fp1
#[test]
fn test_decode_0302_fabs_p_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x4c, 0x98],  Instruction { size: 12, operation: FABS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_PACKED_DECIMAL_REAL_STATIC(0)) }
, &[" fabs.p (a0),fp1",
]);}
//  fabs fp3,fp1
#[test]
fn test_decode_0303_fabs_fp3_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0c, 0x98],  Instruction { size: 10, operation: FABS, operands: [ FR(FP3), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fabs fp3,fp1",
]);}
//  facos fp0,fp1
#[test]
fn test_decode_0304_facos_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x9c],  Instruction { size: 10, operation: FACOS, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" facos fp0,fp1",
]);}
//  facos.s (a6),fp1
#[test]
fn test_decode_0305_facos_s_a6_fp1() {
test_decoding_result_ok(&[0xf2, 0x16, 0x44, 0x9c],  Instruction { size: 4, operation: FACOS, operands: [ ARIND(A6), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" facos.s (a6),fp1",
]);}
//  fadd fp0,fp1
#[test]
fn test_decode_0306_fadd_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa2],  Instruction { size: 10, operation: FADD, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fadd fp0,fp1",
]);}
//  fsadd.s (a6),fp1
#[test]
fn test_decode_0307_fsadd_s_a6_fp1() {
test_decoding_result_ok(&[0xf2, 0x16, 0x44, 0xe2],  Instruction { size: 4, operation: FSADD, operands: [ ARIND(A6), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsadd.s (a6),fp1",
]);}
//  fdadd.d (a6),fp1
#[test]
fn test_decode_0308_fdadd_d_a6_fp1() {
test_decoding_result_ok(&[0xf2, 0x16, 0x54, 0xe6],  Instruction { size: 8, operation: FDADD, operands: [ ARIND(A6), FR(FP1) ], extra: FloatFormat(FPF_DOUBLE) }
, &[" fdadd.d (a6),fp1",
]);}
//  fasin fp3
#[test]
fn test_decode_0309_fasin_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x8c],  Instruction { size: 10, operation: FASIN, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fasin fp3",
]);}
//  fasin fp0,fp1
#[test]
fn test_decode_0310_fasin_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x8c],  Instruction { size: 10, operation: FASIN, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fasin fp0,fp1",
]);}
//  fasin.s (a0),fp1
#[test]
fn test_decode_0311_fasin_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x8c],  Instruction { size: 4, operation: FASIN, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fasin.s (a0),fp1",
]);}
//  fatan fp3
#[test]
fn test_decode_0312_fatan_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x8a],  Instruction { size: 10, operation: FATAN, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fatan fp3",
]);}
//  fatan fp0,fp1
#[test]
fn test_decode_0313_fatan_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x8a],  Instruction { size: 10, operation: FATAN, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fatan fp0,fp1",
]);}
//  fatan.s (a0),fp1
#[test]
fn test_decode_0314_fatan_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x8a],  Instruction { size: 4, operation: FATAN, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fatan.s (a0),fp1",
]);}
//  fatanh fp3
#[test]
fn test_decode_0315_fatanh_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x8d],  Instruction { size: 10, operation: FATANH, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fatanh fp3",
]);}
//  fatanh fp0,fp1
#[test]
fn test_decode_0316_fatanh_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x8d],  Instruction { size: 10, operation: FATANH, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fatanh fp0,fp1",
]);}
//  fatanh.s (a0),fp1
#[test]
fn test_decode_0317_fatanh_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x8d],  Instruction { size: 4, operation: FATANH, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fatanh.s (a0),fp1",
]);}
//  lab: fbne.l lab
#[test]
fn test_decode_0318_lab_fbne_l_lab() {
test_decoding_result_ok(&[0xf2, 0xce, 0xff, 0xff, 0xff, 0xfe],  Instruction { size: 4, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NE) }
, &[" lab: fbne.l lab",
]);}
//  lab: fbf.w lab
#[test]
fn test_decode_0319_lab_fbf_w_lab() {
test_decoding_result_ok(&[0xf2, 0x80, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_F) }
, &[" lab: fbf.w lab",
]);}
//  lab: fbeq.w lab
#[test]
fn test_decode_0320_lab_fbeq_w_lab() {
test_decoding_result_ok(&[0xf2, 0x81, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_EQ) }
, &[" lab: fbeq.w lab",
]);}
//  lab: fbogt.w lab
#[test]
fn test_decode_0321_lab_fbogt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x82, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_OGT) }
, &[" lab: fbogt.w lab",
]);}
//  lab: fboge.w lab
#[test]
fn test_decode_0322_lab_fboge_w_lab() {
test_decoding_result_ok(&[0xf2, 0x83, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_OGE) }
, &[" lab: fboge.w lab",
]);}
//  lab: fbolt.w lab
#[test]
fn test_decode_0323_lab_fbolt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x84, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_OLT) }
, &[" lab: fbolt.w lab",
]);}
//  lab: fbole.w lab
#[test]
fn test_decode_0324_lab_fbole_w_lab() {
test_decoding_result_ok(&[0xf2, 0x85, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_OLE) }
, &[" lab: fbole.w lab",
]);}
//  lab: fbogl.w lab
#[test]
fn test_decode_0325_lab_fbogl_w_lab() {
test_decoding_result_ok(&[0xf2, 0x86, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_OGL) }
, &[" lab: fbogl.w lab",
]);}
//  lab: fbor.w lab
#[test]
fn test_decode_0326_lab_fbor_w_lab() {
test_decoding_result_ok(&[0xf2, 0x87, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_OR) }
, &[" lab: fbor.w lab",
]);}
//  lab: fbun.w lab
#[test]
fn test_decode_0327_lab_fbun_w_lab() {
test_decoding_result_ok(&[0xf2, 0x88, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_UN) }
, &[" lab: fbun.w lab",
]);}
//  lab: fbueq.w lab
#[test]
fn test_decode_0328_lab_fbueq_w_lab() {
test_decoding_result_ok(&[0xf2, 0x89, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_UEQ) }
, &[" lab: fbueq.w lab",
]);}
//  lab: fbugt.w lab
#[test]
fn test_decode_0329_lab_fbugt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x8a, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_UGT) }
, &[" lab: fbugt.w lab",
]);}
//  lab: fbuge.w lab
#[test]
fn test_decode_0330_lab_fbuge_w_lab() {
test_decoding_result_ok(&[0xf2, 0x8b, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_UGE) }
, &[" lab: fbuge.w lab",
]);}
//  lab: fbult.w lab
#[test]
fn test_decode_0331_lab_fbult_w_lab() {
test_decoding_result_ok(&[0xf2, 0x8c, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_ULT) }
, &[" lab: fbult.w lab",
]);}
//  lab: fbule.w lab
#[test]
fn test_decode_0332_lab_fbule_w_lab() {
test_decoding_result_ok(&[0xf2, 0x8d, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_ULE) }
, &[" lab: fbule.w lab",
]);}
//  lab: fbne.w lab
#[test]
fn test_decode_0333_lab_fbne_w_lab() {
test_decoding_result_ok(&[0xf2, 0x8e, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NE) }
, &[" lab: fbne.w lab",
]);}
//  lab: fbt.w lab
#[test]
fn test_decode_0334_lab_fbt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x8f, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_T) }
, &[" lab: fbt.w lab",
]);}
//  lab: fbsf.w lab
#[test]
fn test_decode_0335_lab_fbsf_w_lab() {
test_decoding_result_ok(&[0xf2, 0x90, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_SF) }
, &[" lab: fbsf.w lab",
]);}
//  lab: fbseq.w lab
#[test]
fn test_decode_0336_lab_fbseq_w_lab() {
test_decoding_result_ok(&[0xf2, 0x91, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_SEQ) }
, &[" lab: fbseq.w lab",
]);}
//  lab: fbgt.w lab
#[test]
fn test_decode_0337_lab_fbgt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x92, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_GT) }
, &[" lab: fbgt.w lab",
]);}
//  lab: fbge.w lab
#[test]
fn test_decode_0338_lab_fbge_w_lab() {
test_decoding_result_ok(&[0xf2, 0x93, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_GE) }
, &[" lab: fbge.w lab",
]);}
//  lab: fblt.w lab
#[test]
fn test_decode_0339_lab_fblt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x94, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_LT) }
, &[" lab: fblt.w lab",
]);}
//  lab: fble.w lab
#[test]
fn test_decode_0340_lab_fble_w_lab() {
test_decoding_result_ok(&[0xf2, 0x95, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_LE) }
, &[" lab: fble.w lab",
]);}
//  lab: fbgl.w lab
#[test]
fn test_decode_0341_lab_fbgl_w_lab() {
test_decoding_result_ok(&[0xf2, 0x96, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_GL) }
, &[" lab: fbgl.w lab",
]);}
//  lab: fbgle.w lab
#[test]
fn test_decode_0342_lab_fbgle_w_lab() {
test_decoding_result_ok(&[0xf2, 0x97, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_GLE) }
, &[" lab: fbgle.w lab",
]);}
//  lab: fbngle.w lab
#[test]
fn test_decode_0343_lab_fbngle_w_lab() {
test_decoding_result_ok(&[0xf2, 0x98, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NGLE) }
, &[" lab: fbngle.w lab",
]);}
//  lab: fbngl.w lab
#[test]
fn test_decode_0344_lab_fbngl_w_lab() {
test_decoding_result_ok(&[0xf2, 0x99, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NGL) }
, &[" lab: fbngl.w lab",
]);}
//  lab: fbnle.w lab
#[test]
fn test_decode_0345_lab_fbnle_w_lab() {
test_decoding_result_ok(&[0xf2, 0x9a, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NLE) }
, &[" lab: fbnle.w lab",
]);}
//  lab: fbnlt.w lab
#[test]
fn test_decode_0346_lab_fbnlt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x9b, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NLT) }
, &[" lab: fbnlt.w lab",
]);}
//  lab: fbnge.w lab
#[test]
fn test_decode_0347_lab_fbnge_w_lab() {
test_decoding_result_ok(&[0xf2, 0x9c, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NGE) }
, &[" lab: fbnge.w lab",
]);}
//  lab: fbngt.w lab
#[test]
fn test_decode_0348_lab_fbngt_w_lab() {
test_decoding_result_ok(&[0xf2, 0x9d, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_NGT) }
, &[" lab: fbngt.w lab",
]);}
//  lab: fbsne.w lab
#[test]
fn test_decode_0349_lab_fbsne_w_lab() {
test_decoding_result_ok(&[0xf2, 0x9e, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_SNE) }
, &[" lab: fbsne.w lab",
]);}
//  lab: fbst.w lab
#[test]
fn test_decode_0350_lab_fbst_w_lab() {
test_decoding_result_ok(&[0xf2, 0x9f, 0xff, 0xfe],  Instruction { size: 2, operation: FBCC, operands: [ PCDISP(2, simple_disp(-2)), NoOperand ], extra: FPCondition(FPCC_ST) }
, &[" lab: fbst.w lab",
]);}
//  fcmp fp2,fp4
#[test]
fn test_decode_0351_fcmp_fp2_fp4() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0a, 0x38],  Instruction { size: 10, operation: FCMP, operands: [ FR(FP2), FR(FP4) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fcmp fp2,fp4",
]);}
//  fcmp.s (a0),fp4
#[test]
fn test_decode_0352_fcmp_s_a0_fp4() {
test_decoding_result_ok(&[0xf2, 0x10, 0x46, 0x38],  Instruction { size: 4, operation: FCMP, operands: [ ARIND(A0), FR(FP4) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fcmp.s (a0),fp4",
]);}
//  fcos fp3
#[test]
fn test_decode_0353_fcos_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x9d],  Instruction { size: 10, operation: FCOS, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fcos fp3",
]);}
//  fcos fp0,fp1
#[test]
fn test_decode_0354_fcos_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x9d],  Instruction { size: 10, operation: FCOS, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fcos fp0,fp1",
]);}
//  fcos.s (a0),fp1
#[test]
fn test_decode_0355_fcos_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x9d],  Instruction { size: 4, operation: FCOS, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fcos.s (a0),fp1",
]);}
//  fcosh fp3
#[test]
fn test_decode_0356_fcosh_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x99],  Instruction { size: 10, operation: FCOSH, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fcosh fp3",
]);}
//  fcosh fp0,fp1
#[test]
fn test_decode_0357_fcosh_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x99],  Instruction { size: 10, operation: FCOSH, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fcosh fp0,fp1",
]);}
//  fcosh.s (a0),fp1
#[test]
fn test_decode_0358_fcosh_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x99],  Instruction { size: 4, operation: FCOSH, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fcosh.s (a0),fp1",
]);}
//  fdiv fp0,fp1
#[test]
fn test_decode_0359_fdiv_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa0],  Instruction { size: 10, operation: FDIV, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fdiv fp0,fp1",
]);}
//  fdiv.s (a0),fp1
#[test]
fn test_decode_0360_fdiv_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa0],  Instruction { size: 4, operation: FDIV, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fdiv.s (a0),fp1",
]);}
//  fsdiv.s (a0),fp1
#[test]
fn test_decode_0361_fsdiv_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xe0],  Instruction { size: 4, operation: FSDIV, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsdiv.s (a0),fp1",
]);}
//  fddiv.s (a0),fp1
#[test]
fn test_decode_0362_fddiv_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xe4],  Instruction { size: 4, operation: FDDIV, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fddiv.s (a0),fp1",
]);}
//  fetox fp3
#[test]
fn test_decode_0363_fetox_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x90],  Instruction { size: 10, operation: FETOX, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fetox fp3",
]);}
//  fetox fp0,fp1
#[test]
fn test_decode_0364_fetox_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x90],  Instruction { size: 10, operation: FETOX, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fetox fp0,fp1",
]);}
//  fetox.s (a0),fp1
#[test]
fn test_decode_0365_fetox_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x90],  Instruction { size: 4, operation: FETOX, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fetox.s (a0),fp1",
]);}
//  fetoxm1 fp3
#[test]
fn test_decode_0366_fetoxm1_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x88],  Instruction { size: 10, operation: FETOXM1, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fetoxm1 fp3",
]);}
//  fetoxm1 fp0,fp1
#[test]
fn test_decode_0367_fetoxm1_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x88],  Instruction { size: 10, operation: FETOXM1, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fetoxm1 fp0,fp1",
]);}
//  fetoxm1.s (a0),fp1
#[test]
fn test_decode_0368_fetoxm1_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x88],  Instruction { size: 4, operation: FETOXM1, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fetoxm1.s (a0),fp1",
]);}
//  fgetexp fp3
#[test]
fn test_decode_0369_fgetexp_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x9e],  Instruction { size: 10, operation: FGETEXP, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fgetexp fp3",
]);}
//  fgetexp fp0,fp1
#[test]
fn test_decode_0370_fgetexp_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x9e],  Instruction { size: 10, operation: FGETEXP, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fgetexp fp0,fp1",
]);}
//  fgetexp.s (a0),fp1
#[test]
fn test_decode_0371_fgetexp_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x9e],  Instruction { size: 4, operation: FGETEXP, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fgetexp.s (a0),fp1",
]);}
//  fgetman fp3
#[test]
fn test_decode_0372_fgetman_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x9f],  Instruction { size: 10, operation: FGETMAN, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fgetman fp3",
]);}
//  fgetman fp0,fp1
#[test]
fn test_decode_0373_fgetman_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x9f],  Instruction { size: 10, operation: FGETMAN, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fgetman fp0,fp1",
]);}
//  fgetman.s (a0),fp1
#[test]
fn test_decode_0374_fgetman_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x9f],  Instruction { size: 4, operation: FGETMAN, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fgetman.s (a0),fp1",
]);}
//  fint fp3
#[test]
fn test_decode_0375_fint_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x81],  Instruction { size: 10, operation: FINT, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fint fp3",
]);}
//  fint fp0,fp1
#[test]
fn test_decode_0376_fint_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x81],  Instruction { size: 10, operation: FINT, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fint fp0,fp1",
]);}
//  fint.s (a0),fp1
#[test]
fn test_decode_0377_fint_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x81],  Instruction { size: 4, operation: FINT, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fint.s (a0),fp1",
]);}
//  fintrz fp3
#[test]
fn test_decode_0378_fintrz_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x83],  Instruction { size: 10, operation: FINTRZ, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fintrz fp3",
]);}
//  fintrz fp0,fp1
#[test]
fn test_decode_0379_fintrz_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x83],  Instruction { size: 10, operation: FINTRZ, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fintrz fp0,fp1",
]);}
//  fintrz.s (a0),fp1
#[test]
fn test_decode_0380_fintrz_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x83],  Instruction { size: 4, operation: FINTRZ, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fintrz.s (a0),fp1",
]);}
//  flog10 fp3
#[test]
fn test_decode_0381_flog10_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x95],  Instruction { size: 10, operation: FLOG10, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flog10 fp3",
]);}
//  flog10 fp0,fp1
#[test]
fn test_decode_0382_flog10_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x95],  Instruction { size: 10, operation: FLOG10, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flog10 fp0,fp1",
]);}
//  flog10.s (a0),fp1
#[test]
fn test_decode_0383_flog10_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x95],  Instruction { size: 4, operation: FLOG10, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" flog10.s (a0),fp1",
]);}
//  flog2 fp3
#[test]
fn test_decode_0384_flog2_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x96],  Instruction { size: 10, operation: FLOG2, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flog2 fp3",
]);}
//  flog2 fp0,fp1
#[test]
fn test_decode_0385_flog2_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x96],  Instruction { size: 10, operation: FLOG2, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flog2 fp0,fp1",
]);}
//  flog2.s (a0),fp1
#[test]
fn test_decode_0386_flog2_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x96],  Instruction { size: 4, operation: FLOG2, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" flog2.s (a0),fp1",
]);}
//  flogn fp3
#[test]
fn test_decode_0387_flogn_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x94],  Instruction { size: 10, operation: FLOGN, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flogn fp3",
]);}
//  flogn fp0,fp1
#[test]
fn test_decode_0388_flogn_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x94],  Instruction { size: 10, operation: FLOGN, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flogn fp0,fp1",
]);}
//  flogn.s (a0),fp1
#[test]
fn test_decode_0389_flogn_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x94],  Instruction { size: 4, operation: FLOGN, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" flogn.s (a0),fp1",
]);}
//  flognp1 fp3
#[test]
fn test_decode_0390_flognp1_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x86],  Instruction { size: 10, operation: FLOGNP1, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flognp1 fp3",
]);}
//  flognp1 fp0,fp1
#[test]
fn test_decode_0391_flognp1_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x86],  Instruction { size: 10, operation: FLOGNP1, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" flognp1 fp0,fp1",
]);}
//  flognp1.s (a0),fp1
#[test]
fn test_decode_0392_flognp1_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x86],  Instruction { size: 4, operation: FLOGNP1, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" flognp1.s (a0),fp1",
]);}
//  fmod fp0,fp1
#[test]
fn test_decode_0393_fmod_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa1],  Instruction { size: 10, operation: FMOD, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmod fp0,fp1",
]);}
//  fmod.s (a0),fp1
#[test]
fn test_decode_0394_fmod_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa1],  Instruction { size: 4, operation: FMOD, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fmod.s (a0),fp1",
]);}
//  fmovecr #30,fp1
#[test]
fn test_decode_0395_fmovecr_30_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x5c, 0x9e],  Instruction { size: 10, operation: FMOVECR, operands: [ IMM8(30), FR(FP1) ], extra: NoExtra }
, &[" fmovecr #30,fp1",
]);}
//  lab: fdbgt d6,lab
#[test]
fn test_decode_0396_lab_fdbgt_d6_lab() {
test_decoding_result_ok(&[0xf2, 0x4e, 0x00, 0x12, 0xff, 0xfc],  Instruction { size: 2, operation: FDBCC, operands: [ DR(D6), PCDISP(4, simple_disp(-4)) ], extra: FPCondition(FPCC_GT) }
, &[" lab: fdbgt d6,lab",
]);}
//  fmove fp3,fp5
#[test]
fn test_decode_0397_fmove_fp3_fp5() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0e, 0x80],  Instruction { size: 10, operation: FMOVE, operands: [ FR(FP3), FR(FP5) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmove fp3,fp5",
]);}
//  fmove.x (a0),fp5
#[test]
fn test_decode_0398_fmove_x_a0_fp5() {
test_decoding_result_ok(&[0xf2, 0x10, 0x4a, 0x80],  Instruction { size: 10, operation: FMOVE, operands: [ ARIND(A0), FR(FP5) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmove.x (a0),fp5",
]);}
//  fmove.s (a0),fp5
#[test]
fn test_decode_0399_fmove_s_a0_fp5() {
test_decoding_result_ok(&[0xf2, 0x10, 0x46, 0x80],  Instruction { size: 4, operation: FMOVE, operands: [ ARIND(A0), FR(FP5) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fmove.s (a0),fp5",
]);}
//  fsmove.d (a0),fp5
#[test]
fn test_decode_0400_fsmove_d_a0_fp5() {
test_decoding_result_ok(&[0xf2, 0x10, 0x56, 0xc0],  Instruction { size: 8, operation: FSMOVE, operands: [ ARIND(A0), FR(FP5) ], extra: FloatFormat(FPF_DOUBLE) }
, &[" fsmove.d (a0),fp5",
]);}
//  fdmove.p (a0),fp5
#[test]
fn test_decode_0401_fdmove_p_a0_fp5() {
test_decoding_result_ok(&[0xf2, 0x10, 0x4e, 0xc4],  Instruction { size: 12, operation: FDMOVE, operands: [ ARIND(A0), FR(FP5) ], extra: FloatFormat(FPF_PACKED_DECIMAL_REAL_STATIC(0)) }
, &[" fdmove.p (a0),fp5",
]);}
//  fmove.s fp4,(a1)
#[test]
fn test_decode_0402_fmove_s_fp4_a1_() {
test_decoding_result_ok(&[0xf2, 0x11, 0x66, 0x00],  Instruction { size: 4, operation: FMOVE, operands: [ FR(FP4), ARIND(A1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fmove.s fp4,(a1)",
]);}
//  fmove.p fp4,(a1){#12}
#[test]
fn test_decode_0403_fmove_p_fp4_a1_12_() {
test_decoding_result_ok(&[0xf2, 0x11, 0x6e, 0x0c],  Instruction { size: 12, operation: FMOVE, operands: [ FR(FP4), ARIND(A1) ], extra: FloatFormat(FPF_PACKED_DECIMAL_REAL_STATIC(12)) }
, &[" fmove.p fp4,(a1){#12}",
]);}
//  fmove.p fp4,(a1){#-64}
#[test]
fn test_decode_0404_fmove_p_fp4_a1_64_() {
test_decoding_result_ok(&[0xf2, 0x11, 0x6e, 0x40],  Instruction { size: 12, operation: FMOVE, operands: [ FR(FP4), ARIND(A1) ], extra: FloatFormat(FPF_PACKED_DECIMAL_REAL_STATIC(-64)) }
, &[" fmove.p fp4,(a1){#-64}",
]);}
//  fmove.p fp4,(a1){#63}
#[test]
fn test_decode_0405_fmove_p_fp4_a1_63_() {
test_decoding_result_ok(&[0xf2, 0x11, 0x6e, 0x3f],  Instruction { size: 12, operation: FMOVE, operands: [ FR(FP4), ARIND(A1) ], extra: FloatFormat(FPF_PACKED_DECIMAL_REAL_STATIC(63)) }
, &[" fmove.p fp4,(a1){#63}",
]);}
//  fmove.p fp4,(a1){d3}
#[test]
fn test_decode_0406_fmove_p_fp4_a1_d3_() {
test_decoding_result_ok(&[0xf2, 0x11, 0x7e, 0x30],  Instruction { size: 12, operation: FMOVE, operands: [ FR(FP4), ARIND(A1) ], extra: FloatFormat(FPF_PACKED_DECIMAL_REAL_DYNAMIC(D3)) }
, &[" fmove.p fp4,(a1){d3}",
]);}
//  fmovem.x fp0-fp4,-(a3)
#[test]
fn test_decode_0407_fmovem_x_fp0_fp4_a3_() {
test_decoding_result_ok(&[0xf2, 0x23, 0xe0, 0x1f],  Instruction { size: 10, operation: FMOVEM, operands: [ REGLIST(0b11111), ARDEC(A3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmovem.x fp0-fp4,-(a3)",
]);}
//  fmovem.x d7,-(a3)
#[test]
fn test_decode_0408_fmovem_x_d7_a3_() {
test_decoding_result_ok(&[0xf2, 0x23, 0xe8, 0x70],  Instruction { size: 10, operation: FMOVEM, operands: [ DR(D7), ARDEC(A3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmovem.x d7,-(a3)",
]);}
//  fmovem.x d7,(a3)
#[test]
fn test_decode_0409_fmovem_x_d7_a3_() {
test_decoding_result_ok(&[0xf2, 0x13, 0xf8, 0x70],  Instruction { size: 10, operation: FMOVEM, operands: [ DR(D7), ARIND(A3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmovem.x d7,(a3)",
]);}
//  fmovem.x (a3),d7
#[test]
fn test_decode_0410_fmovem_x_a3_d7() {
test_decoding_result_ok(&[0xf2, 0x13, 0xd8, 0x70],  Instruction { size: 10, operation: FMOVEM, operands: [ ARIND(A3), DR(D7) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmovem.x (a3),d7",
]);}
//  fmovem.x (a3)+,d7
#[test]
fn test_decode_0411_fmovem_x_a3_d7() {
test_decoding_result_ok(&[0xf2, 0x1b, 0xd8, 0x70],  Instruction { size: 10, operation: FMOVEM, operands: [ ARINC(A3), DR(D7) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmovem.x (a3)+,d7",
]);}
//  fmovem.x (a3)+,fp0/fp6
#[test]
fn test_decode_0412_fmovem_x_a3_fp0_fp6() {
test_decoding_result_ok(&[0xf2, 0x1b, 0xd0, 0x82],  Instruction { size: 10, operation: FMOVEM, operands: [ ARINC(A3), REGLIST(0b1000_0010) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmovem.x (a3)+,fp0/fp6",
]);}
//  fmul fp0,fp1
#[test]
fn test_decode_0413_fmul_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa3],  Instruction { size: 10, operation: FMUL, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fmul fp0,fp1",
]);}
//  fmul.s (a0),fp1
#[test]
fn test_decode_0414_fmul_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa3],  Instruction { size: 4, operation: FMUL, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fmul.s (a0),fp1",
]);}
//  fsmul.s (a0),fp1
#[test]
fn test_decode_0415_fsmul_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xe3],  Instruction { size: 4, operation: FSMUL, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsmul.s (a0),fp1",
]);}
//  fdmul.s (a0),fp1
#[test]
fn test_decode_0416_fdmul_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xe7],  Instruction { size: 4, operation: FDMUL, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fdmul.s (a0),fp1",
]);}
//  fneg fp3
#[test]
fn test_decode_0417_fneg_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x9a],  Instruction { size: 10, operation: FNEG, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fneg fp3",
]);}
//  fneg fp0,fp1
#[test]
fn test_decode_0418_fneg_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x9a],  Instruction { size: 10, operation: FNEG, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fneg fp0,fp1",
]);}
//  fneg.s (a0),fp1
#[test]
fn test_decode_0419_fneg_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x9a],  Instruction { size: 4, operation: FNEG, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fneg.s (a0),fp1",
]);}
//  fsneg.s (a0),fp1
#[test]
fn test_decode_0420_fsneg_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xda],  Instruction { size: 4, operation: FSNEG, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsneg.s (a0),fp1",
]);}
//  fdneg.s (a0),fp1
#[test]
fn test_decode_0421_fdneg_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xde],  Instruction { size: 4, operation: FDNEG, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fdneg.s (a0),fp1",
]);}
//  fnop
#[test]
fn test_decode_0422_fnop() {
test_decoding_result_ok(&[0xf2, 0x80, 0x00, 0x00],  Instruction { size: 0, operation: FNOP, operands: [ NoOperand, NoOperand ], extra: NoExtra }
, &[" fnop",
]);}
//  frem fp0,fp1
#[test]
fn test_decode_0423_frem_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa5],  Instruction { size: 10, operation: FREM, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" frem fp0,fp1",
]);}
//  frem.s (a0),fp1
#[test]
fn test_decode_0424_frem_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa5],  Instruction { size: 4, operation: FREM, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" frem.s (a0),fp1",
]);}
//  fscale fp0,fp1
#[test]
fn test_decode_0425_fscale_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa6],  Instruction { size: 10, operation: FSCALE, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fscale fp0,fp1",
]);}
//  fscale.s (a0),fp1
#[test]
fn test_decode_0426_fscale_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa6],  Instruction { size: 4, operation: FSCALE, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fscale.s (a0),fp1",
]);}
//  fsgt (a0)
#[test]
fn test_decode_0427_fsgt_a0_() {
test_decoding_result_ok(&[0xf2, 0x50, 0x00, 0x12],  Instruction { size: 1, operation: FSCC, operands: [ Implied, ARIND(A0) ], extra: FPCondition(FPCC_GT) }
, &[" fsgt (a0)",
]);}
//  fsgldiv fp0,fp1
#[test]
fn test_decode_0428_fsgldiv_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa4],  Instruction { size: 10, operation: FSGLDIV, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsgldiv fp0,fp1",
]);}
//  fsgldiv.s (a0),fp1
#[test]
fn test_decode_0429_fsgldiv_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa4],  Instruction { size: 4, operation: FSGLDIV, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsgldiv.s (a0),fp1",
]);}
//  fsglmul fp0,fp1
#[test]
fn test_decode_0430_fsglmul_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa7],  Instruction { size: 10, operation: FSGLMUL, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsglmul fp0,fp1",
]);}
//  fsglmul.s (a0),fp1
#[test]
fn test_decode_0431_fsglmul_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa7],  Instruction { size: 4, operation: FSGLMUL, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsglmul.s (a0),fp1",
]);}
//  fsin fp3
#[test]
fn test_decode_0432_fsin_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x8e],  Instruction { size: 10, operation: FSIN, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsin fp3",
]);}
//  fsin fp0,fp1
#[test]
fn test_decode_0433_fsin_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x8e],  Instruction { size: 10, operation: FSIN, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsin fp0,fp1",
]);}
//  fsin.s (a0),fp1
#[test]
fn test_decode_0434_fsin_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x8e],  Instruction { size: 4, operation: FSIN, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsin.s (a0),fp1",
]);}
//  fsincos fp0,fp1:fp2
#[test]
fn test_decode_0435_fsincos_fp0_fp1_fp2() {
test_decoding_result_ok(&[0xf2, 0x00, 0x01, 0x31],  Instruction { size: 10, operation: FSINCOS, operands: [ FR(FP0), FPAIR(FP2,FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsincos fp0,fp1:fp2",
]);}
//  fsincos.s (a0),fp1:fp2
#[test]
fn test_decode_0436_fsincos_s_a0_fp1_fp2() {
test_decoding_result_ok(&[0xf2, 0x10, 0x45, 0x31],  Instruction { size: 4, operation: FSINCOS, operands: [ ARIND(A0), FPAIR(FP2,FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsincos.s (a0),fp1:fp2",
]);}
//  fsinh fp3
#[test]
fn test_decode_0437_fsinh_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x82],  Instruction { size: 10, operation: FSINH, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsinh fp3",
]);}
//  fsinh fp0,fp1
#[test]
fn test_decode_0438_fsinh_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x82],  Instruction { size: 10, operation: FSINH, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsinh fp0,fp1",
]);}
//  fsinh.s (a0),fp1
#[test]
fn test_decode_0439_fsinh_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x82],  Instruction { size: 4, operation: FSINH, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsinh.s (a0),fp1",
]);}
//  fsqrt fp3
#[test]
fn test_decode_0440_fsqrt_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x84],  Instruction { size: 10, operation: FSQRT, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsqrt fp3",
]);}
//  fsqrt fp0,fp1
#[test]
fn test_decode_0441_fsqrt_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x84],  Instruction { size: 10, operation: FSQRT, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsqrt fp0,fp1",
]);}
//  fsqrt.s (a0),fp1
#[test]
fn test_decode_0442_fsqrt_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x84],  Instruction { size: 4, operation: FSQRT, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsqrt.s (a0),fp1",
]);}
//  fssqrt fp3
#[test]
fn test_decode_0443_fssqrt_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0xc1],  Instruction { size: 10, operation: FSSQRT, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fssqrt fp3",
]);}
//  fdsqrt fp3
#[test]
fn test_decode_0444_fdsqrt_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0xc5],  Instruction { size: 10, operation: FDSQRT, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fdsqrt fp3",
]);}
//  fsub fp0,fp1
#[test]
fn test_decode_0445_fsub_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0xa8],  Instruction { size: 10, operation: FSUB, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fsub fp0,fp1",
]);}
//  fsub.s (a0),fp1
#[test]
fn test_decode_0446_fsub_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0xa8],  Instruction { size: 4, operation: FSUB, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" fsub.s (a0),fp1",
]);}
//  fssub.x (a0),fp1
#[test]
fn test_decode_0447_fssub_x_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x48, 0xe8],  Instruction { size: 10, operation: FSSUB, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" fssub.x (a0),fp1",
]);}
//  fdsub.l (a0),fp1
#[test]
fn test_decode_0448_fdsub_l_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x40, 0xec],  Instruction { size: 4, operation: FDSUB, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_LONG_INT) }
, &[" fdsub.l (a0),fp1",
]);}
//  ftan fp3
#[test]
fn test_decode_0449_ftan_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x8f],  Instruction { size: 10, operation: FTAN, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftan fp3",
]);}
//  ftan fp0,fp1
#[test]
fn test_decode_0450_ftan_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x8f],  Instruction { size: 10, operation: FTAN, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftan fp0,fp1",
]);}
//  ftan.s (a0),fp1
#[test]
fn test_decode_0451_ftan_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x8f],  Instruction { size: 4, operation: FTAN, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" ftan.s (a0),fp1",
]);}
//  ftanh fp3
#[test]
fn test_decode_0452_ftanh_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x89],  Instruction { size: 10, operation: FTANH, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftanh fp3",
]);}
//  ftanh fp0,fp1
#[test]
fn test_decode_0453_ftanh_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x89],  Instruction { size: 10, operation: FTANH, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftanh fp0,fp1",
]);}
//  ftanh.s (a0),fp1
#[test]
fn test_decode_0454_ftanh_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x89],  Instruction { size: 4, operation: FTANH, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" ftanh.s (a0),fp1",
]);}
//  ftentox fp3
#[test]
fn test_decode_0455_ftentox_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x92],  Instruction { size: 10, operation: FTENTOX, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftentox fp3",
]);}
//  ftentox fp0,fp1
#[test]
fn test_decode_0456_ftentox_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x92],  Instruction { size: 10, operation: FTENTOX, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftentox fp0,fp1",
]);}
//  ftentox.s (a0),fp1
#[test]
fn test_decode_0457_ftentox_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x92],  Instruction { size: 4, operation: FTENTOX, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" ftentox.s (a0),fp1",
]);}
//  ftrapgt
#[test]
fn test_decode_0458_ftrapgt() {
test_decoding_result_ok(&[0xf2, 0x7c, 0x00, 0x12],  Instruction { size: 0, operation: FTRAPCC, operands: [ Implied, NoOperand ], extra: FPCondition(FPCC_GT) }
, &[" ftrapgt",
]);}
//  ftrapeq.w #123
#[test]
fn test_decode_0459_ftrapeq_w_123() {
test_decoding_result_ok(&[0xf2, 0x7a, 0x00, 0x01, 0x00, 0x7b],  Instruction { size: 2, operation: FTRAPCC, operands: [ Implied, IMM16(123) ], extra: FPCondition(FPCC_EQ) }
, &[" ftrapeq.w #123",
]);}
//  ftrapne.l #1234567
#[test]
fn test_decode_0460_ftrapne_l_1234567() {
test_decoding_result_ok(&[0xf2, 0x7b, 0x00, 0x0e, 0x00, 0x12, 0xd6, 0x87],  Instruction { size: 4, operation: FTRAPCC, operands: [ Implied, IMM32(1234567) ], extra: FPCondition(FPCC_NE) }
, &[" ftrapne.l #1234567",
]);}
//  ftst.l (a0)
#[test]
fn test_decode_0461_ftst_l_a0_() {
test_decoding_result_ok(&[0xf2, 0x10, 0x40, 0x3a],  Instruction { size: 4, operation: FTST, operands: [ ARIND(A0), NoOperand ], extra: FloatFormat(FPF_LONG_INT) }
, &[" ftst.l (a0)",
]);}
//  ftst fp7
#[test]
fn test_decode_0462_ftst_fp7() {
test_decoding_result_ok(&[0xf2, 0x00, 0x1c, 0x3a],  Instruction { size: 10, operation: FTST, operands: [ FR(FP7), NoOperand ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftst fp7",
]);}
//  ftwotox fp3
#[test]
fn test_decode_0463_ftwotox_fp3() {
test_decoding_result_ok(&[0xf2, 0x00, 0x0d, 0x91],  Instruction { size: 10, operation: FTWOTOX, operands: [ FR(FP3), FR(FP3) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftwotox fp3",
]);}
//  ftwotox fp0,fp1
#[test]
fn test_decode_0464_ftwotox_fp0_fp1() {
test_decoding_result_ok(&[0xf2, 0x00, 0x00, 0x91],  Instruction { size: 10, operation: FTWOTOX, operands: [ FR(FP0), FR(FP1) ], extra: FloatFormat(FPF_EXTENDED_REAL) }
, &[" ftwotox fp0,fp1",
]);}
//  ftwotox.s (a0),fp1
#[test]
fn test_decode_0465_ftwotox_s_a0_fp1() {
test_decoding_result_ok(&[0xf2, 0x10, 0x44, 0x91],  Instruction { size: 4, operation: FTWOTOX, operands: [ ARIND(A0), FR(FP1) ], extra: FloatFormat(FPF_SINGLE) }
, &[" ftwotox.s (a0),fp1",
]);}
}
