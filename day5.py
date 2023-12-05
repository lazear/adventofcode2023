import math
import re
import numpy as np
from collections import defaultdict

maps = """\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4\
""".splitlines()

# maps = open("input/day5").read().splitlines()

seeds = [int(x) for x in re.findall("(\d+)", maps[0])]

xs = []
name = ""
curr = []
for row in maps[1:]:
    if len(row) == 0:
        if len(curr) > 0:
            xs.append(curr)
            curr = []
    elif row[0].isdigit():
        row = [int(x) for x in re.findall("(\d+)", row)]
        curr.append(row)
xs.append(curr)


def run(seeds):
    lowest_ptr = math.inf
    for seed in seeds:
        ptr = seed
        for ranges in xs:
            for [dst, src, width] in ranges:
                if ptr >= src and ptr <= src + width:
                    ptr = dst + ptr - src
                    break
        lowest_ptr = min(lowest_ptr, ptr)
    return lowest_ptr


print(f"part_1={run(seeds)}")

# start with bottom range
# for [dest, src, width] in xs[::-1]:


for i in range(len(xs)):
    # can we break up this range into smaller ones?
    for [d1, s1, w1] in xs[i]:
        A, B = (d1, d1 + w1 - 1)
        print(f"top {s1}..{s1+w1} => {d1}..{d1+w1}")
        for j in range(i + 1, len(xs)):
            for [d2, s2, w2] in xs[j]:
                C, D = (s2, s2 + w2 - 1)
                print(f" -b {s2}..{s2+w2} => {d2}..{d2+w2}")
                if max(A, C) <= min(B, D):
                    print(f"overlap? ({d1}..{d1+w1}) & ({s2}..{s2+w2})")

            break
        # break
    break
