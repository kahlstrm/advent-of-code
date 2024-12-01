test_input = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""


def is_ver_reflection(i, pattern):
    left_side = reversed(range(i))
    right_side = range(i, len(pattern[0]))
    for l_i, r_i in zip(left_side, right_side):
        for j in range(len(pattern)):
            left_one = pattern[j][l_i]
            right_one = pattern[j][r_i]
            if left_one != right_one:
                return False
    return True


def swap(j, i, pattern):
    if pattern[j][i] == 0:
        pattern[j][i] = 1
    else:
        pattern[j][i] = 0


def is_hor_reflection(j, pattern):
    top_side = reversed(range(j))
    bottom_side = range(j, len(pattern))
    for t_i, b_i in zip(top_side, bottom_side):
        for i in range(len(pattern[0])):
            if pattern[t_i][i] != pattern[b_i][i]:
                return False
    return True


with open("input.txt") as f:
    lines = f.read().splitlines()
    if len(lines) == 0:
        print("using test_input")
    else:
        print("using input.txt")
    lines = lines if len(lines) != 0 else test_input.splitlines()
    patterns = []
    cur = []
    for line in lines:
        if len(line) != 0:
            cur.append([0 if c == "." else 1 for c in line])
        else:
            patterns.append(cur)
            cur = []
    patterns.append(cur)
    p1_reflection_lines = []
    for pattern in patterns:
        found = False
        if found:
            break
        for j in range(1, len(pattern)):
            if is_hor_reflection(j, pattern):
                # print(f"horizontal starting at {j}:")
                p1_reflection_lines.append((j, None))
                found = True
                break
        if not found:
            for i in range(1, len(pattern[0])):
                if is_ver_reflection(i, pattern):
                    # print(f"vertical starting at {i}:")
                    p1_reflection_lines.append((None, i))
                    found = True
                    break
        # if found:
        #     for line in pattern:
        #         print("".join("." if c == 0 else "#" for c in line))
        if not found:
            raise Exception("not found reflection")
    print("p2")
    p2_reflection_lines = []
    for p_i, pattern in enumerate(patterns):
        found = False
        for y in range(len(pattern)):
            if found:
                break
            for x in range(len(pattern[0])):
                if found:
                    break
                swap(y, x, pattern)
                for j in range(1, len(pattern)):
                    if (
                        is_hor_reflection(j, pattern)
                        and (j, None) != p1_reflection_lines[p_i]
                    ):
                        # print(f"horizontal starting at {j}:")
                        p2_reflection_lines.append((j, None))
                        found = True
                        break
                if not found:
                    for i in range(1, len(pattern[0])):
                        if (
                            is_ver_reflection(i, pattern)
                            and (None, i) != p1_reflection_lines[p_i]
                        ):
                            # print(f"vertical starting at {i}:")
                            p2_reflection_lines.append((None, i))
                            found = True
                            break
                # if found:
                #     for line in pattern:
                #         print("".join("." if c == 0 else "#" for c in line))
                swap(y, x, pattern)
        if not found:
            raise Exception("not found reflection")
    total1 = 0
    total2 = 0
    for j, i in p1_reflection_lines:
        if j:
            total1 += j * 100
        if i:
            total1 += i
    for j, i in p2_reflection_lines:
        if j:
            total2 += j * 100
        if i:
            total2 += i
    print(total1)
    print(total2)
