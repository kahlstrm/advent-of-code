import sys

sys.setrecursionlimit(100000)

test_input = r""".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."""


def draw(visited, arr):
    visited = [(j, i) for j, i, _ in list(visited)]
    for j, line in enumerate(arr):
        for i, c in enumerate(line):
            if (j, i) in visited:
                print("#", end="")
            else:
                print(c, end="")
        print()


def traverse(j, i, arr, coming_from, seen: set):
    if (
        j < 0
        or i < 0
        or j == len(arr)
        or i == len(arr[0])
        or (j, i, coming_from) in seen
    ):
        return seen
    # draw(seen, arr)
    seen.add((j, i, coming_from))
    cur = arr[j][i]
    # print(coming_from, cur)
    if cur == ".":
        if coming_from == "R":
            traverse(j, i - 1, arr, "R", seen)
        elif coming_from == "L":
            traverse(j, i + 1, arr, "L", seen)
        elif coming_from == "T":
            traverse(j + 1, i, arr, "T", seen)
        elif coming_from == "B":
            traverse(j - 1, i, arr, "B", seen)
    elif cur == "|":
        if coming_from in "RL":
            traverse(j + 1, i, arr, "T", seen)
            traverse(j - 1, i, arr, "B", seen)
        elif coming_from == "B":
            traverse(j - 1, i, arr, "B", seen)
        elif coming_from == "T":
            traverse(j + 1, i, arr, "T", seen)
    elif cur == "-":
        if coming_from in "BT":
            traverse(j, i - 1, arr, "R", seen)
            traverse(j, i + 1, arr, "L", seen)
        elif coming_from == "R":
            traverse(j, i - 1, arr, "R", seen)
        elif coming_from == "L":
            traverse(j, i + 1, arr, "L", seen)
    elif cur == "\\":
        if coming_from == "R":
            traverse(j - 1, i, arr, "B", seen)
        elif coming_from == "L":
            traverse(j + 1, i, arr, "T", seen)
        elif coming_from == "T":
            traverse(j, i + 1, arr, "L", seen)
        elif coming_from == "B":
            traverse(j, i - 1, arr, "R", seen)
    elif cur == "/":
        if coming_from == "R":
            traverse(j + 1, i, arr, "T", seen)
        elif coming_from == "L":
            traverse(j - 1, i, arr, "B", seen)
        elif coming_from == "T":
            traverse(j, i - 1, arr, "R", seen)
        elif coming_from == "B":
            traverse(j, i + 1, arr, "L", seen)


with open("input.txt") as f:
    lines = f.read().splitlines()
    if len(lines) == 0:
        print("using test_input")
        lines = test_input.splitlines()
    else:
        print("using input.txt")
    lines = [list(line) for line in lines]
    max_visited = -1
    for i in range(len(lines[0])):
        visited = set()
        traverse(0, i, lines, "T", visited)
        visited = [(j, i) for j, i, _ in list(visited)]
        max_visited = max(len(set(visited)), max_visited)
        visited = set()
        traverse(len(lines) - 1, i, lines, "B", visited)
        visited = [(j, i) for j, i, _ in list(visited)]
        max_visited = max(len(set(visited)), max_visited)
    for j in range(len(lines)):
        visited = set()
        traverse(j, 0, lines, "L", visited)
        visited = [(j, i) for j, i, _ in list(visited)]
        if j == 0:
            p1 = len(set(visited))
        max_visited = max(len(set(visited)), max_visited)
        visited = set()
        traverse(j, len(lines[0]) - 1, lines, "R", visited)
        visited = [(j, i) for j, i, _ in list(visited)]
        max_visited = max(len(set(visited)), max_visited)
    print(p1)
    print(max_visited)
