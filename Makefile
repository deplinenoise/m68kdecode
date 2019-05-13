
VASM = ../newage_a500/t2-output/hostbuild/vasm

.PHONY: all

all: src/decoder.c src/decoder.h tests/decode_tests.c

clean:
	rm -f src/decoder.c src/decoder.h tests/decode_tests.c

src/decoder.c src/decoder.h: imask.txt gendecoder.py
	python gendecoder.py imask.txt src/decoder.c src/decoder.h

tests/decode_tests.c: tests/decode_tests.txt gen_decoding_tests.py
	python gen_decoding_tests.py $(VASM) tests/decode_tests.txt tests/decode_tests.c

