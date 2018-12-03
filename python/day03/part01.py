import sys
import re
from collections import Counter
import itertools as it

pattern = re.compile(r'\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')


def parse_line(line):
    row = pattern.match(line)
    id = row[1]
    left = int(row[2])
    top = int(row[3])
    w = int(row[4])
    h = int(row[5])
    return id, left, top, w, h


claims = Counter((x, y)
                 for _, left, top, w, h in map(parse_line, sys.stdin)
                 for x, y in it.product(range(left, left+w), range(top, top+h))
                 )

answer = sum(1 for _, ctr in claims.items() if ctr >= 2)
print(answer)
