test_input = """Time: 46     68     98     66
Distance:   358   1054   1807   1080"""
with open("input.txt") as f:
    lines = f.read().splitlines()
    races = []
    lines = lines if len(lines) != 0 else test_input.splitlines()
    times = [int(time) for time in lines[0].removeprefix("Time:").split()]
    distances = [
        int(dist) for dist in lines[1].removeprefix("Distance:").strip().split()
    ]
    races = list(zip(times, distances))
    ways_to_win = None
    for time, goal_distance in races:
        ways = 0
        for i in range(time):
            distance = i * (time - i)
            if distance > goal_distance:
                ways += 1
        print(ways)
        if ways_to_win is None:
            ways_to_win = ways
        else:
            ways_to_win *= ways
print(ways_to_win)
