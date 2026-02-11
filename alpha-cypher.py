#!/usr/bin/env python3

import argparse
import logging
import operator
from functools import reduce
from itertools import combinations, permutations
from typing import Any, Callable, TypeIs

type Number = int | float
type BinaryOp = Callable[[Number, Number], Number]


def is_valid_numeric_core(number: Number) -> TypeIs[int]:
    try:
        val = float(number)
        return val > 0 and val % 1 == 0 and val < 100
    except (ValueError, TypeError):
        return False


def is_processable(number: Number) -> TypeIs[int]:
    try:
        val = float(number)
        return val > 0 and val % 1 == 0 and val >= 100
    except (ValueError, TypeError):
        return False


def word_to_nums(word: str) -> int:
    return int(
        "".join([str(ord(letter) - ord("A") + 1) for letter in word.strip().upper()])
    )


def nums_to_character(number: int) -> str:
    return chr(number + 64)


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

    ## Testing
    logging.debug(f"Split spots: {number} -> {all_split_spots}")

    resulting_groups: list[list[int]] = []
    # 12345
    # split_spots    = [ [0, 1, 2],     [0, 1, 3],     [0, 2, 3],     [1, 2, 3] ]
    # grouped_digits = [ [1, 2, 3, 45], [1, 2, 34, 5], [1, 23, 4, 5], [12, 3, 4, 5] ]
    for split_spots in all_split_spots:
        working_digits = digits.copy()

        ## insert split character in reverse order so not mess up subsequent inserts
        for split_spot in reversed(split_spots):
            working_digits.insert((split_spot + 1), "-")

        ## this is dumb but works and is readable
        grouped_digits: list[int] = [int(d) for d in "".join(working_digits).split("-")]

        resulting_groups.append(grouped_digits)

        ## TODO: do we need to check if the grouping of numbers has a length limit?
        ##       e.g. can we group "1234" as [1 234] ?
        # if not any([number >= 100 for number in grouped_digits]):
        #     resulting_groups.append(grouped_digits)

    return resulting_groups


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


# given a numberlogging.basicConfig(level=logging.INFO)
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

    digit_groups: list[list[int]] = split_number_into_groups(number)
    ## Testing
    logging.debug(f"Digit groups: {number} -> {digit_groups}")

    ## tuple just to help debug, previously was just the list of core int values
    current_cores: list[tuple[list[int], int]] = [
        (digit_group, digit_group_core)
        for digit_group in digit_groups
        if (digit_group_core := numeric_core_iteration(digit_group)) is not None
    ]
    ## Testing
    logging.debug(f"Cores per digit groups: {number} -> {current_cores}")

    result_core: int | None = (
        int(min([group_core_tuple[1] for group_core_tuple in current_cores]))
        if len(current_cores) > 0
        else None
    )

    ## Testing
    logging.debug(f"Final numeric core: {number} ->  {result_core}")
    return result_core


def cypher_to_string(data: list[Any]) -> str:  # pyright: ignore[reportExplicitAny]
    return "".join([f"{data[i : i + 5]}\n" for i in range(0, len(data), 5)])


def main() -> None:
    parser: argparse.ArgumentParser = argparse.ArgumentParser()
    _ = parser.add_argument(
        "--debug", action="store_true", type=bool, help="Enable debug logging"
    )
    args: argparse.Namespace = parser.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.debug else logging.INFO,  # pyright: ignore[reportAny]
        style="{",
        format="[{asctime}] {levelname:8} [{funcName} @ line {lineno}]\n{message}\n",
        datefmt="%Y-%m-%d %H:%M:%S",
    )

    cypher: list[str] = (
        [
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
        if not args.debug  # pyright: ignore[reportAny]
        else ["pigs"]
    )

    ## Initial print
    logging.info(f"Initial cypher: {cypher_to_string(data=cypher)}")

    ## Convert to large numbers
    cypher_as_large_nums: list[int] = cypher_to_nums(cypher=cypher)
    logging.info(f"Cypher as large numbers: {cypher_to_string(cypher_as_large_nums)}")

    ## Get numberic cores per large number
    cypher_as_cores: list[int] = [
        resulting_core
        for number in cypher_as_large_nums
        if (resulting_core := numeric_core(number) is not None)
    ]
    logging.info(f"Cypher as numeric cores: {cypher_to_string(cypher_as_cores)}")

    ## Get character for each numeric core
    cypher_as_characters: list[str] = [
        nums_to_character(number) for number in cypher_as_cores
    ]
    logging.info(f"Cypher as characters: {cypher_to_string(cypher_as_characters)}")


if __name__ == "__main__":
    main()
