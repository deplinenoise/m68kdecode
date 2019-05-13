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

        cpu = '68040'
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
            elif l.startswith('! '):
                cpu = l[2:]
            else:
                stderr.write('bad input line: {}'.format(l))
        
        if len(asm_lines) > 0 and len(result_lines) > 0:
            with open('test.asm', 'w') as of:
                of.write('start:\n')
                for line in asm_lines:
                    of.write('\t')
                    of.write(line)
                    of.write('\n')
            
            if subprocess.call([vasm, '-m' + cpu, 'test.asm', '-quiet', '-Fbin', '-o', 'test.out']) != 0:
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
    of.write('#include "lib.h"\n')
    of.write('#include "tests/test_support.inl"\n')

    func_names = []

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
        func_name = 'test_decode_{:04d}_{}'.format(testnum, name)
        func_names.append(func_name)
        of.write('int {}()\n{{\n'.format(func_name))
        of.write('const uint8_t code_bytes[] = {\n')
        of.write(', '.join(['0x{0:02x}'.format(byt) for byt in code_bytes]))
        of.write('};\n')

        is_error_test = result_lines[0].find('{') == -1

        if not is_error_test:
            of.write('const m68k_instruction expected =')
        else:
            of.write('const m68k_decoding_error expected =')

        for l in result_lines:
            of.write(l)
            of.write('\n')

        of.write(';\n')


        if is_error_test:
            of.write('return test_decoding_result_err(')
        else:
            of.write('return test_decoding_result_ok(')
        of.write('"{}", code_bytes, {}, expected, "'.format(func_name, len(code_bytes)))
        for al in asm_lines:
            of.write(al.replace('\n', '\\n'))
        of.write('");}\n')

        testnum = testnum + 1

    of.write('int main(int argc, char* argv[])\n{\n')
    of.write('const int test_count = {};\n'.format(len(func_names)))
    of.write('int pass_count = 0;\n')
    for fn in func_names:
        of.write('  pass_count += {}();'.format(fn))

    of.write('printf("%5d / %5d test(s) passed\\n", pass_count, test_count);\n')

    of.write('return pass_count == test_count ? 0 : 1;\n');
    of.write('}\n')

os.remove('test.asm')
os.remove('test.out')

if subprocess.call(['astyle', '-q', '-n', '--style=kr', outfilename]) != 0:
    sys.exit(1)
