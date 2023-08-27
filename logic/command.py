from .dice import roll_dice

from .character_stats import gen_char_stats
from .dice_roll_result import DiceRollResult
from .text_line import TextLine
from .text_type import TextType

import re


def process_command(raw_command: str) -> TextLine:
    cmd_str = raw_command.lstrip('/').lower()
    match cmd_str:
        case "roll":
            roll_cmd = cmd_str.lstrip("roll")
            match roll_cmd:
                case "dice":
                    return roll_dice_cmd(roll_cmd)
                case _: raise ValueError(f"Unknown roll command: {roll_cmd}")
        case "gen":
            gen_cmd = cmd_str.lstrip("gen")
            match gen_cmd:
                case "character stats":
                    return gen_char_stats(gen_cmd)
                case _: raise ValueError(f"Unknown gen command: {gen_cmd}")
        case _: raise ValueError(f"Unknown command: {raw_command}")


def roll_dice_cmd(roll_cmd: str) -> TextLine:
    result_struct = DiceRollResult()
    re1 = re.compile(r"dice\s*d66")
    re2 = re.compile(r"dice\s*(\d+)\s*d\s*(\d+)\s*([\+-/\*])\s*(\d+)")
    re3 = re.compile(r"dice\s*(\d+)\s*d\s*(\d+)")
    if re1.match(roll_cmd) is not None:
        result_struct = roll_dice(2, 6, 0, "+")
        result_txt = f"{result_struct.rolls[0]}{result_struct.rolls[1]} ({result_struct.rolls[0]},{result_struct.rolls[1]})"
        return TextLine(result_txt, TextType.COMMAND_OUTPUT)
    elif re2.match(roll_cmd) is not None:
        matches = re2.match(roll_cmd)
        num_dice = int(matches.group(1))
        die_size = int(matches.group(2))
        mod_sign = matches.group(3)
        modifier = int(matches.group(4))
        result_struct = roll_dice(num_dice, die_size, modifier, mod_sign)
        return TextLine(result_struct.__str__(), TextType.COMMAND_OUTPUT)
    elif re3.match(roll_cmd) is not None:
        matches = re3.match(roll_cmd)
        num_dice = int(matches.group(1))
        die_size = int(matches.group(2))
        result_struct = roll_dice(num_dice, die_size, 0, "+")
        return TextLine(result_struct.__str__(), TextType.COMMAND_OUTPUT)


def process_gen_command(raw_command: str) -> TextLine:
    if raw_command.__contains__("character stats"):
        stats = gen_char_stats()
        return TextLine(stats.__str__(), TextType.COMMAND_OUTPUT)

    return TextLine("Not implemented yet", TextType.COMMAND_OUTPUT)