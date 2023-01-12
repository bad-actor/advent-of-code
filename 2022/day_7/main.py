def part_1():
    total = 0
    dirs = {}
    with open("test_data_1.txt") as fp:
        for line in fp.readlines():
            if line[0] == "$":
                _, cmd, *args = line.strip().split()

                if cmd == "cd":
                    
            else:
                results = line.strip().split()
                if results[0] == "dir":
                    dirs[results[1]] = {}
                else:
                    
    print(total)


def part_2():
    total = 0
    with open("test_data_2.txt") as fp:
        for line in fp.readlines():
            pass

    print(total)


if __name__ == "__main__":
    part_1()
    part_2()
