def part_1():
    total = 0
    with open("data_1.txt") as fp:
        for line in fp.readlines():
            r1s, r2s = line.split(",")
            r1sl, r1su = r1s.split("-")
            r2sl, r2su = r2s.split("-")
            r1 = set(range(int(r1sl), int(r1su) + 1))
            r2 = set(range(int(r2sl), int(r2su) + 1))

            if r1.issubset(r2) or r2.issubset(r1):
                total += 1

    print(total)


def part_2():
    total = 0
    with open("data_1.txt") as fp:
        for line in fp.readlines():
            r1s, r2s = line.split(",")
            r1sl, r1su = r1s.split("-")
            r2sl, r2su = r2s.split("-")
            r1 = set(range(int(r1sl), int(r1su) + 1))
            r2 = set(range(int(r2sl), int(r2su) + 1))

            if r1.intersection(r2):
                total += 1

    print(total)


if __name__ == "__main__":
    part_1()
    part_2()
