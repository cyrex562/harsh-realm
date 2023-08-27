from typing import Tuple
from .dice import proc_roll_dice_cmd


def calc_stat_dm(stat: int) -> int:
    if stat == 0:
        return -3
    elif stat >= 1 and stat <= 2:
        return -2
    elif stat >= 3 and stat <= 5:
        return -1
    elif stat >= 6 and stat <= 8:
        return 0
    elif stat >= 9 and stat <= 11:
        return 1
    elif stat >= 12 and stat <= 14:
        return 2
    else:
        return 3


BASE_HEIGHT_TABLE = [
    0,
    140,
    145,
    150,
    155,
    160,
    165,
    170,
    175,
    180,
    185,
    190,
    195,
    200,
    205,
    210,
]

HEIGHT_MOD_TABLE = [
    0,
    0,
    0,
    -10,
    -8,
    -6,
    -4,
    -2,
    -1,
    0,
    0,
    0,
    0,
    1,
    2,
    4,
    6,
    8,
    10,
]


def calc_height(strength: int) -> int:
    base_height = BASE_HEIGHT_TABLE[strength]
    height_mod_roll = proc_roll_dice_cmd(3, 6, 0, "+")
    height_mod = HEIGHT_MOD_TABLE[height_mod_roll.total]
    return base_height + height_mod


BASE_MASS_TABLE = [
    36,
    42,
    48,
    54,
    60,
    66,
    72,
    78,
    84,
    90,
    96,
    102,
    108,
    114,
    120,
    126,
]
MASS_DEX_MOD_TABLE = [
    0,
    12,
    10,
    8,
    6,
    4,
    2,
    0,
    0,
    0,
    -2,
    -4,
    -6,
    -8,
    -10,
    -12,
    0,
    0,
    0,
]

MASS_MOD_TABLE = [
    0,
    0,
    0,
    -12,
    -10,
    -8,
    -6,
    -4,
    -2,
    -1,
    0,
    0,
    1,
    2,
    4,
    6,
    8,
    10,
    12,
]


def calc_mass(end: int, dex: int) -> int:
    base_mass = BASE_MASS_TABLE[end]
    mass_dex_mod = MASS_DEX_MOD_TABLE[dex]
    mass_mod_roll = proc_roll_dice_cmd(3, 6, 3, "-")
    mass_mod = MASS_MOD_TABLE[mass_mod_roll.total]
    return base_mass + mass_dex_mod + mass_mod


SKIN_TONE_TABLE = [
    "",
    "pale white",
    "white",
    "tanned",
    "olive",
    "brown",
    "dark brown",
]

EYE_COLOR_TABLE = ["brown", "blue", "green", "hazel", "gray"]


def gen_skin_tone() -> str:
    skin_tone_roll = proc_roll_dice_cmd(1, 6, 0, "+")
    return SKIN_TONE_TABLE[skin_tone_roll.total]


def gen_hair_color() -> str:
    hair_color_roll = proc_roll_dice_cmd(2, 6, 0, "+").total
    if hair_color_roll >= 2 and hair_color_roll <= 5:
        return "black"
    elif hair_color_roll >= 6 and hair_color_roll <= 8:
        return "brown"
    else:
        return "blond"


def gen_eye_color(hair_color: str) -> str:
    eye_color_roll = proc_roll_dice_cmd(2, 6, 0, "+").total
    if eye_color_roll >= 2 and eye_color_roll <= 4:
        if hair_color == "black":
            return "brown"
        elif hair_color == "brown":
            return "brown"
        else:
            return "blue"
    elif eye_color_roll >= 5 and eye_color_roll <= 6:
        if hair_color == "black":
            return "blue"
        elif hair_color == "brown":
            return "green"
        else:
            return "green"
    elif eye_color_roll >= 7 and eye_color_roll <= 8:
        if hair_color == "black":
            return "green"
        elif hair_color == "brown":
            return "blue"
        else:
            return "hazel"
    elif eye_color_roll >= 9 and eye_color_roll <= 10:
        if hair_color == "black":
            return "hazel"
        elif hair_color == "brown":
            return "hazel"
        else:
            return "brown"
    else:
        gray_roll = proc_roll_dice_cmd(2, 6, 0, "+")
        if gray_roll in range(2, 9):
            left_roll = proc_roll_dice_cmd(1, 5, 0, "+").total
            left_color = EYE_COLOR_TABLE[left_roll]
            right_roll = proc_roll_dice_cmd(1, 5, 0, "+").total
            right_color = EYE_COLOR_TABLE[right_roll]
            return f"{left_color}/{right_color}"
        elif gray_roll in range(10, 11):
            return "red"
        elif gray_roll == 12:
            return "violet"
        else:
            return "gray"


