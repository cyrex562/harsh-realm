from .dice_roll_result import DiceRollResult


import random


def roll_dice(num_dice: int, die_size: int, modifier: int, mod_sign: str) -> DiceRollResult:
    result = DiceRollResult()
    result.num_dice = num_dice
    result.die_size = die_size
    result.modifier = modifier

    for i in range(num_dice):
        roll = random.randint(1,die_size)
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