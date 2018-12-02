from strutils import parseInt, split
from sequtils import toSeq, deduplicate
import tables

var two_count:int = 0
var three_count:int = 0
for line in stdin.lines:
  var freq_map = toCountTable[char](toSeq(line.items))
  let lengths = deduplicate(toSeq(values(freq_map)))
  if 2 in lengths:
    two_count += 1
  if 3 in lengths:
    three_count += 1

echo "Checksum is ", two_count * three_count
