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
outfile = sys.argv[2]

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
        return '(w{} & 0b{:016b}) == 0b{:016b}'.format(n, self.masks[n], self.instruction_patterns[n])

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
    
with open(outfile, "w") as of:
    of.write('use crate::*;\n')
    of.write('use codestream::*;\n')

    seen_insn_names = {}
    of.write('#[derive(Debug, PartialEq)]\npub enum Operation {\n');
    for i in instructions:
        if not seen_insn_names.has_key(i.name):
            seen_insn_names[i.name] = True
            of.write('  {},'.format(i.name))
    of.write('}\n');

    of.write('#[allow(non_snake_case)]\n')
    of.write('pub fn decode_instruction(code: &[u8]) -> Result<DecodedInstruction, DecodingError> {\n')
    of.write('  let mut cs = CodeStream::new(code);\n')
    of.write('  let w0 = cs.pull16();\n')
    of.write('  let sz;\n')
    of.write('  let src;\n')
    of.write('  let dst;\n')
    of.write('  let mut extra = NoExtra;\n')
    for i in instructions:
        of.write('if (w0 & 0b{:016b}) == 0b{:016b} '.format(i.masks[0], i.instruction_patterns[0]))
        if len(i.masks) > 1:
            of.write('&& cs.has_words({})'.format(len(i.masks) - 1))
        of.write('{\n')
        #of.write('println!("w0 match {}");\n'.format(i.name))
        for n in range(1, len(i.masks)):
            of.write('let w{} = cs.peek_word({});\n'.format(n, n-1))

        if len(i.masks) > 1:
            of.write('if {} {{\n'.format(' && '.join([i.match_expr(n) for n in range(1, len(i.masks))])))

        for c in i.captures:
            if c.name != '?':
                of.write('let {} = get_bits(w{}, {}, {});\n'.format(c.name, c.wordindex, c.bit, c.length))
                #of.write('println!("{} = {{}}", {});\n'.format(c.name, c.name))

        predicate_nesting = 0

        for predicate in re.findall(R_PREDICATE, i.result):
            predicate_nesting = predicate_nesting + 1
            of.write('if {} {{\n'.format(predicate[2:-1]))

        if len(i.masks) > 1:
            of.write('cs.skip_words({});\n'.format(len(i.masks)-1))

        expr = re.sub(R_PREDICATE, "", i.result)

        of.write(expr + '\n')

        if expr.find('return') == -1:
            of.write('return cs.check_overflow(Instruction {{ size: sz, operation: {}, operands: [ src, dst ], extra: extra }});\n'.format(i.name))

        for x in range(0, predicate_nesting):
            of.write('}\n')

        if len(i.masks) > 1:
            of.write('}\n')

        of.write('}\n')
    of.write('  return Err(NotImplemented);\n')
    of.write('}\n')

subprocess.call(['rustfmt', outfile])
