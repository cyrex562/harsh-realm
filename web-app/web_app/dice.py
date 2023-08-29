from .text_line import TextLine
from .text_type import TextType
import reflex as rx

import re
import random


class DiceRollResult(rx.Model, table=True):
    num_dice: int
    die_size: int
    modifier: int
    mod_sign: str
    rolls: list[int]
    total: int

    def __init__(self, num_dice=0, die_size=0, modifier=0, mod_sign="+") -> None:
        self.num_dice = num_dice
        self.die_size = die_size
        self.modifier = modifier
        self.mod_sign = mod_sign
        self.rolls = []
        self.total = 0

    def __str__(self):
        return f"{self.num_dice}d{self.die_size}{self.mod_sign}{self.modifier} = {self.total} ({self.rolls})"


def roll_dice(
    num_dice: int, die_size: int, modifier: int, mod_sign: str
) -> DiceRollResult:
    result = DiceRollResult()
    result.num_dice = num_dice
    result.die_size = die_size
    result.modifier = modifier

    for i in range(num_dice):
        roll = random.randint(1, die_size)
        result.rolls.append(roll)
        result.total += roll

    result.mod_sign = mod_sign
    if mod_sign == "-":
        result.total -= modifier
    elif mod_sign == "*":
        result.total *= modifier
    elif mod_sign == "/":
        result.total /= modifier
    else:
        result.total += modifier
    return result


def proc_roll_dice_cmd(cmd: str) -> TextLine:
    print(f'proc_roll_dice_cmd: "{cmd}"')
    result_struct = DiceRollResult()
    re1 = re.compile(r"dice\s*d66")
    re2 = re.compile(r"dice\s*(\d+)\s*d\s*(\d+)\s*([\+-/\*])\s*(\d+)")
    re3 = re.compile(r"dice\s*(\d+)\s*d\s*(\d+)")
    if re1.match(cmd) is not None:
        result_struct = roll_dice(2, 6, 0, "+")
        result_txt = f"{result_struct.rolls[0]}{result_struct.rolls[1]} ({result_struct.rolls[0]},{result_struct.rolls[1]})"
        return TextLine(result_txt, TextType.ANSWER.value)
    elif re2.match(cmd) is not None:
        matches = re2.match(cmd)
        num_dice = int(matches.group(1))
        die_size = int(matches.group(2))
        mod_sign = matches.group(3)
        modifier = int(matches.group(4))
        result_struct = roll_dice(num_dice, die_size, modifier, mod_sign)
        return TextLine(result_struct.__str__(), TextType.ANSWER.value)
    elif re3.match(cmd) is not None:
        matches = re3.match(cmd)
        num_dice = int(matches.group(1))
        die_size = int(matches.group(2))
        result_struct = roll_dice(num_dice, die_size, 0, "+")
        return TextLine(result_struct.__str__(), TextType.ANSWER.value)
