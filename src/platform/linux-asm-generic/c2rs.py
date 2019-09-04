#!/usr/bin/env python3

import os
import re
import sys


def main():
    if len(sys.argv) != 2:
        print("Usage: %s input-file" % sys.argv[0])
        sys.exit(1)

    macro_pattern = re.compile("#define\s+(\w+)\s+(\w+)\s*(.*)")
    consts_pattern = re.compile("\s*([A-Z_0-9]+)\s*=\s*(-?\d+)(.*)")
    comments_pattern = re.compile("(:?,)\s*/\*(.*)\*/")
    comments2_pattern = re.compile("^/\*(.*)\*/$")
    comments3_pattern = re.compile("\s*/\*(.*)\*/")
    with open(sys.argv[1]) as fh:
        for line in fh:
            m = macro_pattern.match(line)
            if m:
                m2 = comments3_pattern.match(m.group(3))
                if m2:
                    print("/// {}".format(m2.groups()[-1].strip()))
                print("pub const {}: i32 = {};".format(m.group(1), m.group(2)))
                continue

            m = consts_pattern.match(line)
            if m:
                m2 = comments3_pattern.match(m.group(3))
                if m2:
                    print("/// {}".format(m2.groups()[-1].strip()))
                print("pub const {}: i32 = {};".format(m.group(1), m.group(2)))
                continue

            m = comments2_pattern.match(line)
            if m:
                print("/// {}".format(m.groups()[-1].strip()))
                continue

            print(line, end="")

if __name__ == "__main__":
    main()
