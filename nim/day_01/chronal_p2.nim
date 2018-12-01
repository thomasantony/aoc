from strutils import parseInt
from itertools import cycle  #  nimble install itertools

var freq_list: seq[int]
var total: int
var all_lines: seq[string]

for line in stdin.lines:
  let number:int = parseInt(line)
  total = total + number
  all_lines.add(line)
  freq_list.add(total)

# Loop through list infinite number of times until repeated frequency is found
for line in all_lines.cycle:
  let number:int = parseInt(line)
  total = total + number
  if total in freq_list:
    echo "Answer is ", total
    break
