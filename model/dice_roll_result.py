import reflex as rx


class DiceRollResult(rx.Model, table=True):
    num_dice: int
    die_size: int
    modifier: int
    mod_sign: str
    rolls: list[int]
    total: int

    def __init__(self, num_dice=0, die_size=0, modifier=0, mod_sign='+') -> None:
        self.num_dice = num_dice
        self.die_size = die_size
        self.modifier = modifier
        self.mod_sign = mod_sign
        self.rolls = []
        self.total = 0

    def __str__(self):
        return f"{self.num_dice}d{self.die_size}{self.mod_sign}{self.modifier} = {self.total} ({self.rolls})"