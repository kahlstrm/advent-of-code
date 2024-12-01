test_input = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

total = 0
with open("input.txt") as f:
    lines = f.readlines()
    for line in lines if len(lines) != 0 else test_input.splitlines():
        max_reds = 0
        max_greens = 0
        max_blues = 0
        line = line.strip()
        if line == "":
            continue
        id, rounds = line.removeprefix("Game ").split(":")
        game_good = True
        id = int(id)
        for throw in rounds.split(";"):
            reds = 0
            greens = 0
            blues = 0
            for blob in throw.split(","):
                amount, color = blob.split()
                amount = int(amount)
                if color == "red":
                    reds += amount
                    if reds > max_reds:
                        max_reds = reds
                elif color == "green":
                    greens += amount
                    if greens > max_greens:
                        max_greens = greens
                elif color == "blue":
                    blues += amount
                    if blues > max_blues:
                        max_blues = blues
                else:
                    raise Exception("wtf bro")
        total += max_blues * max_greens * max_reds
print(total)
