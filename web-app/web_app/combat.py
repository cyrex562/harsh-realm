import random
import re

from .dice import roll_dice

from .text_type import TextType
from .text_line import TextLine


def proc_attack_roll_cmd(cmd: str) -> TextLine:
    re1 = re.compile(
        r"to hit\s*bonus=\s*(\d+)\s*skill=\s*(\d+)\s*modifier=\s*(\d+)\s*target-ac=\s*(\d+)"
    )
    matches = re1.match(cmd)
    if matches is None:
        raise ValueError(f'unknown command "{cmd}"')
    attack_bonus = int(matches.group(1))
    skill = int(matches.group(2))
    modifier = int(matches.group(3))
    target_ac = int(matches.group(4))
    roll = roll_dice(1, 20, 0, "+")
    attack_score = roll + attack_bonus + skill + modifier
    if attack_score >= target_ac:
        result = "hit"
    else:
        result = "miss"
    return TextLine(
        f'atk bonus={attack_bonus}, skill={skill}, modifier={modifier}, roll={roll}, score={attack_score}, result="{result}"',
        TextType.ANSWER.value,
    )
