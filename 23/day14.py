test_input = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."""


def swap(j1, i1, j2, i2, arr):
    tmp = arr[j1][i1]
    arr[j1][i1] = arr[j2][i2]
    arr[j2][i2] = tmp


def tilt(direction, arr, moving_rocks):
    if direction == "N":
        while True:
            moved_something = False
            for rock_i in range(len(moving_rocks)):
                j, i = moving_rocks[rock_i]
                if j == 0:
                    continue
                move_to = j
                while move_to != 0 and arr[move_to - 1][i] not in "#O":
                    move_to -= 1
                if move_to != j:
                    moved_something = True
                    # print(f"swapping {j},{i} with {j-1},{i}")
                    moving_rocks[rock_i] = (j - 1, i)
                    swap(j, i, j - 1, i, arr)
            if not moved_something:
                break
    if direction == "S":
        while True:
            moved_something = False
            for rock_i in range(len(moving_rocks)):
                j, i = moving_rocks[rock_i]
                if j == len(arr) - 1:
                    continue
                move_to = j
                while move_to != len(arr) - 1 and arr[move_to + 1][i] not in "#O":
                    move_to += 1
                if move_to != j:
                    moved_something = True
                    # print(f"swapping {j},{i} with {j-1},{i}")
                    moving_rocks[rock_i] = (move_to, i)
                    swap(j, i, move_to, i, arr)
            if not moved_something:
                break
    if direction == "W":
        while True:
            moved_something = False
            for rock_i in range(len(moving_rocks)):
                j, i = moving_rocks[rock_i]
                if i == 0:
                    continue
                move_to = i
                while move_to != 0 and arr[j][move_to - 1] not in "#O":
                    move_to -= 1
                if move_to != i:
                    moved_something = True
                    # print(f"swapping {j},{i} with {j-1},{i}")
                    moving_rocks[rock_i] = (j, move_to)
                    swap(j, i, j, move_to, arr)
            if not moved_something:
                break
    if direction == "E":
        while True:
            moved_something = False
            for rock_i in range(len(moving_rocks)):
                j, i = moving_rocks[rock_i]
                if i == len(arr[0]) - 1:
                    continue
                move_to = i
                while move_to != len(arr[0]) - 1 and arr[j][move_to + 1] not in "#O":
                    move_to += 1
                if move_to != i:
                    moved_something = True
                    # print(f"swapping {j},{i} with {j-1},{i}")
                    moving_rocks[rock_i] = (j, move_to)
                    swap(j, i, j, move_to, arr)
            if not moved_something:
                break


with open("input.txt") as f:
    lines = f.read().splitlines()
    if len(lines) == 0:
        print("using test_input")
        lines = test_input.splitlines()
    else:
        print("using input.txt")
    lines = [list(line) for line in lines]
    moving_rocks = []
    for j in range(len(lines)):
        for i in range(len(lines[0])):
            if lines[j][i] == "O":
                moving_rocks.append((j, i))
    tilt("N", lines, moving_rocks)
    load = 0
    for i, line in enumerate(lines):
        for c in line:
            if c == "O":
                load += len(lines) - i
    print(load)
    tilt("W", lines, moving_rocks)
    tilt("S", lines, moving_rocks)
    tilt("E", lines, moving_rocks)
    moving_rocks.sort()
    cycles = 1
    prev_poses = {}
    while cycles < 1000000000:
        prev_poses[moving_rocks.__str__()] = cycles
        tilt("N", lines, moving_rocks)
        tilt("W", lines, moving_rocks)
        tilt("S", lines, moving_rocks)
        tilt("E", lines, moving_rocks)
        cycles += 1
        moving_rocks.sort()
        moving_rosk = moving_rocks.__str__()
        if moving_rosk in prev_poses:
            i = prev_poses[moving_rosk]
            cycle_length = cycles - i
            cycles_left = 1000000000 - cycles
            if cycle_length <= cycles_left:
                print(i, cycle_length)
                cycles += cycle_length * (cycles_left // cycle_length)
    load = 0
    for i, line in enumerate(lines):
        for c in line:
            if c == "O":
                load += len(lines) - i
    print(load)
