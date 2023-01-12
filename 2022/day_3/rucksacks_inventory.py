priorities = {
    chr(item): priority
    for priority, item in enumerate(range(ord("a"), ord("z") + 1), 1)
} | {
    chr(item): priority
    for priority, item in enumerate(range(ord("A"), ord("Z") + 1), 27)
}


def part_1():
    total = 0

    with open("data_1.txt") as fp:
        for line in fp.readlines():
            line = line.strip()
            n = int(len(line) / 2)
            c1 = set(line[:n])
            c2 = set(line[n:])
            dups = c1.intersection(c2)
            for item in dups:
                total += priorities[item]

    print(total)


def part_2():
    total = 0
    items = None
    index = 0

    with open("data_2.txt") as fp:
        for line in fp.readlines():
            line = line.strip()

            if items is None:
                items = set(line)
            else:
                items = items.intersection(line)

            index += 1
            if index == 3:
                assert len(items) == 1
                total += priorities[list(items)[0]]
                index = 0
                items = None

    print(total)


def f(items):
    assert len(items) == 1
    return priorities[list(items)[0]]


if __name__ == "__main__":
    part_1()
    part_2()
