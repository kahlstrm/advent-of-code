with open("1.txt", "r") as input_file:
    input = input_file.read()

input = input.split("\n\n")
input = [sum([int(number) for number in elf.split("\n")]) for elf in input]
input.sort(reverse=True)
print(sum(input[:3]))
