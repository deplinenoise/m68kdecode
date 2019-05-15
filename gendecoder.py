#! /usr/bin/env python

import re
import sys
import subprocess

R_BLANK = re.compile('^\s*$')
R_COMMENT = re.compile('^#.*')
R_LINEFMT = re.compile('^([A-Z][A-Z0-9?]+)\s+((?:(?:(?:[01A-Za-z?]{4}_){3}[01A-Za-z?]{4})\s+)+)(.*?)$')
R_PREDICATE = re.compile('\\?\\(.*?\\)')

lineno = 0
infile = sys.argv[1]
outfile_c = sys.argv[2]
outfile_h = sys.argv[3]

class Capture(object):
    def __init__(self, name, bit, length):
        object.__init__(self)
        self.name = name
        self.bit = bit
        self.length = length
        self.wordindex = 0

    def make_longer(self):
        self.bit = self.bit - 1
        self.length = self.length + 1

    def __repr__(self):
        return "{}({}/{}:{})".format(self.name, self.wordindex, self.bit, self.length)

class Instruction(object):
    def __init__(self, name, result):
        object.__init__(self)
        self.name = name
        self.masks = []
        self.instruction_patterns = []
        self.captures = []
        self.result = result

    def add_mask(self, mask, instruction_pattern, captures):
        self.masks.append(mask)
        self.instruction_patterns.append(instruction_pattern)
        for c in captures:
            c.wordindex = len(self.masks) - 1
            self.captures.append(c)

    def match_expr(self, n):
        return '(w{} & 0x{:04x}) == 0x{:04x}'.format(n, self.masks[n], self.instruction_patterns[n])

    def __repr__(self):
        return "{}:\n".format(self.name) +  \
                "masks:\n    " + \
                ' '.join(["{:016b}".format(m) for m in self.masks]) + \
                "patterns:\n    " + \
                ' '.join(["{:016b}".format(p) for p in self.instruction_patterns]) + \
                "\ncaptures:\n    " + \
                ' '.join([str(c) for c in self.captures]);

def analyze_mask(mask):
    assert len(mask) == 16

    out_mask = 0
    out_insn = 0

    prev_capture = None
    bit = 16
    captures = []

    for ch in mask:
        bit = bit - 1
        if ch == '0':
            out_mask = out_mask | (1 << bit)
            prev_capture = None
        elif ch == '1':
            out_insn = out_insn | (1 << bit)
            out_mask = out_mask | (1 << bit)
            prev_capture = None
        elif ch == prev_capture:
            captures[-1].make_longer()
        else:
            prev_capture = ch
            captures.append(Capture(ch, bit, 1))
            
    return (out_mask, out_insn, captures)

instructions = []

with open(infile, "r") as inf:
    for line in inf:

        lineno = lineno + 1

        if R_COMMENT.match(line):
            continue
        elif R_BLANK.match(line):
            continue
        m = R_LINEFMT.match(line)
        if not m:
            sys.stderr.write('{}({}): bad line\n'.format(infile, lineno))
            sys.stderr.write('{}({}): line "{}"\n'.format(infile, lineno, line))
            sys.exit(1)

        name, bits, result = m.groups()

        i = Instruction(name, result)

        bits = bits.strip()
        for mask in bits.replace('_', '').split():
            am, ai, ac = analyze_mask(mask)
            i.add_mask(am, ai, ac)

        instructions.append(i)
        #print(i)

def split_by_length(insns):
    groups = [ ]
    prev_length = -1
    for i in insns:
        if len(i.masks) != prev_length:
            groups.append([])
            prev_length = len(i.masks)
        groups[-1].append(i)
    return groups

