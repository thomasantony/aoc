import sys
import re
import collections as cl
import itertools as it
pattern = re.compile(r'\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')


def parse_line(line):
    return map(int, pattern.findall(line)[0])


claim_counter = cl.Counter((x, y)
                           for _, left, top, w, h in map(parse_line, sys.stdin)
                           for x, y in it.product(range(left, left+w), range(top, top+h))
                           )

answer = sum(1 for _, ctr in claim_counter.items() if ctr >= 2)
print(answer)
