def main():
    with open("../data.txt") as fp:
        lines = fp.readlines()

    elves = [0]
    for line in lines:
        if line.strip():
            elves[-1] += int(line)
        else:
            elves.append(0)

    indexed_elves = [(elf, total_calories) for elf, total_calories in enumerate(elves)]
    indexed_elves.sort(key=lambda e: e[1], reverse=True)

    sum_calories = 0
    for index, (elf, total_calories) in enumerate(indexed_elves[:3]):
        sum_calories += total_calories
        print(f"Elf {elf} is {index + 1} with {total_calories} calories")

    print(f"The top three elves are carrying {sum_calories} altogether")


if __name__ == "__main__":
    main()
