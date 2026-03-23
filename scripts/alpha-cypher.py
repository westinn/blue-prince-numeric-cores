#!/usr/bin/env python3

import argparse
import logging
import operator
from argparse import Namespace
from functools import reduce
from itertools import combinations, permutations
from typing import Any, Callable, Self, TypeIs

type Number = int | float
type BinaryOp = Callable[[Number, Number], Number]


class NumericCoreCalculator:
    def __init__(self: Self, cypher: list[str]) -> None:
        self.cypher: list[str] = cypher

    def set_cypher(self, cypher: list[str]) -> None:
        self.cypher = cypher

    @staticmethod
    def cypher_to_string(data: list[Any]) -> str:  # pyright: ignore[reportExplicitAny]
        return "".join([f"{data[i : i + 5]}\n" for i in range(0, len(data), 5)])

    @staticmethod
    def is_valid_numeric_core(number: Number) -> TypeIs[int]:
        try:
            val = float(number)
            return val > 0 and val % 1 == 0 and val < 1000
        except (ValueError, TypeError):
            return False

    @staticmethod
    def is_processable(number: Number) -> TypeIs[int]:
        try:
            val: float = float(number)
            return val > 0 and val % 1 == 0 and val >= 1000
        except (ValueError, TypeError):
            return False

    @staticmethod
    def word_to_digit_groups(word: str) -> list[int]:
        return [(ord(letter) - ord("A") + 1) for letter in word.strip().upper()]

    @staticmethod
    def cypher_to_nums(cypher: list[str]) -> list[list[int]]:
        return [
            NumericCoreCalculator.word_to_digit_groups(word) for word in cypher
        ]

    @staticmethod
    def nums_to_character(number: int) -> str:
        return chr(int(number) + 64)

    @staticmethod
    def split_number_into_groups(number: int) -> list[list[int]]:
        ## 12345
        ## [ [0, 1, 2],     [0, 1, 3],     [0, 2, 3],     [1, 2, 3] ]
        ## [ [1, 2, 3, 45], [1, 2, 34, 5], [1, 23, 4, 5], [12, 3, 4, 5] ]
        ## technically we can pass a float here and it "passes" the type check,
        ## but we actually need no decimals for length, even if 10.0 passes
        digits: list[str] = [d for d in str(int(number))]

        ## in terms of possible spots to split a number, there are "len(number) - 1" possible locationss
        ## in terms of how many splits we need to pick, there is "groups_needed - 1" amount, so 3 for us
        groups_needed = 4
        all_split_spots: list[list[int]] = [
            list[int](split_spots)
            for split_spots in combinations(
                range(len(str(int(number))) - 1), groups_needed - 1
            )
        ]

        logging.debug(f"Split spots: {number} -> {all_split_spots}")

        resulting_groups: list[list[int]] = []
        for split_spots in all_split_spots:
            working_digits = digits.copy()

            ## insert split character in reverse order so not mess up subsequent inserts
            for split_spot in reversed(split_spots):
                working_digits.insert((split_spot + 1), "-")

            ## this is dumb but works and is readable
            grouped_digits: list[int] = [
                int(d) for d in "".join(working_digits).split("-")
            ]
            resulting_groups.append(grouped_digits)
        return resulting_groups

    @staticmethod
    def numeric_core_iteration(digit_group: list[int]) -> int | None:
        ops: list[BinaryOp] = [
            operator.add,
            operator.sub,
            operator.mul,
            operator.truediv,
        ]

        ## every combo without add, with add at the beginning
        op_combos: list[list[BinaryOp]] = [
            op_combo
            for no_add_op_combo in permutations(ops[1:])
            if (operator.truediv, 0)
            not in zip(op_combo := ops[:1] + list(no_add_op_combo), digit_group)
        ]

        def apply_ops(
            result: Number, zip_op_digit: tuple[BinaryOp, Number]
        ) -> Number:
            return zip_op_digit[0](result, zip_op_digit[1])

        ## for every combination of operation, get the resulting core with reduce
        op_combo_cores: list[int] = [
            potential_core
            for op_combo in op_combos
            if (
                potential_core := NumericCoreCalculator.numeric_core(
                    number=reduce(
                        apply_ops, zip(op_combo, digit_group), initial=0
                    )
                )
            )
            is not None
        ]
        return min(op_combo_cores) if len(op_combo_cores) > 0 else None

    @staticmethod
    def numeric_core(number: Number) -> int | None:
        if NumericCoreCalculator.is_valid_numeric_core(number):
            return number
        if not NumericCoreCalculator.is_processable(number):
            return None

        ## not a core yet but still processable
        digit_groups: list[list[int]] = (
            NumericCoreCalculator.split_number_into_groups(number)
        )
        logging.debug(f"Digit groups: {number} -> {digit_groups}")

        ## tuple just to help debug, previously was just the list of core int values
        current_cores: list[tuple[list[int], int]] = [
            (digit_group, digit_group_core)
            for digit_group in digit_groups
            if (
                digit_group_core
                := NumericCoreCalculator.numeric_core_iteration(digit_group)
            )
            is not None
        ]
        logging.debug(f"Cores per digit groups: {number} -> {current_cores}")

        result_core: int | None = (
            int(min([tuple_dg_dgcore[1] for tuple_dg_dgcore in current_cores]))
            if len(current_cores) > 0
            else None
        )

        logging.debug(f"Final numeric core: {number} ->  {result_core}")
        return result_core

    def solve_cypher(self) -> None:
        ## Initial print
        logging.info(
            msg=f"Initial cypher: \n{self.cypher_to_string(data=self.cypher)}"
        )

        ## Convert initial cypher into digit groups per given letters
        cypher_as_digit_groups: list[list[int]] = [
            self.word_to_digit_groups(word) for word in self.cypher
        ]
        logging.info(
            msg=f"Cypher as digit groups: \n{self.cypher_to_string(data=cypher_as_digit_groups)}"
        )

        ## Get numeric cores per digit group
        ## will recurse into larger numeric_core function that handles further splitting
        # cypher_as_cores2: list[int] = []
        # for digit_group in cypher_as_digit_groups:
        #     resulting_core = self.numeric_core_iteration(
        #         digit_group=digit_group
        #     )
        #     if resulting_core is not None:
        #         cypher_as_cores.append(resulting_core)

        cypher_as_cores: list[int] = [
            resulting_core
            for digit_group in cypher_as_digit_groups
            if (
                resulting_core := self.numeric_core_iteration(
                    digit_group=digit_group
                )
            )
            is not None
        ]
        logging.info(
            msg=f"Cypher as numeric cores: \n{self.cypher_to_string(cypher_as_cores)}"
        )

        ## Get character for each numeric core
        cypher_as_characters: list[str] = [
            self.nums_to_character(number) for number in cypher_as_cores
        ]
        logging.info(
            msg=f"Cypher as characters: \n{self.cypher_to_string(cypher_as_characters)}"
        )

        ## Cypher as final strings
        cypher_result: str = "".join(
            [
                f"{''.join(cypher_as_characters[i : i + 5])}\n"
                for i in range(0, len(cypher_as_characters), 5)
            ]
        )
        logging.info(msg=f"Cypher result: \n\n{cypher_result}")


def main() -> None:
    def setup_args() -> Namespace:
        parser: argparse.ArgumentParser = argparse.ArgumentParser()
        _ = parser.add_argument(
            "--debug", action="store_true", help="Enable debug logging"
        )
        args: argparse.Namespace = parser.parse_args()
        return args

    args: Namespace = setup_args()
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
        else ["pigs", "sand"]
    )

    cypher_calc: NumericCoreCalculator = NumericCoreCalculator(cypher=cypher)
    cypher_calc.solve_cypher()


if __name__ == "__main__":
    main()
