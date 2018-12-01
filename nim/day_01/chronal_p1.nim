from strutils import parseInt

var total: int
var line: string

for line in stdin.lines:
  let number:int = parseInt(line)
  total = total + number

echo "Total is ", total
