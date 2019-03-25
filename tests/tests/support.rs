// Test support functions
pub use m68kdecode::*;

pub fn test_decoding_result_ok(bytes: &[u8], expected: Instruction, asm: &[&str]) {
    let r = decode_instruction(&bytes);

    match r {
        Err(e) => {
            println!("Expected: {:?}", expected);
            println!("Got: {:?}", e);
            for l in asm {
                println!("{}", l);
            }
            assert!(false);
        }
        Ok(DecodedInstruction {
            bytes_used,
            instruction,
        }) => {
            assert!(bytes_used == bytes.len() as u32);
            if instruction != expected {
                println!("Expected: {:?}", expected);
                println!("Got: {:?}", instruction);
                for l in asm {
                    println!("{}", l);
                }
                assert!(false);
            }
        }
    }
}

pub fn test_decoding_result_err(bytes: &[u8], expected: DecodingError, asm: &[&str]) {
    let r = decode_instruction(&bytes);
    match r {
        Err(e) => {
            if e == expected {
                return;
            }
        }
        _ => (),
    };

    println!("Expected: {:?}", expected);
    println!("Got: {:?}", r);
    for l in asm {
        println!("{}", l);
    }
    assert!(false);
}
