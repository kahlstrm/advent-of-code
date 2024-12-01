from itertools import permutations

test_input = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""

total_1 = 0
total_2 = 0
expansion_size_1 = 2
expansion_size_2 = 1000000
with open("input.txt") as f:
    lines = f.read().splitlines()
    if len(lines) == 0:
        print("using test_input")
    else:
        print("using input.txt")
    lines = lines if len(lines) != 0 else test_input.splitlines()
    expanded_x = set(
        i
        for i in range(len(lines[0]))
        if all(lines[j][i] != "#" for j in range(len(lines)))
    )
    expanded_y = set(
        j
        for j in range(len(lines))
        if all(lines[j][i] != "#" for i in range(len(lines[j])))
    )
    stars = []
    for j in range(len(lines)):
        for i in range(len(lines[0])):
            if lines[j][i] == "#":
                stars.append((j, i))

    def get_distance(a, b, expansion_size):
        range_y = range(min(b[0], a[0]), max(b[0], a[0]))
        count_expanded_y = len(expanded_y.intersection(range_y))
        range_x = range(min(b[1], a[1]), max(b[1], a[1]))
        count_expanded_x = len(expanded_x.intersection(range_x))
        other = (
            abs(a[1] - b[1]) + abs(a[0] - b[0]) - count_expanded_y - count_expanded_x
        )
        return (count_expanded_x + count_expanded_y) * expansion_size + other

    distance_total_1 = sum(
        get_distance(a, b, expansion_size_1) for a, b in permutations(stars, 2) if a > b
    )
    distance_total_2 = sum(
        get_distance(a, b, expansion_size_2) for a, b in permutations(stars, 2) if a > b
    )
    print(distance_total_1)
    print(distance_total_2)
