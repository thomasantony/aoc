import sys
import string
import itertools as it

matching_pairs = list(it.chain.from_iterable((l+u, u+l)
                      for l, u in zip(string.ascii_lowercase, string.ascii_uppercase)))

data = sys.stdin.readline().strip()


def fully_react(data):
    i = 0
    while i < len(data):
        pair = data[i:i+2]
        if pair in matching_pairs:
            data = data[:i] + data[i+2:]
            i -= 1
        else:
            i += 1
    return len(data)


answer = min(fully_react(data.replace(char, '').replace(char.upper(), ''))
             for char in string.ascii_lowercase)
print('Answer', answer)