def gen_soc_standing(soc: int) -> Tuple[str, str]:
    if soc == 0:
        return (
            "No social class",
            "fugitive wanted for exceptionally repugnant crimes; no contact with civilization",
        )
    elif soc == 1:
        return ("lower class", "exile, outcast")
    elif soc == 2:
        return ("lower class", "very poor")
    elif soc == 3:
        return ("lower class", "very low status; deprived background")
    elif soc == 4:
        return ("lower class", "unskilled laborer; poor")
    elif soc == 5:
        return ("lower class", "working or lower class; semi-skilled")
    elif soc == 6:
        return ("middle class", "skilled worker")
    elif soc == 7:
        return ("middle class", "middle class")
    elif soc == 8:
        return ("middle class", "upper edge of middle class")
    elif soc == 9:
        return (
            "middle class",
            "educated or respected professional/executive; a distant relative of a noble family",
        )
    elif soc == 10:
        return ("middle class", "untitled or close member of a noble family")
    elif soc == 11:
        return ("nobility", "knight, knightess, dame")
    elif soc == 12:
        return ("nobility", "baron, baronet, baroness")
    elif soc == 13:
        return ("nobility", "marquis, marquessa, marchioness")
    elif soc == 14:
        return ("nobility", "count, countess")
    elif soc == 15:
        return ("nobility", "duke, duchess")
    elif soc == 16:
        return ("nobility", "archduke, archduchess")
    elif soc == 17:
        return ("nobility", "king, queen")
    else:
        return ("nobility", "emperor, empress")


class CharacterStats:
    strength: int
    strength_dm: int
    dexterity: int
    dexterity_dm: int
    endurance: int
    endurance_dm: int
    intelligence: int
    intelligence_dm: int
    education: int
    education_dm: int
    iq: int
    social_standing: int
    social_standing_dm: int
    height: int
    mass: int
    skin_tone: str
    hair_color: str
    eye_color: str
    soc_class: str
    soc_desc_title: str

    def __init__(
        self,
        strength: int,
        strength_dm: int,
        dexterity: int,
        dexterity_dm: int,
        endurance: int,
        endurance_dm: int,
        intelligence: int,
        intelligence_dm: int,
        education: int,
        education_dm: int,
        iq: int,
        social_standing: int,
        social_standing_dm: int,
        height: int,
        mass: int,
        skin_tone: str,
        hair_color: str,
        eye_color: str,
        soc_class: str,
        soc_desc_title: str,
    ) -> None:
        self.strength = strength
        self.strength_dm = strength_dm
        self.dexterity = dexterity
        self.dexterity_dm = dexterity_dm
        self.endurance = endurance
        self.endurance_dm = endurance_dm
        self.intelligence = intelligence
        self.intelligence_dm = intelligence_dm
        self.education = education
        self.education_dm = education_dm
        self.iq = iq
        self.social_standing = social_standing
        self.social_standing_dm = social_standing_dm
        self.height = height
        self.mass = mass
        self.skin_tone = skin_tone
        self.hair_color = hair_color
        self.eye_color = eye_color
        self.soc_class = soc_class
        self.soc_desc_title = soc_desc_title

    def __str__(self) -> str:
        return f"""
        Strength: {self.strength} (DM {self.strength_dm})\n
        Dexterity: {self.dexterity} (DM {self.dexterity_dm})\n
        Endurance: {self.endurance} (DM {self.endurance_dm})\n
        Intelligence: {self.intelligence} (DM {self.intelligence_dm})\n
        IQ: {self.iq}\n
        Education: {self.education} (DM {self.education_dm})\n
        Social Standing: {self.social_standing} (DM {self.social_standing_dm})(class: {self.soc_class}, description/title: {self.soc_desc_title})\n
        Height: {self.height} cm\n
        Mass: {self.mass} kg\n
        Skin Tone: \"{self.skin_tone}\"\n
        Hair Color: \"{self.hair_color}\"\n
        Eye Color: \"{self.eye_color}\""""


def gen_character_stats() -> CharacterStats:
    strength_roll = proc_roll_dice_cmd(3, 6, 0, "+")
    strength = strength_roll.total
    strength_dm = calc_stat_dm(strength)

    dexterity_roll = proc_roll_dice_cmd(3, 6, 0, "+")
    dexterity = dexterity_roll.total
    dexterity_dm = calc_stat_dm(dexterity)

    endurance_roll = proc_roll_dice_cmd(3, 6, 0, "+")
    endurance = endurance_roll.total
    endurance_dm = calc_stat_dm(endurance)

    intelligence_roll = proc_roll_dice_cmd(3, 6, 0, "+")
    intelligence = intelligence_roll.total
    intelligence_dm = calc_stat_dm(intelligence)

    education_roll = proc_roll_dice_cmd(3, 6, 0, "+")
    education = education_roll.total
    education_dm = calc_stat_dm(education)

    iq = intelligence + education

    social_standing_roll = proc_roll_dice_cmd(3, 6, 0, "+")
    social_standing = social_standing_roll.total
    social_standing_dm = calc_stat_dm(social_standing)

    height = calc_height(strength)
    mass = calc_mass(endurance, dexterity)
    skin_tone = gen_skin_tone()
    hair_color = gen_hair_color()
    eye_color = gen_eye_color(hair_color)
    soc_class, soc_desc_title = gen_soc_standing(social_standing)

    return CharacterStats(
        strength,
        strength_dm,
        dexterity,
        dexterity_dm,
        endurance,
        endurance_dm,
        intelligence,
        intelligence_dm,
        education,
        education_dm,
        iq,
        social_standing,
        social_standing_dm,
        height,
        mass,
        skin_tone,
        hair_color,
        eye_color,
        soc_class,
        soc_desc_title,
    )
