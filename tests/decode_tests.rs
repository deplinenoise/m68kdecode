// auto-generated from decoding_tests.txt by gen_decoding_tests.py
mod tests {
    use m68kdecode::*;
    //  move.b d0,d1
    #[test]
    fn test_decode_0001_move_b_d0_d1() {
        test_decoding_result_ok(
            &[0x12, 0x00],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D0), DR(D1)],
            },
            &[" move.b d0,d1"],
        );
    }
    //  move.b d2,d3
    #[test]
    fn test_decode_0002_move_b_d2_d3() {
        test_decoding_result_ok(
            &[0x16, 0x02],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D2), DR(D3)],
            },
            &[" move.b d2,d3"],
        );
    }
    //  move.b d4,d5
    #[test]
    fn test_decode_0003_move_b_d4_d5() {
        test_decoding_result_ok(
            &[0x1a, 0x04],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D4), DR(D5)],
            },
            &[" move.b d4,d5"],
        );
    }
    //  move.b d6,d7
    #[test]
    fn test_decode_0004_move_b_d6_d7() {
        test_decoding_result_ok(
            &[0x1e, 0x06],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D6), DR(D7)],
            },
            &[" move.b d6,d7"],
        );
    }
    //  move.w a0,a1
    #[test]
    fn test_decode_0005_move_w_a0_a1() {
        test_decoding_result_ok(
            &[0x32, 0x48],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A0), AR(A1)],
            },
            &[" move.w a0,a1"],
        );
    }
    //  move.w a2,a3
    #[test]
    fn test_decode_0006_move_w_a2_a3() {
        test_decoding_result_ok(
            &[0x36, 0x4a],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A2), AR(A3)],
            },
            &[" move.w a2,a3"],
        );
    }
    //  move.w a4,a5
    #[test]
    fn test_decode_0007_move_w_a4_a5() {
        test_decoding_result_ok(
            &[0x3a, 0x4c],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A4), AR(A5)],
            },
            &[" move.w a4,a5"],
        );
    }
    //  move.w a6,a7
    #[test]
    fn test_decode_0008_move_w_a6_a7() {
        test_decoding_result_ok(
            &[0x3e, 0x4e],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A6), AR(A7)],
            },
            &[" move.w a6,a7"],
        );
    }
    //  move.b 123(a0,d0),d3
    #[test]
    fn test_decode_0009_move_b_123_a0_d0_d3() {
        test_decoding_result_ok(
            &[0x16, 0x30, 0x00, 0x7b],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [ARDISP(A0, dr_disp(D0, 123)), DR(D3)],
            },
            &[" move.b 123(a0,d0),d3"],
        );
    }
    //  move.w 123(a0,d0),d3
    #[test]
    fn test_decode_0010_move_w_123_a0_d0_d3() {
        test_decoding_result_ok(
            &[0x36, 0x30, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [ARDISP(A0, dr_disp(D0, 123)), DR(D3)],
            },
            &[" move.w 123(a0,d0),d3"],
        );
    }
    //  move.l 123(a0,d0),d3
    #[test]
    fn test_decode_0011_move_l_123_a0_d0_d3() {
        test_decoding_result_ok(
            &[0x26, 0x30, 0x00, 0x7b],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [ARDISP(A0, dr_disp(D0, 123)), DR(D3)],
            },
            &[" move.l 123(a0,d0),d3"],
        );
    }
    //  move.l 123(a0,d0),a1
    #[test]
    fn test_decode_0012_move_l_123_a0_d0_a1() {
        test_decoding_result_ok(
            &[0x22, 0x70, 0x00, 0x7b],
            Instruction {
                size: 4,
                operation: MOVEA,
                operands: [ARDISP(A0, dr_disp(D0, 123)), AR(A1)],
            },
            &[" move.l 123(a0,d0),a1"],
        );
    }
    //  move.w 123(a0,d0),a1
    #[test]
    fn test_decode_0013_move_w_123_a0_d0_a1() {
        test_decoding_result_ok(
            &[0x32, 0x70, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [ARDISP(A0, dr_disp(D0, 123)), AR(A1)],
            },
            &[" move.w 123(a0,d0),a1"],
        );
    }
    //  move.b #$12,d7
    #[test]
    fn test_decode_0014_move_b_12_d7() {
        test_decoding_result_ok(
            &[0x1e, 0x3c, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [IMM8(0x12), DR(D7)],
            },
            &[" move.b #$12,d7"],
        );
    }
    //  move.w #$1234,d7
    #[test]
    fn test_decode_0015_move_w_1234_d7() {
        test_decoding_result_ok(
            &[0x3e, 0x3c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [IMM16(0x1234), DR(D7)],
            },
            &[" move.w #$1234,d7"],
        );
    }
    //  move.l #$12345678,d7
    #[test]
    fn test_decode_0016_move_l_12345678_d7() {
        test_decoding_result_ok(
            &[0x2e, 0x3c, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [IMM32(0x12345678), DR(D7)],
            },
            &[" move.l #$12345678,d7"],
        );
    }
    //  move.l D1,-(A2)
    #[test]
    fn test_decode_0017_move_l_d1_a2_() {
        test_decoding_result_ok(
            &[0x25, 0x01],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [DR(D1), ARDEC(A2)],
            },
            &[" move.l D1,-(A2)"],
        );
    }
    //  move.l D1,(A2)+
    #[test]
    fn test_decode_0018_move_l_d1_a2_() {
        test_decoding_result_ok(
            &[0x24, 0xc1],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [DR(D1), ARINC(A2)],
            },
            &[" move.l D1,(A2)+"],
        );
    }
    //  move.l -(A4),(A2)+
    #[test]
    fn test_decode_0019_move_l_a4_a2_() {
        test_decoding_result_ok(
            &[0x24, 0xe4],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [ARDEC(A4), ARINC(A2)],
            },
            &[" move.l -(A4),(A2)+"],
        );
    }
    //  move.l 4.w,A0
    #[test]
    fn test_decode_0020_move_l_4_w_a0() {
        test_decoding_result_ok(
            &[0x20, 0x78, 0x00, 0x04],
            Instruction {
                size: 4,
                operation: MOVEA,
                operands: [ABS16(4), AR(A0)],
            },
            &[" move.l 4.w,A0"],
        );
    }
    //  move.l $11223344,A0
    #[test]
    fn test_decode_0021_move_l_11223344_a0() {
        test_decoding_result_ok(
            &[0x20, 0x79, 0x11, 0x22, 0x33, 0x44],
            Instruction {
                size: 4,
                operation: MOVEA,
                operands: [ABS32(0x11223344), AR(A0)],
            },
            &[" move.l $11223344,A0"],
        );
    }
    //  move.w #$1234,123(d0)
    #[test]
    fn test_decode_0022_move_w_1234_123_d0_() {
        test_decoding_result_ok(
            &[0x31, 0xbc, 0x12, 0x34, 0x01, 0xa0, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [
                    IMM16(0x1234),
                    DISP(Displacement {
                        base_displacement: 123,
                        outer_displacement: 0,
                        indexer: Indexer::DR(D0, 0),
                        indirection: NoIndirection,
                    }),
                ],
            },
            &[" move.w #$1234,123(d0)"],
        );
    }
    //  move.w -8(pc),d3
    #[test]
    fn test_decode_0023_move_w_8_pc_d3() {
        test_decoding_result_ok(
            &[0x36, 0x3a, 0xff, 0xf8],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [
                    PCDISP(Displacement {
                        base_displacement: -8,
                        outer_displacement: 0,
                        indexer: Indexer::NoIndexer,
                        indirection: NoIndirection,
                    }),
                    DR(D3),
                ],
            },
            &[" move.w -8(pc),d3"],
        );
    }
    //  move.w -8(pc,d2*8),d3
    #[test]
    fn test_decode_0024_move_w_8_pc_d2_8_d3() {
        test_decoding_result_ok(
            &[0x36, 0x3b, 0x26, 0xf8],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [
                    PCDISP(Displacement {
                        base_displacement: -8,
                        outer_displacement: 0,
                        indexer: Indexer::DR(D2, 3),
                        indirection: NoIndirection,
                    }),
                    DR(D3),
                ],
            },
            &[" move.w -8(pc,d2*8),d3"],
        );
    }
    //  move.w 123(a1,d2*4),9876(a2,d3*2)
    #[test]
    fn test_decode_0025_move_w_123_a1_d2_4_9876_a2_d3_2_() {
        test_decoding_result_ok(
            &[0x35, 0xb1, 0x24, 0x7b, 0x33, 0x20, 0x26, 0x94],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [
                    ARDISP(A1, dr_disp_scale(D2, 123, 2)),
                    ARDISP(A2, dr_disp_scale(D3, 9876, 1)),
                ],
            },
            &[" move.w 123(a1,d2*4),9876(a2,d3*2)"],
        );
    }
    //  move.w d0,12345(a0,a1*2)
    #[test]
    fn test_decode_0026_move_w_d0_12345_a0_a1_2_() {
        test_decoding_result_ok(
            &[0x31, 0x80, 0x93, 0x20, 0x30, 0x39],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [
                    DR(D0),
                    ARDISP(
                        A0,
                        Displacement {
                            base_displacement: 12345,
                            outer_displacement: 0,
                            indexer: Indexer::AR(A1, 1),
                            indirection: NoIndirection,
                        },
                    ),
                ],
            },
            &[" move.w d0,12345(a0,a1*2)"],
        );
    }
    //  lea (a0),a1
    #[test]
    fn test_decode_0027_lea_a0_a1() {
        test_decoding_result_ok(
            &[0x43, 0xd0],
            Instruction {
                size: 4,
                operation: LEA,
                operands: [ARIND(A0), AR(A1)],
            },
            &[" lea (a0),a1"],
        );
    }
    //  lea 8(a0),a1
    #[test]
    fn test_decode_0028_lea_8_a0_a1() {
        test_decoding_result_ok(
            &[0x43, 0xe8, 0x00, 0x08],
            Instruction {
                size: 4,
                operation: LEA,
                operands: [ARDISP(A0, simple_disp(8)), AR(A1)],
            },
            &[" lea 8(a0),a1"],
        );
    }
    //  ori #17,ccr
    #[test]
    fn test_decode_0029_ori_17_ccr() {
        test_decoding_result_ok(
            &[0x00, 0x3c, 0x00, 0x11],
            Instruction {
                size: 1,
                operation: ORITOCCR,
                operands: [IMM8(17), Implied],
            },
            &[" ori #17,ccr"],
        );
    }
    //  ori #$1234,sr
    #[test]
    fn test_decode_0030_ori_1234_sr() {
        test_decoding_result_ok(
            &[0x00, 0x7c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ORITOSR,
                operands: [IMM16(0x1234), Implied],
            },
            &[" ori #$1234,sr"],
        );
    }
    //  ori.w #$1234,d0
    #[test]
    fn test_decode_0031_ori_w_1234_d0() {
        test_decoding_result_ok(
            &[0x00, 0x40, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ORI,
                operands: [IMM16(0x1234), DR(D0)],
            },
            &[" ori.w #$1234,d0"],
        );
    }
    //  ori.b #$12,d2
    #[test]
    fn test_decode_0032_ori_b_12_d2() {
        test_decoding_result_ok(
            &[0x00, 0x02, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: ORI,
                operands: [IMM8(0x12), DR(D2)],
            },
            &[" ori.b #$12,d2"],
        );
    }
    //  ori.w #$1234,123(a0,d0)
    #[test]
    fn test_decode_0033_ori_w_1234_123_a0_d0_() {
        test_decoding_result_ok(
            &[0x00, 0x70, 0x12, 0x34, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: ORI,
                operands: [IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123))],
            },
            &[" ori.w #$1234,123(a0,d0)"],
        );
    }
    //  ori.l #$12345678,-(a0)
    #[test]
    fn test_decode_0034_ori_l_12345678_a0_() {
        test_decoding_result_ok(
            &[0x00, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: ORI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
            &[" ori.l #$12345678,-(a0)"],
        );
    }
    //  andi #17,ccr
    #[test]
    fn test_decode_0035_andi_17_ccr() {
        test_decoding_result_ok(
            &[0x02, 0x3c, 0x00, 0x11],
            Instruction {
                size: 1,
                operation: ANDITOCCR,
                operands: [IMM8(17), Implied],
            },
            &[" andi #17,ccr"],
        );
    }
    //  andi #$1234,sr
    #[test]
    fn test_decode_0036_andi_1234_sr() {
        test_decoding_result_ok(
            &[0x02, 0x7c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ANDITOSR,
                operands: [IMM16(0x1234), Implied],
            },
            &[" andi #$1234,sr"],
        );
    }
    //  andi.w #$1234,d0
    #[test]
    fn test_decode_0037_andi_w_1234_d0() {
        test_decoding_result_ok(
            &[0x02, 0x40, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ANDI,
                operands: [IMM16(0x1234), DR(D0)],
            },
            &[" andi.w #$1234,d0"],
        );
    }
    //  andi.b #$12,d2
    #[test]
    fn test_decode_0038_andi_b_12_d2() {
        test_decoding_result_ok(
            &[0x02, 0x02, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: ANDI,
                operands: [IMM8(0x12), DR(D2)],
            },
            &[" andi.b #$12,d2"],
        );
    }
    //  andi.w #$1234,123(a0,d0)
    #[test]
    fn test_decode_0039_andi_w_1234_123_a0_d0_() {
        test_decoding_result_ok(
            &[0x02, 0x70, 0x12, 0x34, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: ANDI,
                operands: [IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123))],
            },
            &[" andi.w #$1234,123(a0,d0)"],
        );
    }
    //  andi.l #$12345678,-(a0)
    #[test]
    fn test_decode_0040_andi_l_12345678_a0_() {
        test_decoding_result_ok(
            &[0x02, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: ANDI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
            &[" andi.l #$12345678,-(a0)"],
        );
    }
    //  eori #17,ccr
    #[test]
    fn test_decode_0041_eori_17_ccr() {
        test_decoding_result_ok(
            &[0x0a, 0x3c, 0x00, 0x11],
            Instruction {
                size: 1,
                operation: EORITOCCR,
                operands: [IMM8(17), Implied],
            },
            &[" eori #17,ccr"],
        );
    }
    //  eori #$1234,sr
    #[test]
    fn test_decode_0042_eori_1234_sr() {
        test_decoding_result_ok(
            &[0x0a, 0x7c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: EORITOSR,
                operands: [IMM16(0x1234), Implied],
            },
            &[" eori #$1234,sr"],
        );
    }
    //  eori.w #$1234,d0
    #[test]
    fn test_decode_0043_eori_w_1234_d0() {
        test_decoding_result_ok(
            &[0x0a, 0x40, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: EORI,
                operands: [IMM16(0x1234), DR(D0)],
            },
            &[" eori.w #$1234,d0"],
        );
    }
    //  eori.b #$12,d2
    #[test]
    fn test_decode_0044_eori_b_12_d2() {
        test_decoding_result_ok(
            &[0x0a, 0x02, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: EORI,
                operands: [IMM8(0x12), DR(D2)],
            },
            &[" eori.b #$12,d2"],
        );
    }
    //  eori.w #$1234,123(a0,d0)
    #[test]
    fn test_decode_0045_eori_w_1234_123_a0_d0_() {
        test_decoding_result_ok(
            &[0x0a, 0x70, 0x12, 0x34, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: EORI,
                operands: [IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123))],
            },
            &[" eori.w #$1234,123(a0,d0)"],
        );
    }
    //  eori.l #$12345678,-(a0)
    #[test]
    fn test_decode_0046_eori_l_12345678_a0_() {
        test_decoding_result_ok(
            &[0x0a, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: EORI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
            &[" eori.l #$12345678,-(a0)"],
        );
    }
    //  addi.l #$12345678,-(a0)
    #[test]
    fn test_decode_0047_addi_l_12345678_a0_() {
        test_decoding_result_ok(
            &[0x06, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: ADDI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
            &[" addi.l #$12345678,-(a0)"],
        );
    }
    //  subi.l #$12345678,-(a0)
    #[test]
    fn test_decode_0048_subi_l_12345678_a0_() {
        test_decoding_result_ok(
            &[0x04, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: SUBI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
            &[" subi.l #$12345678,-(a0)"],
        );
    }
    //  rtm d3
    #[test]
    fn test_decode_0049_rtm_d3() {
        test_decoding_result_ok(
            &[0x06, 0xc3],
            Instruction {
                size: 0,
                operation: RTM,
                operands: [DR(D3), NoOperand],
            },
            &[" rtm d3"],
        );
    }
    //  rtm a1
    #[test]
    fn test_decode_0050_rtm_a1() {
        test_decoding_result_ok(
            &[0x06, 0xc9],
            Instruction {
                size: 0,
                operation: RTM,
                operands: [AR(A1), NoOperand],
            },
            &[" rtm a1"],
        );
    }
    //  callm #3,(a1)
    #[test]
    fn test_decode_0051_callm_3_a1_() {
        test_decoding_result_ok(
            &[0x06, 0xd1, 0x00, 0x03],
            Instruction {
                size: 0,
                operation: CALLM,
                operands: [IMM8(3), ARIND(A1)],
            },
            &[" callm #3,(a1)"],
        );
    }
    //  callm #99,$12345678
    #[test]
    fn test_decode_0052_callm_99_12345678() {
        test_decoding_result_ok(
            &[0x06, 0xf9, 0x00, 0x63, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 0,
                operation: CALLM,
                operands: [IMM8(99), ABS32(0x12345678)],
            },
            &[" callm #99,$12345678"],
        );
    }
    //  cmp2.l (a0),d3
    #[test]
    fn test_decode_0053_cmp2_l_a0_d3() {
        test_decoding_result_ok(
            &[0x04, 0xd0, 0x30, 0x00],
            Instruction {
                size: 4,
                operation: CMP2,
                operands: [ARIND(A0), DR(D3)],
            },
            &[" cmp2.l (a0),d3"],
        );
    }
    //  cmp2.b 90(a0,d2),a6
    #[test]
    fn test_decode_0054_cmp2_b_90_a0_d2_a6() {
        test_decoding_result_ok(
            &[0x00, 0xf0, 0xe0, 0x00, 0x20, 0x5a],
            Instruction {
                size: 1,
                operation: CMP2,
                operands: [ARDISP(A0, dr_disp(D2, 90)), AR(A6)],
            },
            &[" cmp2.b 90(a0,d2),a6"],
        );
    }
    //  chk2.w 90(a0,d2),a6
    #[test]
    fn test_decode_0055_chk2_w_90_a0_d2_a6() {
        test_decoding_result_ok(
            &[0x02, 0xf0, 0xe8, 0x00, 0x20, 0x5a],
            Instruction {
                size: 2,
                operation: CHK2,
                operands: [ARDISP(A0, dr_disp(D2, 90)), AR(A6)],
            },
            &[" chk2.w 90(a0,d2),a6"],
        );
    }
    //  cmpi.b #$a5,90(a0,d2*4)
    #[test]
    fn test_decode_0056_cmpi_b_a5_90_a0_d2_4_() {
        test_decoding_result_ok(
            &[0x0c, 0x30, 0x00, 0xa5, 0x24, 0x5a],
            Instruction {
                size: 1,
                operation: CMPI,
                operands: [IMM8(0xa5), ARDISP(A0, dr_disp_scale(D2, 90, 2))],
            },
            &[" cmpi.b #$a5,90(a0,d2*4)"],
        );
    }
    //  cmpi.w #$a512,90(a0,d2*4)
    #[test]
    fn test_decode_0057_cmpi_w_a512_90_a0_d2_4_() {
        test_decoding_result_ok(
            &[0x0c, 0x70, 0xa5, 0x12, 0x24, 0x5a],
            Instruction {
                size: 2,
                operation: CMPI,
                operands: [IMM16(0xa512), ARDISP(A0, dr_disp_scale(D2, 90, 2))],
            },
            &[" cmpi.w #$a512,90(a0,d2*4)"],
        );
    }
    //  cmpi.l #$12345678,90(a0,d2*4)
    #[test]
    fn test_decode_0058_cmpi_l_12345678_90_a0_d2_4_() {
        test_decoding_result_ok(
            &[0x0c, 0xb0, 0x12, 0x34, 0x56, 0x78, 0x24, 0x5a],
            Instruction {
                size: 4,
                operation: CMPI,
                operands: [IMM32(0x12345678), ARDISP(A0, dr_disp_scale(D2, 90, 2))],
            },
            &[" cmpi.l #$12345678,90(a0,d2*4)"],
        );
    }
    //  btst #18,d0
    #[test]
    fn test_decode_0059_btst_18_d0() {
        test_decoding_result_ok(
            &[0x08, 0x00, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: BTST,
                operands: [IMM16(18), DR(D0)],
            },
            &[" btst #18,d0"],
        );
    }
    //  btst #18,(a0)+
    #[test]
    fn test_decode_0060_btst_18_a0_() {
        test_decoding_result_ok(
            &[0x08, 0x18, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: BTST,
                operands: [IMM16(18), ARINC(A0)],
            },
            &[" btst #18,(a0)+"],
        );
    }
    //  bclr #18,(a0)+
    #[test]
    fn test_decode_0061_bclr_18_a0_() {
        test_decoding_result_ok(
            &[0x08, 0x98, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: BCLR,
                operands: [IMM16(18), ARINC(A0)],
            },
            &[" bclr #18,(a0)+"],
        );
    }
    //  bchg #18,(a0)+
    #[test]
    fn test_decode_0062_bchg_18_a0_() {
        test_decoding_result_ok(
            &[0x08, 0x58, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: BCHG,
                operands: [IMM16(18), ARINC(A0)],
            },
            &[" bchg #18,(a0)+"],
        );
    }
    //  bset #18,(a0)+
    #[test]
    fn test_decode_0063_bset_18_a0_() {
        test_decoding_result_ok(
            &[0x08, 0xd8, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: BSET,
                operands: [IMM16(18), ARINC(A0)],
            },
            &[" bset #18,(a0)+"],
        );
    }
    //  moves.l a0,(a1)
    #[test]
    fn test_decode_0064_moves_l_a0_a1_() {
        test_decoding_result_ok(
            &[0x0e, 0x91, 0x88, 0x00],
            Instruction {
                size: 4,
                operation: MOVES,
                operands: [AR(A0), ARIND(A1)],
            },
            &[" moves.l a0,(a1)"],
        );
    }
    //  moves.b d0,(a1)
    #[test]
    fn test_decode_0065_moves_b_d0_a1_() {
        test_decoding_result_ok(
            &[0x0e, 0x11, 0x08, 0x00],
            Instruction {
                size: 1,
                operation: MOVES,
                operands: [DR(D0), ARIND(A1)],
            },
            &[" moves.b d0,(a1)"],
        );
    }
    //  cas d0,d1,(a0)
    #[test]
    fn test_decode_0066_cas_d0_d1_a0_() {
        test_decoding_result_err(
            &[0x0c, 0xd0, 0x00, 0x40],
            NotImplemented,
            &[" cas d0,d1,(a0)"],
        );
    }
    //  cas2 d0:d1,d2:d3,(a0):(a1)
    #[test]
    fn test_decode_0067_cas2_d0_d1_d2_d3_a0_a1_() {
        test_decoding_result_err(
            &[0x0c, 0xfc, 0x80, 0x80, 0x90, 0xc1],
            NotImplemented,
            &[" cas2 d0:d1,d2:d3,(a0):(a1)"],
        );
    }
}
