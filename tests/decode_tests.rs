// auto-generated from decoding_tests.txt by gen_decoding_tests.py
mod tests {
    use m68kdecode::*;
    fn do_test(bytes: &[u8], expected: Instruction) {
        let r = decode_instruction(&bytes).unwrap();
        assert!(r.bytes_used == bytes.len() as u32);
        if r.instruction != expected {
            println!("Expected: {:?}", expected);
            println!("Got: {:?}", r.instruction);
            assert!(false);
        }
    }
    //  move.b d0,d1
    #[test]
    fn test_decode_1() {
        do_test(
            &[0x12, 0x00],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D0), DR(D1)],
            },
        );
    }
    //  move.b d2,d3
    #[test]
    fn test_decode_2() {
        do_test(
            &[0x16, 0x02],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D2), DR(D3)],
            },
        );
    }
    //  move.b d4,d5
    #[test]
    fn test_decode_3() {
        do_test(
            &[0x1a, 0x04],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D4), DR(D5)],
            },
        );
    }
    //  move.b d6,d7
    #[test]
    fn test_decode_4() {
        do_test(
            &[0x1e, 0x06],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [DR(D6), DR(D7)],
            },
        );
    }
    //  move.w a0,a1
    #[test]
    fn test_decode_5() {
        do_test(
            &[0x32, 0x48],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A0), AR(A1)],
            },
        );
    }
    //  move.w a2,a3
    #[test]
    fn test_decode_6() {
        do_test(
            &[0x36, 0x4a],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A2), AR(A3)],
            },
        );
    }
    //  move.w a4,a5
    #[test]
    fn test_decode_7() {
        do_test(
            &[0x3a, 0x4c],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A4), AR(A5)],
            },
        );
    }
    //  move.w a6,a7
    #[test]
    fn test_decode_8() {
        do_test(
            &[0x3e, 0x4e],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [AR(A6), AR(A7)],
            },
        );
    }
    //  move.b 123(a0,d0),d3
    #[test]
    fn test_decode_9() {
        do_test(
            &[0x16, 0x30, 0x00, 0x7b],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [ARDISP(A0, dr_disp(D0, 123)), DR(D3)],
            },
        );
    }
    //  move.w 123(a0,d0),d3
    #[test]
    fn test_decode_10() {
        do_test(
            &[0x36, 0x30, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [ARDISP(A0, dr_disp(D0, 123)), DR(D3)],
            },
        );
    }
    //  move.l 123(a0,d0),d3
    #[test]
    fn test_decode_11() {
        do_test(
            &[0x26, 0x30, 0x00, 0x7b],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [ARDISP(A0, dr_disp(D0, 123)), DR(D3)],
            },
        );
    }
    //  move.l 123(a0,d0),a1
    #[test]
    fn test_decode_12() {
        do_test(
            &[0x22, 0x70, 0x00, 0x7b],
            Instruction {
                size: 4,
                operation: MOVEA,
                operands: [ARDISP(A0, dr_disp(D0, 123)), AR(A1)],
            },
        );
    }
    //  move.w 123(a0,d0),a1
    #[test]
    fn test_decode_13() {
        do_test(
            &[0x32, 0x70, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: MOVEA,
                operands: [ARDISP(A0, dr_disp(D0, 123)), AR(A1)],
            },
        );
    }
    //  move.b #$12,d7
    #[test]
    fn test_decode_14() {
        do_test(
            &[0x1e, 0x3c, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: MOVE,
                operands: [IMM8(0x12), DR(D7)],
            },
        );
    }
    //  move.w #$1234,d7
    #[test]
    fn test_decode_15() {
        do_test(
            &[0x3e, 0x3c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [IMM16(0x1234), DR(D7)],
            },
        );
    }
    //  move.l #$12345678,d7
    #[test]
    fn test_decode_16() {
        do_test(
            &[0x2e, 0x3c, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [IMM32(0x12345678), DR(D7)],
            },
        );
    }
    //  move.l D1,-(A2)
    #[test]
    fn test_decode_17() {
        do_test(
            &[0x25, 0x01],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [DR(D1), ARDEC(A2)],
            },
        );
    }
    //  move.l D1,(A2)+
    #[test]
    fn test_decode_18() {
        do_test(
            &[0x24, 0xc1],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [DR(D1), ARINC(A2)],
            },
        );
    }
    //  move.l -(A4),(A2)+
    #[test]
    fn test_decode_19() {
        do_test(
            &[0x24, 0xe4],
            Instruction {
                size: 4,
                operation: MOVE,
                operands: [ARDEC(A4), ARINC(A2)],
            },
        );
    }
    //  move.l 4.w,A0
    #[test]
    fn test_decode_20() {
        do_test(
            &[0x20, 0x78, 0x00, 0x04],
            Instruction {
                size: 4,
                operation: MOVEA,
                operands: [ABS16(4), AR(A0)],
            },
        );
    }
    //  move.l $11223344,A0
    #[test]
    fn test_decode_21() {
        do_test(
            &[0x20, 0x79, 0x11, 0x22, 0x33, 0x44],
            Instruction {
                size: 4,
                operation: MOVEA,
                operands: [ABS32(0x11223344), AR(A0)],
            },
        );
    }
    //  move.w #$1234,123(d0)
    #[test]
    fn test_decode_22() {
        do_test(
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
        );
    }
    //  move.w -8(pc),d3
    #[test]
    fn test_decode_23() {
        do_test(
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
        );
    }
    //  move.w -8(pc,d2*8),d3
    #[test]
    fn test_decode_24() {
        do_test(
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
        );
    }
    //  move.w 123(a1,d2*4),9876(a2,d3*2)
    #[test]
    fn test_decode_25() {
        do_test(
            &[0x35, 0xb1, 0x24, 0x7b, 0x33, 0x20, 0x26, 0x94],
            Instruction {
                size: 2,
                operation: MOVE,
                operands: [
                    ARDISP(A1, dr_disp_scale(D2, 123, 2)),
                    ARDISP(A2, dr_disp_scale(D3, 9876, 1)),
                ],
            },
        );
    }
    //  move.w d0,12345(a0,a1*2)
    #[test]
    fn test_decode_26() {
        do_test(
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
        );
    }
    //  lea (a0),a1
    #[test]
    fn test_decode_27() {
        do_test(
            &[0x43, 0xd0],
            Instruction {
                size: 4,
                operation: LEA,
                operands: [ARIND(A0), AR(A1)],
            },
        );
    }
    //  lea 8(a0),a1
    #[test]
    fn test_decode_28() {
        do_test(
            &[0x43, 0xe8, 0x00, 0x08],
            Instruction {
                size: 4,
                operation: LEA,
                operands: [ARDISP(A0, simple_disp(8)), AR(A1)],
            },
        );
    }
    //  ori #17,ccr
    #[test]
    fn test_decode_29() {
        do_test(
            &[0x00, 0x3c, 0x00, 0x11],
            Instruction {
                size: 1,
                operation: ORITOCCR,
                operands: [IMM8(17), Implied],
            },
        );
    }
    //  ori #$1234,sr
    #[test]
    fn test_decode_30() {
        do_test(
            &[0x00, 0x7c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ORITOSR,
                operands: [IMM16(0x1234), Implied],
            },
        );
    }
    //  ori.w #$1234,d0
    #[test]
    fn test_decode_31() {
        do_test(
            &[0x00, 0x40, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ORI,
                operands: [IMM16(0x1234), DR(D0)],
            },
        );
    }
    //  ori.b #$12,d2
    #[test]
    fn test_decode_32() {
        do_test(
            &[0x00, 0x02, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: ORI,
                operands: [IMM8(0x12), DR(D2)],
            },
        );
    }
    //  ori.w #$1234,123(a0,d0)
    #[test]
    fn test_decode_33() {
        do_test(
            &[0x00, 0x70, 0x12, 0x34, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: ORI,
                operands: [IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123))],
            },
        );
    }
    //  ori.l #$12345678,-(a0)
    #[test]
    fn test_decode_34() {
        do_test(
            &[0x00, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: ORI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
        );
    }
    //  andi #17,ccr
    #[test]
    fn test_decode_35() {
        do_test(
            &[0x02, 0x3c, 0x00, 0x11],
            Instruction {
                size: 1,
                operation: ANDITOCCR,
                operands: [IMM8(17), Implied],
            },
        );
    }
    //  andi #$1234,sr
    #[test]
    fn test_decode_36() {
        do_test(
            &[0x02, 0x7c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ANDITOSR,
                operands: [IMM16(0x1234), Implied],
            },
        );
    }
    //  andi.w #$1234,d0
    #[test]
    fn test_decode_37() {
        do_test(
            &[0x02, 0x40, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: ANDI,
                operands: [IMM16(0x1234), DR(D0)],
            },
        );
    }
    //  andi.b #$12,d2
    #[test]
    fn test_decode_38() {
        do_test(
            &[0x02, 0x02, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: ANDI,
                operands: [IMM8(0x12), DR(D2)],
            },
        );
    }
    //  andi.w #$1234,123(a0,d0)
    #[test]
    fn test_decode_39() {
        do_test(
            &[0x02, 0x70, 0x12, 0x34, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: ANDI,
                operands: [IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123))],
            },
        );
    }
    //  andi.l #$12345678,-(a0)
    #[test]
    fn test_decode_40() {
        do_test(
            &[0x02, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: ANDI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
        );
    }
    //  eori #17,ccr
    #[test]
    fn test_decode_41() {
        do_test(
            &[0x0a, 0x3c, 0x00, 0x11],
            Instruction {
                size: 1,
                operation: EORITOCCR,
                operands: [IMM8(17), Implied],
            },
        );
    }
    //  eori #$1234,sr
    #[test]
    fn test_decode_42() {
        do_test(
            &[0x0a, 0x7c, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: EORITOSR,
                operands: [IMM16(0x1234), Implied],
            },
        );
    }
    //  eori.w #$1234,d0
    #[test]
    fn test_decode_43() {
        do_test(
            &[0x0a, 0x40, 0x12, 0x34],
            Instruction {
                size: 2,
                operation: EORI,
                operands: [IMM16(0x1234), DR(D0)],
            },
        );
    }
    //  eori.b #$12,d2
    #[test]
    fn test_decode_44() {
        do_test(
            &[0x0a, 0x02, 0x00, 0x12],
            Instruction {
                size: 1,
                operation: EORI,
                operands: [IMM8(0x12), DR(D2)],
            },
        );
    }
    //  eori.w #$1234,123(a0,d0)
    #[test]
    fn test_decode_45() {
        do_test(
            &[0x0a, 0x70, 0x12, 0x34, 0x00, 0x7b],
            Instruction {
                size: 2,
                operation: EORI,
                operands: [IMM16(0x1234), ARDISP(A0, dr_disp(D0, 123))],
            },
        );
    }
    //  eori.l #$12345678,-(a0)
    #[test]
    fn test_decode_46() {
        do_test(
            &[0x0a, 0xa0, 0x12, 0x34, 0x56, 0x78],
            Instruction {
                size: 4,
                operation: EORI,
                operands: [IMM32(0x12345678), ARDEC(A0)],
            },
        );
    }
}
