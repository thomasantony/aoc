from strutils import parseInt
import sequtils

echo "Total is ", foldl(map(toSeq(stdin.lines), parseInt), a+b)
