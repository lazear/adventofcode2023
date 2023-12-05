l = open("/Users/michael/Downloads/day1.txt").readlines()
d = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "ten": 10,
}


s = 0
for line in l:
    line = line.strip()

    n = []
    # for (x, y) in d.items():
    #     line = line.replace(x, str(y))
    for pos in range(0, len(line)):
        for name, value in d.items():
            if line[pos:].startswith(name):
                n.append(value)
        if line[pos].isdigit():
            n.append(int(line[pos]))
    s += n[0] * 10 + n[-1]

    # if len(line) == 0:
    #     continue
    # n = []
    # for ch in line:
    #     if ch.isdigit():
    #         n.append(ch)
    # s += int(f'{n[0]}{n[-1]}')
print(s)
