with open("6.txt") as file:
    input = file.read()


def alldifferent(arr: list[str]):
    return len(set(arr)) == 14


arr = []
for i in range(len(input)):
    if len(arr) == 14:
        arr.pop(0)
    arr.append(input[i])
    if alldifferent(arr):
        print(arr)
        print(i + 1)
        break
