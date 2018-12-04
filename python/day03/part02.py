import sys
import re
import collections as cl
import itertools as it

pattern = re.compile(r'\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')


def parse_line(line):
    return map(int, pattern.findall(line)[0])


def all_cells(left, top, w, h):
    return it.product(range(left, left+w), range(top, top+h))


claim_ids = {id: (left, top, w, h) for id, left, top, w, h in map(parse_line, sys.stdin)}
claim_counter = cl.Counter((x, y) for _, claim in claim_ids.items()
                           for x, y in all_cells(*claim))

good_claim_id = next(id for id, claim in claim_ids.items()
                     if all(claim_counter[(x, y)] == 1 for x, y in all_cells(*claim)))

print('Good claim id', good_claim_id)
