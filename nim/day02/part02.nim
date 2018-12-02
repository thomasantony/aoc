from sequtils import toSeq, zip
from itertools import combinations

# proc is_good_pair(pair: seq[string]): bool  {. noSideEffect .}=
proc is_good_pair(pair: seq[string]): bool =
  var bad_count = 0
  for c in zip(pair[0], pair[1]):
    if c.a != c.b:
      bad_count += 1
    if bad_count > 1:
      return false
  return true

echo "Answer is"
let all_lines = toSeq(stdin.lines)
for pair in combinations(all_lines, 2):
  if is_good_pair(pair):
    for c in zip(pair[0], pair[1]):
      if c.a != c.b:
        continue
      stdout.write c.a
    break
echo ""
