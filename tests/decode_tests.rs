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

    //#[test]
    fn test_lea_l() {
        do_test(&[0x43, 0xf0, 0x00, 0x7b], Instruction {
            size: 4,
            operation: LEA,
            operands: [
                ARDISP(A0, dr_disp(D0, 123)),
                AR(A1),
            ],
        });
    }

    #[test]
    fn test_move() {
        do_test(&[0x26, 0x30, 0x00, 0x7b], Instruction {
            size: 4,
            operation: MOVE,
            operands: [
                ARDISP(A0, dr_disp(D0, 123)),
                DR(D3),
            ],
        });
    }

    #[test]
    fn test_movea_scale() {
        do_test(&[0x22, 0x70, 0x02, 0x7b], Instruction {
            size: 4,
            operation: MOVEA,
            operands: [
                ARDISP(A0, dr_disp_scale(D0, 123, 1)),
                AR(A1),
            ],
        });
    }

    #[test]
    fn test_move_b_imm_dr() {
        do_test(&[0x1e, 0x3c, 0x00, 0xff], Instruction {
            size: 1, operation: MOVE, operands: [ IMM8(0xff), DR(D7), ],
        });
    }

    #[test]
    fn test_move_w_imm_dr() {
        do_test(&[0x3a, 0x3c, 0x12, 0x34], Instruction {
            size: 2, operation: MOVE, operands: [ IMM16(0x1234), DR(D5), ],
        });
    }

    #[test]
    fn test_move_l_imm_dr() {
        do_test(&[0x28, 0x3c, 0x12, 0x34, 0x56, 0x78], Instruction {
            size: 4, operation: MOVE, operands: [ IMM32(0x12345678), DR(D4), ],
        });
    }

    #[test]
    fn test_move_l_ar_predec() {
        do_test(&[0x25, 0x01], Instruction {
            size: 4, operation: MOVE, operands: [ DR(D1), ARDEC(A2), ],
        });
    }

    #[test]
    fn test_move_l_ar_postinc() {
        do_test(&[0x24, 0xc1], Instruction {
            size: 4, operation: MOVE, operands: [ DR(D1), ARINC(A2), ],
        });
    }

    #[test]
    fn test_move_l_pre_post() {
        do_test(&[0x24, 0xe4], Instruction {
            size: 4, operation: MOVE, operands: [ 
                ARDEC(A4),
                ARINC(A2),
            ],
        });
    }

    #[test]
    fn test_move_l_abs16() {
        do_test(&[0x20, 0x78, 0x00, 0x04], Instruction {
            size: 4, operation: MOVEA, operands: [ 
                ABS16(4),
                AR(A0),
            ],
        });
    }

    #[test]
    fn test_move_l_abs32() {
        do_test(&[0x20, 0x79, 0x11, 0x22, 0x33, 0x44], Instruction {
            size: 4, operation: MOVEA, operands: [ 
                ABS32(0x11223344),
                AR(A0),
            ],
        });
    }

    // 31 bc 12 34 01 a0 00 7b
    #[test]
    fn test_move_imm_020_dx_base() {
        do_test(&[0x31, 0xbc, 0x12, 0x34, 0x01, 0xa0, 0x00, 0x7b], Instruction {
            size: 2, operation: MOVE, operands: [ 
                IMM16(0x1234),
                DISP(Displacement {
                    base_displacement: 123,
                    outer_displacement: 0,
                    indexer: Indexer::DR(D0, 0),
                    indirection: NoIndirection,
                }),
            ],
        });
    }

    #[test]
    fn test_move_neg_pc_dx() {
        do_test(&[0x36, 0x3a, 0xff, 0xf8], Instruction {
            size: 2, operation: MOVE, operands: [ 
                PCDISP(Displacement {
                    base_displacement: -8,
                    outer_displacement: 0,
                    indexer: Indexer::NoIndexer,
                    indirection: NoIndirection,
                }),
                DR(D3),
            ],
        });
    }

    // 3e 3b 26 f8

    #[test]
    fn test_move_neg_pc_dx_scale() {
        do_test(&[0x3e, 0x3b, 0x26, 0xf8], Instruction {
            size: 2, operation: MOVE, operands: [ 
                PCDISP(Displacement {
                    base_displacement: -8,
                    outer_displacement: 0,
                    indexer: Indexer::DR(D2, 3),
                    indirection: NoIndirection,
                }),
                DR(D7),
            ],
        });
    }
    #[test]
    fn test_move_w_memi_memi() {
        do_test(&[0x35, 0xb1, 0x24, 0x7b, 0x33, 0x20, 0x26, 0x94], Instruction {
            size: 2,
            operation: MOVE,
            operands: [
                ARDISP(A1, dr_disp_scale(D2, 123, 2)),
                ARDISP(A2, dr_disp_scale(D3, 9876, 1)),
            ],
        });
    }
}

