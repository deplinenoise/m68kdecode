
VASM = ../newage_a500/t2-output/hostbuild/vasm

.PHONY: all

all: src/decoder.rs tests/decode_tests.rs
	cargo test

clean:
	rm -f src/decoder.rs tests/decode_tests.rs

src/decoder.rs: imask.txt gendecoder.py
	python gendecoder.py imask.txt $@

tests/decode_tests.rs: tests/decode_tests.txt gen_decoding_tests.py
	python gen_decoding_tests.py $(VASM) tests/decode_tests.txt tests/decode_tests.rs

