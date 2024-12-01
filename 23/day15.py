test_input = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"""


def hash(string):
    value = 0
    for c in string:
        value += ord(c)
        value *= 17
        value %= 256
    return value


with open("input.txt") as f:
    lines = f.read().splitlines()
    if len(lines) == 0:
        print("using test_input")
        lines = test_input.splitlines()
    else:
        print("using input.txt")
    values = [hash(v) for v in lines[0].split(",")]
    boxes: list[list[(str, int)]] = []
    print(sum(values))
    for i in range(256):
        boxes.append([])
    for v in lines[0].split(","):
        label = v.replace("-", "=").split("=")[0]
        idx = hash(label)
        if "-" in v:
            boxes[idx] = [thing for thing in boxes[idx] if thing[0] != label]
        else:
            value = v.split("=")[1]
            value = int(value)
            found = False
            for v_i, v in enumerate(boxes[idx]):
                if v[0] == label:
                    boxes[idx][v_i] = (label, value)
                    found = True
                    break
            if not found:
                boxes[idx].append((label, value))
    total = 0
    for i, box in enumerate(boxes):
        if box:
            for lens_i, lens in enumerate(box):
                total += (i + 1) * (lens_i + 1) * lens[1]
    print(total)
