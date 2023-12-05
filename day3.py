import numpy as np
from typing import List
import re

test = r"""467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..""".splitlines()

test = open("input/day3").read().splitlines()

grid = [list(t) for t in test]

symbols = []
for i, r in enumerate(grid):
    for j, c in enumerate(r):
        if not c.isdigit() and c != '.':
            symbols.append((c, i, j))


removed = [['.' for x in row] for row in grid]

gears = []
for (sym, r, c) in symbols:
    # clear row above, same row, row below
    removed[r][c] = sym
    local = [['.' for _ in grid[r]] for _ in grid]
    for row in range(max(0, r - 1), min(len(grid), r + 2)):
        if grid[row][c].isdigit():
            removed[row][c] = grid[row][c]
            local[row][c] = grid[row][c]
            grid[row][c] = '.'
        i = c + 1
        while i < len(grid[row]) and grid[row][i].isdigit():
            removed[row][i] = grid[row][i]
            local[row][i] = grid[row][i]
            grid[row][i] = '.'
            i += 1
        i = c - 1
        while i >= 0 and grid[row][i].isdigit():
            removed[row][i] = grid[row][i]
            local[row][i] = grid[row][i]
            grid[row][i] = '.'
            i -= 1
    if sym == '*':
        nums = []
        for row in [''.join(r) for r in local]:
            if m := re.findall("(\d+)", row):
                nums.extend(int(i) for i in m)
        if len(nums) == 2:
            gears.append(nums[0] * nums[1])

nums = []
for row in [''.join(r) for r in removed]:
    if m := re.findall("(\d+)", row):
        nums.extend(int(i) for i in m)

print(sum(nums))
print(sum(gears))