# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "z3-solver",
# ]
# ///
from z3 import *

def main() -> None:
    offset = 0

    e = [2,4,1,7,7,5,0,3,1,7,4,1,5,5,3,0]
    a = BitVec("a", len(e)*3)
    constraints = []

    for k in e:
        a_ = a >> offset
        n = a_ & 7
        p = (a_ >> (n ^ 7)) & 7
        constraints.append(n ^ p == k)
        offset += 3

    print(solve(constraints))


if __name__ == "__main__":
    main()