def gen_decoders(of, insns):
    group = insns[0].instruction_patterns[0] >> 12

    of.write('static m68k_decoding_error decode_group_{0:04b}(uint16_t w0, m68k_code_stream *cs, m68k_decoded_instruction *result) {{\n'.format(group))

    of.write('int sz = 0;\n')
    of.write('m68k_operand src = { .kind = M68K_NO_OPERAND };\n')
    of.write('m68k_operand dst = { .kind = M68K_NO_OPERAND };\n')
    of.write('m68k_extra extra = { .kind = M68K_NO_EXTRA };\n')

    groups = split_by_length(insns)

    for lgroup in groups:
        
        mlen = len(lgroup[0].masks) - 1

        if mlen > 0:
            of.write('if (has_words(cs, {})) {{\n'.format(mlen))

            for n in range(1, mlen+1):
                of.write('uint16_t w{} = peek_word(cs, {});\n'.format(n, n-1))

        for i in lgroup:
            of.write('if ((w0 & 0x{:04x}) == 0x{:04x}) {{'.format(i.masks[0], i.instruction_patterns[0]))

            if mlen > 0:
                of.write('if ({}) {{\n'.format(' && '.join([i.match_expr(n) for n in range(1, len(i.masks))])))

            for c in i.captures:
                if c.name != '?':
                    of.write('uint16_t {} = get_bits(w{}, {}, {});\n'.format(c.name, c.wordindex, c.bit, c.length))
                    #of.write('println!("{} = {{}}", {});\n'.format(c.name, c.name))

            predicate_nesting = 0

            for predicate in re.findall(R_PREDICATE, i.result):
                predicate_nesting = predicate_nesting + 1
                of.write('if ({}) {{\n'.format(predicate[2:-1]))

            if len(i.masks) > 1:
                of.write('skip_words(cs, {});\n'.format(len(i.masks)-1))

            expr = re.sub(R_PREDICATE, "", i.result)

            have_extra = False

            for sub_expr in expr.split(';'):
                sub_expr = sub_expr.strip();
                if len(sub_expr) == 0:
                    continue
                if sub_expr.find('extra') != -1:
                    have_extra = True
                of.write(sub_expr)
                of.write(';\n')

            if expr.find('return') == -1:
                of.write('return check_overflow(cs, (m68k_instruction) {{ .size = sz, .operation = M68K_{}, .operands = {{ src, dst }}, .extra = extra }}, result);\n'.format(i.name))

            for x in range(0, predicate_nesting):
                of.write('}\n')

            if len(i.masks) > 1:
                of.write('}\n')

            of.write('}\n')

        if mlen > 0:
            of.write('}\n')

    of.write('  return M68K_NOT_IMPLEMENTED;\n')
    of.write('}\n')

with open(outfile_h, "w") as of:
    seen_insn_names = {}
    of.write('#pragma once\n')
    of.write('/// Instruction names.\n');
    of.write('typedef enum m68k_operation {\n');
    for i in instructions:
        if not seen_insn_names.has_key(i.name):
            seen_insn_names[i.name] = True
            of.write('  M68K_{},\n'.format(i.name))
    of.write('} m68k_operation;\n');
    of.write('/// Instruction name strings.\n');
    of.write('extern const char *m68k_operation_names[];\n');
    
with open(outfile_c, "w") as of:
    of.write('#include "{}"\n'.format(outfile_h));
    of.write('#include "decoder_support.inl"\n')

    has_group = {}

    of.write('const char *m68k_operation_names[] = {\n');
    seen_insn_names = {}
    for i in instructions:
        if not seen_insn_names.has_key(i.name):
            seen_insn_names[i.name] = True
            of.write('  "{}",\n'.format(i.name))
    of.write('};\n');

    for group in range(0, 16):
        insns = [i for i in instructions if (i.instruction_patterns[0] >> 12) == group]
        if len(insns) == 0:
            continue
        has_group[group] = True
        insns.sort(key = lambda x: len(x.masks), reverse = True)
        gen_decoders(of, insns)

    of.write('m68k_decoding_error m68k_decode(const uint8_t *code, uint32_t len, m68k_decoded_instruction *result)\n{\n')
    of.write('  m68k_code_stream cs;\n')
    of.write('  m68k_code_stream_init(&cs, code, len);\n')
    of.write('  uint16_t w0 = pull16(&cs);\n')
    of.write('  switch(w0 >> 12) {\n')
    for group in range(0, 16):
        if not has_group.has_key(group):
            continue
        of.write('    case 0x{0:02x}: return decode_group_{0:04b}(w0, &cs, result);\n'.format(group))
    of.write('    default: return M68K_NOT_IMPLEMENTED;\n')
    of.write('  }\n')

    of.write('}\n')

for fn in (outfile_c, outfile_h):
    if subprocess.call(['astyle', '-q', '-n', '--style=kr', fn]) != 0:
        sys.exit(1)
