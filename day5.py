import math
import re
import numpy as np
from loguru import logger
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

pairs = []
for i in range(0, len(seeds), 2):
    pairs.append((seeds[i], seeds[i] + seeds[i+1] - 1))
print(pairs)



# for pair in pairs:
#     work = []
#     for ranges in xs:
#         # while len(work):
#             # p = work.pop()
#             for [dst, src, w] in ranges:
#                 x, y = pair
#                 a, b = src, src+w

#                 logger.debug(f"{x}-{y} : {a} {b}")
#                 '''
#                 x-y a-b : no overlap
#                 a-b x-y : no overlap
#                 x-a-b-y : subsume
#                 a-x-y-b : subsume
#                 x-a-y-b : partial
#                 a-x-b-y : partial
#                 '''
#                 if y < a or b < x: # no overlap
#                     logger.debug("no overlap")
#                     continue
#                 elif x <= a and y >= b:
#                     logger.debug("x-a-b-y")
#                     work.extend([(x, a), (a, b), (b, y)])
#                 elif x >= a and x < b and y <= b:
#                     logger.debug("a-x-y-b")
#                     work.extend([(a, x), (x, y), (y, b)])
#                 elif x <= a and y <= b:
#                     logger.debug("x-a-y-b")
#                     work.extend([(x, a), (a, y), (y, b)])
#                 elif x >= a and b <= y:
#                     logger.debug("a-x-b-y")
#                     work.extend([(a, x), (x, b), (b, y)])
#                 else:
#                     logger.error(f"unhandled case {x}-{y} {a}-{b}")
#     print(work)