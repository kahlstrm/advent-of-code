test_input = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""
total = 0
digits = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def get_num(num):
    for d in digits:
        if d in num:
            return digits[d]


lines = open("input.txt").read().splitlines()
for line in lines if len(lines) != 0 else test_input.splitlines():
    line = line.strip()
    if len(line) == 0:
        continue
    num = ""
    for char in line:
        if char.isdigit():
            num = char
            break
        else:
            num += char
            thing = get_num(num)
            if thing is not None:
                num = str(thing)
                break
    second = ""
    for char in line[::-1]:
        if char.isdigit():
            second = char
            break
        else:
            second += char
            thing = get_num(second[::-1])
            if thing is not None:
                second = str(thing)
                break
    num = int(num + second)
    print(num)
    total += num
print(total)
