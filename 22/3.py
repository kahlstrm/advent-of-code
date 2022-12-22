import string

chars = list(string.ascii_letters)
with open("3.txt", "r") as input_file:
    input = input_file.read().splitlines()


def find_same(a: str, b: str):
    first = set(a)
    second = set(b)
    return first.intersection(second).pop()


def find_badge(a: str, b: str, c: str):
    first = set(a)
    second = set(b)
    third = set(c)
    return first.intersection(second).intersection(third).pop()


res = [find_same(line[: (len(line) // 2)], line[(len(line) // 2) :]) for line in input]
print(sum([1 + chars.index(char) for char in res]))
res2 = [
    find_badge(first, second, third)
    for [first, second, third] in [input[i : i + 3] for i in range(0, len(input), 3)]
]
print(sum([1 + chars.index(char) for char in res2]))
