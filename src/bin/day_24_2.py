from pathlib import Path
from sympy import symbols, Eq, solve, GreaterThan, LessThan, S


def load_data():
    data = []
    for line in Path("input/day_24.txt").read_text().strip().split("\n"):
        (p, v) = line.split(" @ ")
        print(p)
        p = tuple(int(pi) for pi in p.split(","))
        v = tuple(int(pi) for pi in v.split(","))
        data.append((p, v))
    print(data)
    return data


def part_two(lines):
    vars = symbols("x y z vx vy vz " + " ".join(f"t_{i}" for i in range(3)))
    eqs = []
    for i, ((x0, y0, z0), (vx0, vy0, vz0)) in enumerate(lines[:3]):
        formuli = [
            f"{x0} + {vx0} * t_{i} = x + vx * t_{i}",
            f"{y0} + {vy0} * t_{i} = y + vy * t_{i}",
            f"{z0} + {vz0} * t_{i} = z + vz * t_{i}",
        ]
        eqs += [Eq(*map(S, formula.split("=", 1))) for formula in formuli]

    roots = solve(eqs, vars)[0]
    solution = sum(roots[:3])
    print(solution)
    return solution



def main():
    data = load_data()
    part_two(data)


if __name__ == "__main__":
    main()
