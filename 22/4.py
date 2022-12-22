with open("4.txt", "r") as input_file:
    input = input_file.read().splitlines()


input = [
    [
        set(range(int(x.split("-")[0]), int(x.split("-")[1]) + 1))
        for x in line.split(",")
    ]
    for line in input
]
res = [x for x in input if x[0].issubset(x[1]) or x[1].issubset(x[0])]
print(len(res))
res2 = [x for x in input if not x[0].isdisjoint(x[1])]
print(len(res2))
