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
        self.wordindex = 0

    def make_longer(self):
        self.bit = self.bit - 1
        self.length = self.length + 1

    def __repr__(self):
        return "{}({}/{}:{})".format(self.name, self.wordindex, self.bit, self.length)

class Instruction(object):
    def __init__(self, name):
        object.__init__(self)
        self.name = name
        self.masks = []
        self.captures = []

    def add_mask(self, mask, captures):
        self.masks.append(mask)
        for c in captures:
            c.wordindex = len(self.masks) - 1
            self.captures.append(c)

    def __repr__(self):
        return "{}:\n".format(self.name) +  \
                "masks:\n    " + \
                ' '.join(["{:016b}".format(m) for m in self.masks]) + \
                "\ncaptures:\n    " + \
                ' '.join([str(c) for c in self.captures]);

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
            continue

        name, bits, result = m.groups()

        i = Instruction(name)

        bits = bits.strip()
        for mask in bits.replace('_', '').split():
            am, ac = analyze_mask(mask)
            i.add_mask(am, ac)

        print(i)
    
