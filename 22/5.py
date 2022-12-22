with open("5.txt", "r") as input_file:
    input = input_file.read().split("\n\n")
formation = input[0].splitlines()
count = int(formation[-1].rstrip().split(" ")[-1])
formation = [
    [
        line[i : (i + 4)].strip().replace("[", "").replace("]", "")
        for i in range(0, len(line), 4)
    ]
    for line in formation[:-1]
]
moves = input[1].splitlines()
print(formation)
stacks = [[] for i in range(count)]
print(stacks)
formation.reverse()
for line in formation:
    for i in range(len(line)):
        if line[i] != "":
            stacks[i].append(line[i])
print(stacks)
for move in moves:
    move = (
        move.rstrip()
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
        .split(" ")
    )
    amount, where, to = [int(num) for num in move]
    popped = []
    for _ in range(amount):
        popped.append(stacks[where - 1].pop())
    popped.reverse()
    stacks[to - 1] += popped
print("".join([stack[-1] for stack in stacks]))
