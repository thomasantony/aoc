import sys
import string
import itertools as it

matching_pairs = list(it.chain.from_iterable((l+u, u+l)
                      for l, u in zip(string.ascii_lowercase, string.ascii_uppercase)))

data = sys.stdin.readline().strip()

i = 0
while i < len(data):
    pair = data[i:i+2]
    if pair in matching_pairs:
        data = data[:i] + data[i+2:]
        i -= 1
    else:
        i += 1

print(len(data))
