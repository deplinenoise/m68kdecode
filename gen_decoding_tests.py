#! /usr/bin/env python

import sys
import os
import re
import subprocess

R_NSYMCHAR = re.compile('[^a-zA-Z0-9_]+')
R_UNDERSCORES = re.compile('[_]+')
R_LEADING_UNDERSCORE = re.compile('^_')

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
            if len(l) == 0 or l.startswith('//'):
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
                sys.exit(1)
            else:
                with open('test.out', 'rb') as bf:
                    code_bytes = bytearray(bf.read())
                    tests.append((code_bytes, result_lines, asm_lines))


    
with open(outfilename, "w") as of:
    of.write('// auto-generated from decoding_tests.txt by gen_decoding_tests.py\n')
    of.write('mod tests {\n')
    of.write('  use m68kdecode::*;\n')

    testnum = 1
    for t in tests:
        code_bytes = t[0]
        result_lines = t[1]
        asm_lines = t[2]
        for al in asm_lines:
            of.write('// {}\n'.format(al))
        name = re.sub(R_NSYMCHAR, '_', asm_lines[0].strip()).lower()
        name = re.sub(R_UNDERSCORES, '_', name)
        name = re.sub(R_LEADING_UNDERSCORE, '', name)
        of.write('#[test]\n')
        of.write('fn test_decode_{:04d}_{}() {{\n'.format(testnum, name))
        if result_lines[0].find('Instruction {') == -1:
            of.write('test_decoding_result_err(&[')
        else:
            of.write('test_decoding_result_ok(&[')
        of.write(', '.join(['0x{0:02x}'.format(byt) for byt in code_bytes]))
        of.write('], ')
        for l in result_lines:
            of.write(l)
            of.write('\n')
        of.write(', &[')
        for al in asm_lines:
            of.write('"{}",\n'.format(al))
        of.write(']')
        of.write(');}\n')

        testnum = testnum + 1
    of.write('}\n')

if subprocess.call(['rustfmt', outfilename]) != 0:
    sys.exit(1)

os.remove('test.asm')
os.remove('test.out')
