# only 12 red cubes, 13 green cubes, and 14 blue
import re
from functools import reduce


# l="""Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
# Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
# Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
# Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
# Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

limits = {"red": 12, "green": 13, "blue": 14}

l = open("/Users/michael/Downloads/input").read()

s = 0
p = 0
for line in l.splitlines():
    line = line.strip()
    if len(line) == 0:
        continue
    gid = re.match("Game (\d+):", line).groups()[0]

    allowed = True
    mins = {
        "green": 0,
        "red": 0,
        "blue": 0,
    }
    for game in line.split(";"):
        colors = {
            "green": 0,
            "red": 0,
            "blue": 0,
        }
        for c in colors.keys():
            if m := re.search(f"(\d+) {c}", game):
                colors[c] += int(m.groups()[0])
                mins[c] = max(mins[c], int(m.groups()[0]))
            if colors[c] > limits[c]:
                allowed = False
    power = reduce(lambda x, y: x * y, mins.values(), 1)
    print(gid, allowed, mins, power)
    p += power
    if allowed:
        s += int(gid)
print(s)
print(p)
