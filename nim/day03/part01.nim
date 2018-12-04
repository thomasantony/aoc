from strutils import parseInt
import tables
import nre
import sequtils
from itertools import product

type
  Claim = tuple[left: int, top: int, width: int, height: int]
  Cell = tuple[x: int, y: int]


let pattern = re"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"

proc parse_line(line: string): Claim =
  let res = map(toSeq(line.match(pattern).get().captures), parseInt)
  return (res[1], res[2], res[3], res[4])

var claim_ctr = initCountTable[Cell]()

for line in stdin.lines:
  let claim = parse_line(line)
  for x in claim.left..(claim.left+claim.width-1):
    for y in claim.top..(claim.top+claim.height-1):
      claim_ctr.inc((x, y))


var ctr:int = 0
for cell, count in claim_ctr.pairs:
  if count >= 2:
    ctr += 1

echo "Answer is ", ctr
