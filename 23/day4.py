test_input = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""

total = 0
with open("input.txt") as f:
    lines = f.read().splitlines()
    lines = lines if len(lines) != 0 else test_input.splitlines()
    size = len(lines)
    lines = [(line, 1) for line in lines]
    i = 0
    while i < len(lines):
        line, count = lines[i]
        winning, mine = line.split("|")
        mine = mine.strip().split()
        winning = winning.split(":")[1].strip().split()
        total_matches = len([number for number in mine if number in winning])
        for j in range(i + 1, i + 1 + total_matches):
            if i >= len(lines):
                continue
            lines[j] = lines[j][0], lines[j][1] + count
        i += 1
    for line in lines:
        total += line[1]
        print(line)
print(total)
