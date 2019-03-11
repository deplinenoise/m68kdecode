#! /usr/bin/env python

import re
import sys

R_BLANK = re.compile('^\s*$')
R_COMMENT = re.compile('^#.*')
R_LINEFMT = re.compile('^([A-Z][A-Z0-9]+)\s+((?:(?:(?:[01A-Za-z]{4}_){3}[01A-Za-z]{4})\s+)+)(.*?)$')

lineno = 0
infile = sys.argv[1]

class Capture(object):
    def __init__(self, name, bit, length):
        object.__init__(self)
        self.name = name
        self.bit = bit
        self.length = length

    def make_longer(self):
        self.length = self.length + 1

    def __repr__(self):
        return "{}({}:{})".format(self.name, self.bit, self.length)

def analyze_mask(mask):
    assert len(mask) == 16

    out_mask = 0

    prev_capture = None
    bit = 16
    captures = []

    for ch in mask:
        bit = bit - 1
        if ch == '0':
            prev_capture = None
        elif ch == '1':
            out_mask = out_mask | (1 << bit)
            prev_capture = None
        elif ch == prev_capture:
            captures[-1].make_longer()
        else:
            prev_capture = ch
            captures.append(Capture(ch, bit, 1))
            
    return (out_mask, captures)


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
            continue
        insn, bits, result = m.groups()
        #print(insn, bits, result)
        bits = bits.strip()
        masks = bits.replace('_', '').split()

        am, ac = analyze_mask(masks[0])

        print("{:016b} {}".format(am, ac))
    
