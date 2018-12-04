from strutils import parseInt
import tables
import nre
import sequtils
from itertools import product

type
  Claim = tuple[id: int, left: int, top: int, width: int, height: int]
  Cell = tuple[x: int, y: int]


let pattern = re"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"

proc parse_line(line: string): Claim =
  let res = map(toSeq(line.match(pattern).get().captures), parseInt)
  return (res[0], res[1], res[2], res[3], res[4])

iterator all_cells(claim: Claim): Cell =
  for x in claim.left..(claim.left+claim.width-1):
    for y in claim.top..(claim.top+claim.height-1):
      yield (x, y)

var claim_ctr = initCountTable[Cell]()
var claim_ids = initTable[int, Claim]()

for line in stdin.lines:
  let claim = parse_line(line)
  for cell in claim.all_cells:
      claim_ctr.inc(cell)
      claim_ids[claim.id] = claim

var done:bool = true
for id, claim in claim_ids.pairs():
  done = true
  for cell in claim.all_cells:
    if claim_ctr[cell] != 1:
      done = false
      break

  if done:
    echo "Good claim is ", id
    break
