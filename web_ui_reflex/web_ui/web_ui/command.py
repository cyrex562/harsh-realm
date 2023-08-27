
from .dice import roll_dice

from .character_stats import gen_character_stats
from .dice_roll_result import DiceRollResult
from .text_line import TextLine
from .text_type import TextType


import re


def process_roll_command(raw_command: str) -> TextLine:
    result_struct = DiceRollResult()
    re1 = re.compile(r"roll\s*d66")
    re2 = re.compile(r"roll\s*(\d+)\s*d\s*(\d+)\s*([\+-/\*])\s*(\d+)")
    re3 = re.compile(r"roll\s*(\d+)\s*d\s*(\d+)")
    if re1.match(raw_command) is not None:
        result_struct = roll_dice(2,6,0,"+")
        result_txt = f"{result_struct.rolls[0]}{result_struct.rolls[1]} ({result_struct.rolls[0]},{result_struct.rolls[1]})"
        return TextLine(result_txt,TextType.COMMAND_OUTPUT)
    elif re2.match(raw_command) is not None:
        matches = re2.match(raw_command)
        num_dice = int(matches.group(1))
        die_size = int(matches.group(2))
        mod_sign = matches.group(3)
        modifier = int(matches.group(4))
        result_struct = roll_dice(num_dice,die_size,modifier,mod_sign)
        return TextLine(result_struct.__str__(),TextType.COMMAND_OUTPUT)
    elif re3.match(raw_command) is not None:
        matches = re3.match(raw_command)
        num_dice = int(matches.group(1))
        die_size = int(matches.group(2))
        result_struct = roll_dice(num_dice,die_size,0,"+")
        return TextLine(result_struct.__str__(),TextType.COMMAND_OUTPUT)
    
def process_gen_command(raw_command: str) -> TextLine:
    if raw_command.__contains__("character stats"):
        stats = gen_character_stats()
        return TextLine(stats.__str__(),TextType.COMMAND_OUTPUT)
    
    
    return TextLine("Not implemented yet",TextType.COMMAND_OUTPUT)