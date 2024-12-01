test_input = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"""

total_1 = 0
total_2 = 0
with open("input.txt") as f:
    lines = f.read().splitlines()
    lines = lines if len(lines) != 0 else test_input.splitlines()
    size = len(lines)
    lines = [[int(i) for i in line.split()] for line in lines]
    seqs = []
    for line_i in range(len(lines)):
        line = lines[line_i]
        seqs.append([line])
        while not all(i == 0 for i in seqs[line_i][-1]):
            cur = seqs[line_i][-1]
            new_one = []
            for i in range(len(cur) - 1):
                new_one.append(cur[i + 1] - cur[i])
            seqs[line_i].append(new_one)
    for seq in seqs:
        seq[-1].insert(0, 0)
        seq[-1].append(0)
        for line_i in reversed(range(1, len(seq))):
            cur_bot = seq[line_i]
            cur_top = seq[line_i - 1]
            cur_top.insert(0, cur_top[0] - cur_bot[0])
            cur_top.append(cur_top[-1] + cur_bot[-1])
    for seq in seqs:
        total_1 += seq[0][-1]
        total_2 += seq[0][0]
    print(total_1)
    print(total_2)
