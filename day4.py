import re
from collections import defaultdict

cards = """\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
""".splitlines()
cards = open("input/day4").read().splitlines()

instances = defaultdict(lambda: 1) 

p1 = 0
for card in cards:
    n = int(re.search("Card\s+(\d+):", card).groups()[0])
    left = card.split('|')[0].split(':')[1]
    right = card.split('|')[1]
    w = [int(x) for x in re.findall("(\d+)", left)]
    m = [int(x) for x in re.findall("(\d+)", right)]
    matches = len(set(w).intersection(m))
    points = int(2**(matches - 1))
    p1 += points

    for repeat in range(instances[n - 1]):
        for match in range(matches):
            instances[n + match] += 1

print(p1)
print(sum(instances[k] for k in range(len(cards))))