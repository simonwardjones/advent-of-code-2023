from pathlib import Path


def part_one():
    return sum(
        int(f"{a[0]}{a[-1]}")
        for line in Path("input/day_01.txt").read_text().strip().split("\n")
        if (a := [value for value in line if value.isdigit()])
    )


def part_two():
    numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    lines = []
    for line in Path("input/day_01.txt").read_text().strip().split("\n"):
        line_values = []
        for i, char in enumerate(line):
            if char.isdigit():
                line_values.append(char)
            for j, number in enumerate(numbers):
                if line[i:].startswith(number):
                    line_values.append(str(j + 1))
        lines.append(line_values)
    return sum([int(f"{line[0]}{line[-1]}") for line in lines])


def main():
    one = part_one()
    print(f"Part One: {one}")
    two = part_two()
    print(f"Part Two: {two}")


if __name__ == "__main__":
    main()
