from sequtils import toSeq, zip, filter, map
from itertools import combinations

proc is_good_pair(pair: seq[string]): bool  {. noSideEffect .}=
  var bad_count = 0
  for c in zip(pair[0], pair[1]):
    if c.a != c.b:
      bad_count += 1
      if bad_count > 1:
        return false
  return true


for pair in combinations(toSeq(stdin.lines), 2):
  if not is_good_pair(pair):
    continue
  let output = map(filter(zip(pair[0], pair[1]),
                          proc (x: tuple) : bool = x.a == x.b),
                          proc (y: tuple) : char = y.a)
  echo "Answer is : ", cast[string](output)

  break
