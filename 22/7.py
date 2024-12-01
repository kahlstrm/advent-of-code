with open("7.txt") as file:
    input = file.read().splitlines()
dirs = []


def process_folder(i: int, folder: str, size: int):
    if i >= len(input):
        dirs.append(size)
        return size, i
    line = input[i]
    if line.startswith("$"):
        if line.endswith("ls"):
            return process_folder(i + 1, folder, size)
        elif line.endswith(".."):
            dirs.append(size)
            return size, i
        else:
            subfolder = line.split(" ")[-1]
            sub_size, i = process_folder(i + 1, subfolder, 0)
            return process_folder(i + 1, folder, size + sub_size)
    else:
        if line.startswith("dir"):
            return process_folder(i + 1, folder, size)
        else:
            return process_folder(i + 1, folder, size + int(line.split(" ")[0]))


process_folder(1, input[0].split(" ")[-1], 0)
# first
print(sum(dir for dir in dirs if dir <= 100000))
dirs.sort()
free = 70000000 - (dirs[-1])
# second
print(next((size for size in dirs if size + free >= 30000000), None))
