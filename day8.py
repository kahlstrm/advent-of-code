from math import lcm

test_input = """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"""

total = 0
with open("input.txt") as f:
    lines = f.read().splitlines()
    lines = lines if len(lines) != 0 else test_input.splitlines()
    size = len(lines)
    walk_order = [0 if c == "L" else 1 for c in lines[0]]
    walk_order_len = len(walk_order)
    nodes = {}
    for rest in lines[1:]:
        if len(rest) == 0:
            continue
        splitted = [a.strip() for a in rest.split("=")]
        cur_node = splitted[0]
        left, right = splitted[1].removeprefix("(").removesuffix(")").split(", ")
        nodes[cur_node] = (left, right)
    starts = [node for node in nodes.keys() if node[-1] == "A"]
    cur_nodes = starts[::]
    end_nodes = {}
    print(cur_nodes)
    while len(end_nodes) != len(cur_nodes):
        for o in walk_order:
            total += 1
            if total % 100000 == 0:
                print(total)
            for i in range(len(cur_nodes)):
                cur_nodes[i] = nodes[cur_nodes[i]][o]
            for i in range(len(cur_nodes)):
                cur_node = cur_nodes[i]
                if cur_node[-1] == "Z" and cur_node not in end_nodes:
                    end_nodes[cur_node] = total
    print(end_nodes)
    print(lcm(*(list(end_nodes.values()) + [len(walk_order)])))
    print(total)
