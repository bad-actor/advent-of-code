from typing import Dict, List, TextIO

Crates = Dict[int, List[chr]]


def part_1():
    with open("data_1.txt") as fp:
        crates = read_crates(fp)
        for n, from_, to in read_commands(fp):
            move_crates_9000(crates, n, from_, to)

    print("".join([stack[-1] for stack in crates.values()]))


def part_2():
    with open("data_1.txt") as fp:
        crates = read_crates(fp)
        for n, from_, to in read_commands(fp):
            move_crates_9001(crates, n, from_, to)

    print("".join([stack[-1] for stack in crates.values()]))


def read_crates(fp: TextIO) -> Crates:
    crates: Crates = {}

    for line in fp:
        line = line.rstrip("\n")
        if line:
            for index, crate_index in enumerate(range(1, len(line), 4), 1):
                if index not in crates:
                    crates[index] = []

                crate = line[crate_index]
                if crate == " " or crate.isnumeric():
                    continue  # no crate in position

                crates[index].insert(0, crate)
        else:
            break

    return crates


def read_commands(fp: TextIO) -> None:
    for line in fp:
        command = line.strip().split(" ")
        n = int(command[1])
        from_ = int(command[3])
        to = int(command[5])
        yield n, from_, to


def move_crates_9000(crates: Crates, n: int, from_: int, to: int) -> None:
    for _ in range(n):
        crate = crates[from_].pop()
        crates[to].append(crate)


def move_crates_9001(crates: Crates, n: int, from_: int, to: int) -> None:
    crates_ = []
    for _ in range(n):
        crate = crates[from_].pop()
        crates_.append(crate)

    crates_.reverse()
    crates[to].extend(crates_)


if __name__ == "__main__":
    part_1()
    part_2()
