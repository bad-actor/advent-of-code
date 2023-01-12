points_map = {"A": 1, "B": 2, "C": 3, "X": 1, "Y": 2, "Z": 3}


def part_1():
    win_points = {
        ("A", "X"): 3,
        ("A", "Y"): 6,
        ("A", "Z"): 0,
        ("B", "X"): 0,
        ("B", "Y"): 3,
        ("B", "Z"): 6,
        ("C", "X"): 6,
        ("C", "Y"): 0,
        ("C", "Z"): 3,
    }
    points = 0

    with open("data.txt") as fp:
        for line in fp.readlines():
            o, r = line.strip().split(" ")

            points += points_map[r] + win_points[(o, r)]

    print(points)


def part_2():
    win_points = {"X": 0, "Y": 3, "Z": 6}
    response_points = {
        ("A", "X"): 3,
        ("A", "Y"): 1,
        ("A", "Z"): 2,
        ("B", "X"): 1,
        ("B", "Y"): 2,
        ("B", "Z"): 3,
        ("C", "X"): 2,
        ("C", "Y"): 3,
        ("C", "Z"): 1,
    }
    points = 0

    with open("data.txt") as fp:
        for line in fp.readlines():
            o, w = line.strip().split(" ")

            points += response_points[(o, w)] + win_points[w]

    print(points)


if __name__ == "__main__":
    part_1()
    part_2()
