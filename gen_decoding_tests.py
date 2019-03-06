#! /usr/bin/env python

import sys
import os
import subprocess

vasm = sys.argv[1]
infilename = sys.argv[2]
outfilename = sys.argv[3]

tests = []

with open(infilename, 'r') as f:
    eof = False
    while not eof:
        asm_lines = []
        result_lines = []

        while True:
            l = f.readline()
            if len(l) == 0:
                eof = True
                break
            l = l.strip()
            if len(l) == 0:
                break
            if l.startswith('> '):
                asm_lines.append(l[1:])
            elif l.startswith('# '):
                result_lines.append(l[1:])
            else:
                stderr.write('bad input line: {}'.format(l))
        
        if len(asm_lines) > 0 and len(result_lines) > 0:
            with open('test.asm', 'w') as of:
                of.write('start:\n')
                for line in asm_lines:
                    of.write('\t')
                    of.write(line)
                    of.write('\n')
            
            if subprocess.call([vasm, '-m68020', 'test.asm', '-quiet', '-Fbin', '-o', 'test.out']) != 0:
                sys.stderr.write('vasm failed for:')
                for l in asm_lines:
                    sys.stderr.write(l)
            else:
                with open('test.out', 'rb') as bf:
                    code_bytes = bytearray(bf.read())
                    tests.append((code_bytes, result_lines, asm_lines))
    
with open(outfilename, "w") as of:
    of.write('// auto-generated from decoding_tests.txt by gen_decoding_tests.py\n')
    of.write('mod tests {\n')
    of.write('  use m68kdecode::*;\n')
    of.write(' fn do_test(bytes: &[u8], expected: Instruction) {\n')
    of.write('    let r = decode_instruction(&bytes).unwrap();\n')
    of.write('    assert!(r.bytes_used == bytes.len() as u32);\n')
    of.write('    if r.instruction != expected {\n')
    of.write('        println!("Expected: {:?}", expected);\n')
    of.write('        println!("Got: {:?}", r.instruction);\n')
    of.write('        assert!(false);\n')
    of.write('    }\n')
    of.write('}\n')

    testnum = 1
    for t in tests:
        code_bytes = t[0]
        result_lines = t[1]
        asm_lines = t[2]
        for al in asm_lines:
            of.write('// {}\n'.format(al))
        of.write('#[test]\n')
        of.write('fn test_decode_{}() {{\n'.format(testnum))
        of.write('do_test(&[')
        of.write(', '.join(['0x{0:02x}'.format(byt) for byt in code_bytes]))
        of.write('], ')
        for l in result_lines:
            of.write(l)
            of.write('\n')
        of.write(');}\n')

        testnum = testnum + 1
    of.write('}\n')

subprocess.call(['rustfmt', outfilename])

os.remove('test.asm')
os.remove('test.out')
