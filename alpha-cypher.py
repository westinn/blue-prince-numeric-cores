#!/usr/bin/env python3

import operator
from functools import reduce
from itertools import combinations, permutations
from pprint import pprint
from typing import Callable, TypeIs

type Number = int | float
type BinaryOp = Callable[[Number, Number], Number]

cypher: list[str] = [
    "pigs",
    "sand",
    "mail",
    "date",
    "head",
    "clam",
    "peak",
    "sand",
    "joya",
    "well",
    "toad",
    "card",
    "will",
    "tape",
    "legs",
    "tree",
    "road",
    "maid",
    "slab",
    "rock",
    "hand",
    "vase",
    "safe",
    "clay",
    "toes",
]


def is_valid_numeric_core(number: Number) -> TypeIs[int]:
    try:
        val = float(number)
        return val > 0 and val % 1 == 0 and len(str(int(val))) <= 3
    except (ValueError, TypeError):
        return False


def is_processable(number: Number) -> TypeIs[int]:
    try:
        val = float(number)
        return val > 0 and val % 1 == 0 and len(str(int(val))) > 3
    except (ValueError, TypeError):
        return False


def word_to_nums(word: str) -> int:
    return int(
        "".join([str(ord(letter) - ord("A") + 1) for letter in word.strip().upper()])
    )


def cypher_to_nums(cypher: list[str]) -> list[int]:
    return [word_to_nums(word) for word in cypher]


## 12345
## [ [0, 1, 2],     [0, 1, 3],     [0, 2, 3],     [1, 2, 3] ]
## [ [1, 2, 3, 45], [1, 2, 34, 5], [1, 23, 4, 5], [12, 3, 4, 5] ]
def split_number_into_groups(number: int) -> list[list[int]]:
    ## technically we can pass a float here and it passes,
    ## but we actually need no decimals for length, even if 10.0 passes
    int_number = int(number)
    digits: list[str] = [d for d in str(int_number)]

    ## in terms of possible spots to split a number, there are "len(number) - 1" possible locationss
    ## in terms of how many splits we need to pick, there is "groups_needed - 1" amount, so 3 for us
    groups_needed = 4
    all_split_spots: list[list[int]] = [
        list(split_spots)
        for split_spots in combinations(
            range(len(str(int_number)) - 1), groups_needed - 1
        )
    ]
    print(all_split_spots)

    resulting_groups: list[list[int]] = []

    # 12345
    # split_spots    = [ [0, 1, 2],     [0, 1, 3],     [0, 2, 3],     [1, 2, 3] ]
    # grouped_digits = [ [1, 2, 3, 45], [1, 2, 34, 5], [1, 23, 4, 5], [12, 3, 4, 5] ]
    for split_spots in all_split_spots:
        working_digits = digits.copy()
        # indexes        = [01234]
        # digits         = [12345]
        # split_spots    = [0, 2, 3]
        # grouped_digits = [1, 23, 4, 5]
        ## so we want to split after 0, so at indexes: [0:1], [1:3], [3:4], [4:]
        #
        # better_grouped_digits = [
        #     digits[curr_split_spot : split_spots[ind + 1]]
        #     for ind, curr_split_spot in enumerate(split_spots)
        # ]
        # print(better_grouped_digits)

        ## insert split character in reverse order so not mess up subsequent inserts
        for split_spot in reversed(split_spots):
            working_digits.insert((split_spot + 1), "-")

        grouped_digits: list[int] = [int(d) for d in "".join(working_digits).split("-")]
        resulting_groups.append(grouped_digits)

    return resulting_groups


## for every group,
##   # <<< this is iteration func >>>
##   run every operation order for every group
##      every time an operation per group finishes,
#    check if it is a valid core number
##   if it is not a valid core number, check if it is processable
##   if processable, num recursion
##   else no core number
def numeric_core_iteration(digit_group: list[int]) -> int | None:
    ops: list[BinaryOp] = [operator.add, operator.sub, operator.mul, operator.truediv]

    ## every combo without add, with add at the beginning
    op_combos: list[list[BinaryOp]] = [
        op_combo
        for no_add_op_combo in permutations(ops[1:])
        if (operator.truediv, 0)
        not in zip(op_combo := ops[:1] + list(no_add_op_combo), digit_group)
    ]

    def apply_ops(result: Number, zipped_op_digit: tuple[BinaryOp, Number]) -> Number:
        return zipped_op_digit[0](result, zipped_op_digit[1])

    ## for every combination of operation, get the resulting core with reduce
    op_combo_cores: list[int] = [
        potential_core
        for op_combo in op_combos
        if (
            potential_core := numeric_core(
                reduce(apply_ops, zip(op_combo, digit_group), 0)
            )
        )
        is not None
    ]

    return min(op_combo_cores) if len(op_combo_cores) > 0 else None


# given a number
# find all possible ways to split into 4 groups
# for every group,
#   run every operation order for every group
#   every time an operation per group finishes, check if it is a valid core number
#   if it is not a valid core number, check if it is processable
#   if processable, num recursion
#   else no core number
# once you have the core number from each group, return the smallest
def numeric_core(number: Number) -> int | None:
    if is_valid_numeric_core(number):
        return number
    if not is_processable(number):
        return None

    ## not a core yet but still processable
    print("")
    print(number)

    ## 201514
    ## [[8, 6, 4, 55], [8, 6, 45, 5], [8, 64, 5, 5], [86, 4, 5, 5]]
    digit_groups: list[list[int]] = split_number_into_groups(number)
    print(digit_groups)

    current_cores: list[int] = [
        digit_group_core
        for digit_group in digit_groups
        if (digit_group_core := numeric_core_iteration(digit_group)) is not None
    ]
    return min(current_cores) if len(current_cores) > 0 else None


def print_cypher(data) -> None:
    for i in range(0, len(data), 5):
        print(data[i : i + 5])


def test():
    # 12345
    # [ [0, 1, 2],     [0, 1, 3],     [0, 2, 3],     [1, 2, 3] ]
    # [ [1, 2, 3, 45], [1, 2, 34, 5], [1, 23, 4, 5], [12, 3, 4, 5] ]
    # 4.0
    print("Testing:")
    curr_core = numeric_core(12345)
    print(curr_core)


def main() -> None:
    print("Initial cypher:")
    print_cypher(data=cypher)
    print("")

    cypher_as_nums: list[int] = cypher_to_nums(cypher=cypher)
    print("Cypher after character to numbers:")
    print_cypher(data=cypher_as_nums)
    print("")

    # cypher_as_cores: list[int | None] = [numeric_core(number) for number in cypher_as_nums]
    # pprint(cypher_as_cores)

    test()


if __name__ == "__main__":
    main()
