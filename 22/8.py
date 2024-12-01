with open("8.txt") as file:
    input = file.read().splitlines()


def is_visible(i: int, j: int):
    if i == 0 or j == 0 or i == (len(input[j]) - 1) or j == (len(input) - 1):
        return True
    val = input[j][i]
    bigger = False
    for x in range(i):
        if input[j][x] >= val:
            bigger = True
            break
    if not bigger:
        return True
    bigger = False
    for x in range(i + 1, len(input[j])):
        if input[j][x] >= val:
            bigger = True
            break
    if not bigger:
        return True
    bigger = False
    for y in range(j):
        if input[y][i] >= val:
            bigger = True
            break
    if not bigger:
        return True
    bigger = False
    for y in range(j + 1, len(input)):
        if input[y][i] >= val:
            bigger = True
            break
    if not bigger:
        return True
    return False


count = 0
for j in range(len(input)):
    for i in range(len(input[0])):
        if is_visible(i, j):
            count += 1
print(count)


def scenic_score(i: int, j: int):
    if i == 0 or j == 0 or i == (len(input[j]) - 1) or j == (len(input) - 1):
        return 0
    val = input[j][i]
    count_r = 0
    for x in range(i).__reversed__():
        count_r += 1
        if input[j][x] >= val:
            break
    count_l = 0
    for x in range(i + 1, len(input[j])):
        count_l += 1
        if input[j][x] >= val:
            break
    count_u = 0
    for y in range(j).__reversed__():
        count_u += 1
        if input[y][i] >= val:
            break
    count_d = 0
    for y in range(j + 1, len(input)):
        count_d += 1
        if input[y][i] >= val:
            break
    return count_r * count_l * count_u * count_d


best = 0
for j in range(len(input)):
    for i in range(len(input[0])):
        score = scenic_score(i, j)
        if score > best:
            best = score
print(best)
