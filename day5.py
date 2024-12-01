test_input = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""
parts = []
total = 0
with open("input.txt") as f:
    lines = f.read().splitlines()
    mappings = {}
    cur_map = ""
    for line in lines if len(lines) != 0 else test_input.splitlines():
        if len(line) == 0:
            cur_map = ""
            continue
        if cur_map != "":
            dest_s, source_s, _range = [int(i) for i in line.split(" ")]
            mappings[cur_map].append((source_s, dest_s, _range))
            continue
        if line.startswith("seeds: "):
            seeds = [int(seed) for seed in line.split(": ")[1].split(" ")]
            real_seeds = []
            i = 0
            while i < len(seeds):
                seed_s = seeds[i]
                seed_range = seeds[i + 1]
                real_seeds.append((seed_s, seed_range))
                i += 2
            seeds = real_seeds
        if line.endswith("map:"):
            cur_map = line.removesuffix(" map:")
            mappings[cur_map] = []
    lowest = None
    maps = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]
    # for seed_s, seed_range in seeds:
    #     for seed in range(seed_s, seed_s + seed_range):
    #         cur = seed
    #         for map in maps:
    #             for source_s, dest_s, _range in mappings[map]:
    #                 if cur >= source_s and cur < source_s + _range:
    #                     cur = dest_s + cur - source_s
    #                     break
    #         if lowest is None or cur < lowest:
    #             lowest = cur
    for map in maps:
        i = 0
        print("\n" + map + "\n")
        while i < len(seeds):
            seed_s, seed_range = seeds[i]
            seed_e = seed_s + seed_range
            for source_s, dest_s, _range in mappings[map]:
                source_e = source_s + _range
                print("looppi")
                print(f"seed:{seed_s}-{seed_e}")
                print(f"source:{source_s}-{source_e}")
                if source_e <= seed_s or source_s >= seed_e:
                    print("outside of range")
                    continue
                # whole seed range fits inside source range
                if source_e >= seed_e and source_s <= seed_s:
                    print("whole seed range fits inside source range")
                    print("dest", dest_s)
                    seeds[i] = (dest_s + seed_s - source_s, seed_range)
                    print(seeds[i])
                    break
                # seed range starts inside but does not fit completely, need to split
                if source_e < seed_e and source_s <= seed_s:
                    print(
                        "seed range starts inside but does not fit completely, need to split"
                    )
                    print("dest", dest_s)
                    new_range = (source_e, seed_e - source_e)
                    seeds.append(new_range)
                    print(new_range)
                    seeds[i] = (dest_s + seed_s - source_s, source_e - seed_s)
                    print(seeds[i])
                    break
                # seed range starts outside range and ends inside range
                if source_e >= seed_e and source_s > seed_s:
                    print("seed range starts outside range and ends inside range")
                    new_range = (seed_s, source_s - seed_s)
                    print(new_range)
                    seeds.append(new_range)
                    seeds[i] = (dest_s, _range)
                    print(seeds[i])
                    break
                # seed range has an inside section
                print("seed range has an inside section")
                seeds[i] = (dest_s, _range)
                print(seeds[i])
                print((seed_s, source_s - seed_s))
                seeds.append((seed_s, source_s - seed_s))
                seeds.append((source_e, seed_e - source_e))
                print((source_e, seed_e - source_e))
                break
            i += 1
    print(seeds)
    lowest = None
    for seed in seeds:
        if lowest is None or seed[0] < lowest:
            lowest = seed[0]
    print(lowest)
