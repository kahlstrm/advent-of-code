test_input = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""
parts = []
total = 0
with open("input.txt") as f:
    lines = f.read().splitlines()
    engine: list[str] = []
    for line in lines if len(lines) != 0 else test_input.splitlines():
        if len(line) == 0:
            continue
        engine.append(line)
    for row_i in range(len(engine)):
        row = engine[row_i]
        for char_i in range(len(row)):
            char = row[char_i]
            if not char.isdigit() and char != ".":
                adjacent_numbers = []
                try:
                    top_row = engine[row_i - 1]
                    if top_row[char_i].isdigit():
                        cur = char_i
                        part_num = ""
                        while cur > 0 and top_row[cur - 1].isdigit():
                            cur -= 1
                        while cur < len(top_row) and top_row[cur].isdigit():
                            part_num += top_row[cur]
                            cur += 1
                        print(part_num, row_i)
                        parts.append(int(part_num))
                        adjacent_numbers.append(int(part_num))
                    else:
                        if char_i != 0 and top_row[char_i - 1].isdigit():
                            cur = char_i - 1
                            part_num = ""
                            while cur > 0 and top_row[cur - 1].isdigit():
                                cur -= 1
                            while cur < len(top_row) and top_row[cur].isdigit():
                                part_num += top_row[cur]
                                cur += 1
                            print(part_num, row_i)
                            parts.append(int(part_num))
                            adjacent_numbers.append(int(part_num))
                        if top_row[char_i + 1].isdigit():
                            cur = char_i + 1
                            part_num = ""
                            while cur < len(top_row) and top_row[cur].isdigit():
                                part_num += top_row[cur]
                                cur += 1
                            print(part_num, row_i)
                            parts.append(int(part_num))
                            adjacent_numbers.append(int(part_num))
                except IndexError:
                    pass
                if char_i != 0 and row[char_i - 1].isdigit():
                    count = char_i - 1
                    part_num = ""
                    while count >= 0 and row[count].isdigit():
                        part_num += row[count]
                        count -= 1
                    print(part_num[::-1], row_i)
                    parts.append(int(part_num[::-1]))
                    adjacent_numbers.append(int(part_num[::-1]))
                if char_i != len(row) - 1 and row[char_i + 1].isdigit():
                    count = char_i + 1
                    part_num = ""
                    while count != len(row) and row[count].isdigit():
                        part_num += row[count]
                        count += 1
                    print(part_num, row_i)
                    parts.append(int(part_num))
                    adjacent_numbers.append(int(part_num))
                try:
                    bot_row = engine[row_i + 1]
                    if bot_row[char_i].isdigit():
                        cur = char_i
                        part_num = ""
                        while cur > 0 and bot_row[cur - 1].isdigit():
                            cur -= 1
                        while cur < len(bot_row) and bot_row[cur].isdigit():
                            part_num += bot_row[cur]
                            cur += 1
                        print(part_num, row_i)
                        parts.append(int(part_num))
                        adjacent_numbers.append(int(part_num))
                    else:
                        if char_i != 0 and bot_row[char_i - 1].isdigit():
                            cur = char_i - 1
                            part_num = ""
                            while cur > 0 and bot_row[cur - 1].isdigit():
                                cur -= 1
                            while cur < len(bot_row) and bot_row[cur].isdigit():
                                part_num += bot_row[cur]
                                cur += 1
                            print(part_num, row_i)
                            parts.append(int(part_num))
                            adjacent_numbers.append(int(part_num))
                        if bot_row[char_i + 1].isdigit():
                            cur = char_i + 1
                            part_num = ""
                            while cur < len(bot_row) and bot_row[cur].isdigit():
                                part_num += bot_row[cur]
                                cur += 1
                            print(part_num, row_i)
                            parts.append(int(part_num))
                            adjacent_numbers.append(int(part_num))
                except IndexError:
                    pass
                if len(adjacent_numbers) == 2 and char == "*":
                    total += adjacent_numbers[0] * adjacent_numbers[1]

print(sum(parts))
print(total)
