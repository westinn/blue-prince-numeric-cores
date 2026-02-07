#!/usr/bin/env python3

import itertools
import operator
from functools import reduce
from typing import Callable, TypeIs
from pprint import pprint

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


## A = 1, B = 2,
## returns each character as a number, then combined as a string
def word_to_nums(word: str) -> int:
    return int(
        "".join([str(ord(letter) - ord("A") + 1) for letter in word.strip().upper()])
    )


def cypher_to_nums(cypher: list[str]) -> list[int]:
    return [word_to_nums(word) for word in cypher]


## 86455
## [[8, 6, 4, 55], [8, 6, 45, 5], [8, 64, 5, 5], [86, 4, 5, 5]]
def split_number_into_groups(number: int) -> list[list[int]]:
    int_number = int(number)
    groups_needed = 4

    resulting_groups: list[list[int]] = []

    ## in terms of possible spots to split a number, there are "len(number) - 1" possible locationss
    ## in terms of how many splits we need to pick, there is "groups_needed - 1" amount, so 3 for us

    ## generate all possible split spots
    for curr_set_split_spots in itertools.combinations(
        range(len(str(int_number)) - 1), groups_needed - 1
    ):
        digits: list[str] = [d for d in str(int_number)]

        ## insert split character in reverse order so not mess up subsequent inserts
        for split_spot in reversed(curr_set_split_spots):
            digits.insert((split_spot + 1), "-")

        ## ...the following is silly but easy
        ## join the string digits, split them again by the '-' we just added (thus making the groups), and convert to int
        grouped_digits: list[int] = [int(d) for d in "".join(digits).split("-")]
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
    ops_lookup: dict[BinaryOp, str] = {
        operator.add: "+",
        operator.sub: "-",
        operator.mul: "*",
        operator.truediv: "/",
    }
    ops: list[BinaryOp] = list(ops_lookup.keys())

    ## every combo without add, with add at the beginning
    op_combos: list[list[BinaryOp]] = [
        ops[:1] + list(op) for op in itertools.permutations(ops[1:])
    ]

    ## if there is a op_combo with a division and 0 in the same index, filter it out
    ## no more divide by 0
    valid_op_combos: list[list[BinaryOp]] = [
        op_combo
        for op_combo in op_combos
        if not any(
            op is operator.truediv and digit == 0
            for op, digit in zip(op_combo, digit_group)
        )
    ]

    ## for a single op_combo, apply the ops in order
    ## use enumerate on the op_combo to get curr_index so we can apply to matching index on digits
    def apply_ops(
        accumulated: Number, zipped_op_digit: tuple[BinaryOp, Number]
    ) -> Number:
        return zipped_op_digit[0](accumulated, zipped_op_digit[1])

    ## for every combination of operation,
    ## get the resulting core with reduce
    op_combo_cores: list[int] = [
        potential_core
        for op_combo in valid_op_combos
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

    ## 86455
    ## [[8, 6, 4, 55], [8, 6, 45, 5], [8, 64, 5, 5], [86, 4, 5, 5]]
    digit_groups: list[list[int]] = split_number_into_groups(number)
    # print(digit_groups)

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
    print("Testing:")
    curr_core = numeric_core(201514)
    print(curr_core)


def main() -> None:
    print("Initial cypher:")
    print_cypher(data=cypher)
    print("")

    cypher_as_nums: list[int] = cypher_to_nums(cypher=cypher)
    print("Cypher after character to numbers:")
    print_cypher(data=cypher_as_nums)
    print("")

    ## somehow skipping some 0's?
    ## running into multiple integer values
    # cypher_as_cores: list[int | None] = [numeric_core(number) for number in cypher_as_nums]
    # pprint(cypher_as_cores)

    test()


if __name__ == "__main__":
    main()

def test2():
    321456
    [ [3, 2, 1, 456], ... ]
    -> [ [ +, -, *, / ], ... ]
    

# given a number
# find all possible ways to split into 4 groups
# for every group,
#   run every operation order for every group
#   for every operation result:
#       qsd
#       if it is not a valid core number, check if it is processable
#       if processable, num recursion
#       else no core number
# once you have the core number from each group, return the smallest