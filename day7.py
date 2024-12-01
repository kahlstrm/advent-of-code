test_input = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""

card_convert = {
    v[1]: f"{v[0]:02d}"
    for v in enumerate(
        ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"]
    )
}
total = 0
with open("input.txt") as f:
    lines = f.read().splitlines()
    lines = lines if len(lines) != 0 else test_input.splitlines()
    size = len(lines)
    fives = []
    fours = []
    full_h = []
    threes = []
    two_pairs = []
    pairs = []
    highs = []

    def parse_hand(line: str):
        jokers = 0
        cards, bet = line.split()
        bet = int(bet)
        converted_cards = "".join([card_convert[c] for c in cards])
        res = (converted_cards, bet)
        hand = {}
        for card in cards:
            if card == "J":
                jokers += 1
            elif card not in hand:
                hand[card] = 1
            else:
                hand[card] += 1
        values = sorted(hand.values(), reverse=True)
        print(values)
        if len(values) <= 1:
            fives.append(res)
            return
        best = values[0]
        second_best = values[1]
        if best + jokers >= 5:
            fives.append(res)
            return
        if best + jokers == 4:
            fours.append(res)
            return
        potential_b = best
        potential_sb = second_best
        placeholder = jokers
        while placeholder > 0:
            if potential_b < 3:
                potential_b += 1
            else:
                potential_sb += 1
            placeholder -= 1
        if potential_b + potential_sb == 5:
            full_h.append(res)
            return

        if best + jokers == 3:
            threes.append(res)
            return
        if best == 2 and second_best == 2 and jokers == 0:
            two_pairs.append(res)
            return
        if (best == 1 and jokers > 0) or best == 2:
            pairs.append(res)
            return
        highs.append(res)

    for line in lines:
        parse_hand(line)
    fives.sort(key=lambda c: c[0])
    fours.sort(key=lambda c: c[0])
    full_h.sort(key=lambda c: c[0])
    threes.sort(key=lambda c: c[0])
    two_pairs.sort(key=lambda c: c[0])
    pairs.sort(key=lambda c: c[0])
    highs.sort(key=lambda c: c[0])
    thing = fives + fours + full_h + threes + two_pairs + pairs + highs
    for shit, bet in thing:
        total += bet * size
        size -= 1
    print(total)
