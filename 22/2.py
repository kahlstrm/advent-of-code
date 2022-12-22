with open("2.txt", "r") as input_file:
    input = input_file.readlines()

points = {"X": 1, "Y": 2, "Z": 3}
signs = {"A": "X", "B": "Y", "C": "Z"}
score = 0


def points_from_match(result, you):
    if result == "Y":
        return 3 + points[you]
    if result == "X":
        if you == "X":
            return points["Z"]
        if you == "Y":
            return points["X"]
        if you == "Z":
            return points["Y"]
    if result == "Z":
        if you == "Z":
            return 6 + points["X"]
        if you == "X":
            return 6 + points["Y"]
        if you == "Y":
            return 6 + points["Z"]


for line in input:
    line = line.rstrip().split(" ")
    score += points_from_match(line[1], signs[line[0]])

print(score)
