use crate::*;

pub fn get_bits(word: u16, first: i32, length: i32) -> u16 {
    let s = word >> first;
    let mask = (1 << length) - 1;
    s & mask
}

pub struct CodeStream<'a> {
    bytes: &'a [u8],
    pos: usize,
    error: Option<DecodingError>,
}

impl<'a> CodeStream<'a> {

    pub fn new(b: &'a [u8]) -> CodeStream<'a> {
        CodeStream {
            bytes: b,
            pos: 0,
            error: None,
        }
    }

    pub fn check_overflow(&self, i: Instruction) -> Result<DecodedInstruction, DecodingError> {
        match self.error {
            None => Ok(DecodedInstruction { bytes_used: self.pos as u32, instruction: i }),
            Some(e) => Err(e),
        }
    }

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

    pub fn data_reg_op(&mut self, r: u16) -> Operand {
        DR(self.data_reg(r))
    }

    pub fn address_reg_op(&mut self, r: u16) -> Operand {
        AR(self.address_reg(r))
    }

    pub fn ea(&mut self, src_reg: u16, src_mod: u16, size: i32) -> Operand {
        match src_mod {
            0b000 => DR(self.data_reg(src_reg)),
            0b001 => AR(self.address_reg(src_reg)),
            0b010 => ARIND(self.address_reg(src_reg)),
            0b011 => ARINC(self.address_reg(src_reg)),
            0b100 => ARDEC(self.address_reg(src_reg)),
            0b101 => ARDISP(self.address_reg(src_reg), simple_disp(self.pull16() as i16 as i32)),
            0b110 => {
                let r = Some(self.address_reg(src_reg));
                self.decode_extended_ea(r)
            },
            0b111 => match src_reg {
                0b000 => ABS16(self.pull16() as i16),
                0b001 => ABS32(self.pull32()),
                0b010 => PCDISP(simple_disp(self.pull16() as i16 as i32)),
                0b011 => self.decode_extended_ea(None),
                0b100 => match size {
                    1 => IMM8(self.pull16() as u8),
                    2 => IMM16(self.pull16()),
                    4 => IMM32(self.pull32()),
                    _ => { self.error = Some(BadSize); NoOperand },
                },
                _ => { self.error = Some(NotImplemented); NoOperand },
            },
            _ => { self.error = Some(NotImplemented); NoOperand },
        }
    }

    fn decode_extended_ea(&mut self, src_reg: Option<AddressRegister>) -> Operand {
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
                    None => PCDISP(Displacement {
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
                None => PCDISP(displacement),
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

    pub fn dar(&mut self, d_or_a: u16, regno: u16) -> Operand {
        match d_or_a {
            0 => DR(self.data_reg(regno)),
            _ => AR(self.address_reg(regno))
        }
    }
}
