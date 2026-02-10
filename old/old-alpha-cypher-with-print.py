#!/usr/bin/env python3

import itertools
import operator
from functools import reduce
from typing import Any
from pprint import pprint

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


## A = 1, B = 2,
## returns each character as a number, then combined as a string
def word_to_nums(word: str) -> int:
    return int(
        "".join([str(ord(letter) - ord("A") + 1) for letter in word.strip().upper()])
    )


def cypher_to_nums(cypher: list[str]) -> list[int]:
    ## for each word, we want to convert each letter to a number
    ## that will result in a 4 digit number that has a numeric core
    return [word_to_nums(word) for word in cypher]


## split number into 4
## assign first as +, and rest as * - or /
## smallest whole positive number is the result
## repeat until result is 3 digits or less in length


## 86455
## [[8, 6, 4, 55], [8, 6, 45, 5], [8, 64, 5, 5], [86, 4, 5, 5]]
def split_number_into_groups(number: int) -> list[list[int]]:
    # print(number)
    groups_needed = 4

    resulting_groups: list[list[int]] = []

    ## in terms of possible spots to split a number, there are "len(number) - 1" possible locationss
    ## in terms of how many splits we need to pick, there is "groups_needed - 1" amount, so 3 for us

    ## generate all possible split spots
    for curr_set_split_spots in itertools.combinations(
        range(len(str(number)) - 1), groups_needed - 1
    ):
        digits: list[str] = [d for d in str(number)]

        ## insert split character in reverse order so not mess up subsequent inserts
        for split_spot in reversed(curr_set_split_spots):
            digits.insert((split_spot + 1), "-")

        ## ...the following is silly but easy
        ## join the string digits, split them again by the '-' we just added (thus making the groups), and convert to int
        grouped_digits: list[int] = [int(d) for d in "".join(digits).split("-")]
        resulting_groups.append(grouped_digits)

    return resulting_groups


## takes in a list of int that represent a set of digits split into 4 groups
## this is one of many splits that a larger number could have, but this is one way to split it
## this calculates the numeric core possibilites for a specific split,
##  by looping through possible operation orders
def numeric_core_iteration(
    digit_group: list[int],
) -> dict[int, list[tuple[str, str]]] | None:
    ops_lookup: dict[Any, str] = {
        operator.add: "+",
        operator.sub: "-",
        operator.mul: "*",
        operator.truediv: "/",
    }
    ops: list[Any] = list(ops_lookup.keys())

    ## every combo without add, with add at the beginning
    op_combos: list[list[str]] = [
        ops[:1] + list(op) for op in itertools.permutations(ops[1:])
    ]

    ## for a single op_combo, apply the ops in order
    ## use enumerate on the op_combo to get curr_index so we can apply to matching index on digits
    def apply_ops(accumulated: int, curr_item: tuple[int, Any]) -> int:
        index: int = curr_item[0]
        curr_op: Any = curr_item[1]
        next_digit = digit_group[index]
        ## catch divide by 0 here, but caught in loop below
        return curr_op(accumulated, next_digit)

    pretty_results: dict[int, list[tuple[str, str]]] = {}

    ## for every combination of operation,
    ## get the resulting core with reduce
    ## and print the matching operation/digit combination in order
    for op_combo in op_combos:
        ## catch divide by zero before we start
        ## if we find one, then this op_combo with the current digit group is not valid
        if any(
            [
                op is operator.truediv and digit_group[i] == 0
                for i, op in enumerate(op_combo)
            ]
        ):
            continue

        reduced_core: int = reduce(apply_ops, enumerate(op_combo), 0)

        ## if current operation combo results in negative or decimals, then it is not valid
        ## @TODO does it matter that the core number can be 0? I say it cannot here.
        if reduced_core <= 0 or reduced_core % 1 != 0:
            continue

        ## for every digit in the digit_group
        ##  use the op lookup dict to pull the pretty symbol
        ##  by passing as input: current_op_combo's current op via index + digit it operated on
        ## these digit_group + op_combo variables have matching lengths so this always works
        ## we also print the first op first since the first op/digit is always +

        ## "+ - * /"
        ops_str = " ".join([ops_lookup[op] for op in op_combo])
        ## "+ 8 - 6 / 4 * 55"
        math_str: str = " ".join(
            [f"{ops_lookup[op_combo[i]]} {digit}" for i, digit in enumerate(digit_group)]
        )
        if reduced_core in pretty_results:
            pretty_results[reduced_core].append((ops_str, math_str))
        else:
            pretty_results[reduced_core] = [(ops_str, math_str)]

    if len(pretty_results) <= 0:
        return None
    else:
        return pretty_results


def numeric_core(number: int) -> dict[str, dict[int, list[tuple[str, str]]]]:
    print("")
    print(number)
    ## if the length of digits is 3 or less and the value is positive, we're done
    # if number > 0 and len(str(number)) < 4:
    #     return
    result: dict[str, dict[int, list[tuple[str, str]]]] = {}

    ## 86455
    ## [[8, 6, 4, 55], [8, 6, 45, 5], [8, 64, 5, 5], [86, 4, 5, 5]]
    digit_groups: list[list[int]] = split_number_into_groups(number)
    # print(digit_groups)

    for digit_group in digit_groups:
        digit_group_result: dict[int, list[tuple[str, str]]] | None = (
            numeric_core_iteration(digit_group)
        )
        if digit_group_result is None:
            continue

        digit_group_str: str = " ".join(str(d) for d in digit_group)
        # print(f"test: {digit_group_str}")

        result[digit_group_str] = digit_group_result

    return result


def print_cypher(data) -> None:
    for i in range(0, len(data), 5):
        print(data[i : i + 5])


def test():
    print("Testing:")
    curr_core: dict[str, dict[int, list[tuple[str, str]]]] = numeric_core(201514)
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
    for number in cypher_as_nums:
        curr_core: dict[str, dict[int, list[tuple[str, str]]]] = numeric_core(number)
        pprint(curr_core)


if __name__ == "__main__":
    main()
