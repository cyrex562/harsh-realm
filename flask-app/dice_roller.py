from random import randint
from typing import List


def roll_dice(num_sides: int, num_dice: int) -> List[int]:
    out_list = []
    for n in range(1,num_dice + 1, 1):
        roll_result = randint(1,num_sides)
        out_list.append(roll_result)
    return out_list
