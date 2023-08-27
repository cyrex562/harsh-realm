import csv
from typing import Dict, List, Tuple

FATE_CHART = {}
ODDS = []


# TODO: should we hard code this table?
def init_fate_chart():
    with open('data/mythic_fate.csv') as csvfile:
        fate_reader = csv.reader(csvfile, delimiter=",")
        i = 0
        for row in fate_reader:
            if i == 0:
                i += 1
                continue
            # row => odds, [xyes value xno] chaos 1 - 9
            ODDS.append(row[0])
            FATE_CHART.update({row[0]: {
                    1: {"xyes": int(row[1]), "value": int(row[2]), "xno": int(row[3])},
                    2: {"xyes": int(row[4]), "value": int(row[5]), "xno": int(row[6])},
                    3: {"xyes": int(row[7]), "value": int(row[8]), "xno": int(row[9])},
                    4: {"xyes": int(row[10]), "value": int(row[11]), "xno": int(row[12])},
                    5: {"xyes": int(row[13]), "value": int(row[14]), "xno": int(row[15])},
                    6: {"xyes": int(row[16]), "value": int(row[17]), "xno": int(row[18])},
                    7: {"xyes": int(row[19]), "value": int(row[20]), "xno": int(row[21])},
                    8: {"xyes": int(row[22]), "value": int(row[23]), "xno": int(row[24])},
                    9: {"xyes": int(row[25]), "value": int(row[26]), "xno": int(row[27])},
            }})
            i += 1


def get_fate_chart_cell(odds: str, chaos_factor: int) -> Tuple[str, str, str]:
    if odds not in ODDS:
        raise ValueError("odds value {} not in odds: [{}]".format(odds, ODDS))
    if chaos_factor not in [1,2,3,4,5,6,7,8,9]:
        raise ValueError("chaos factor {} must be a value between 1 and 9".format(chaos_factor))

    odds_row = FATE_CHART[odds]
    odds_for_chaos_factor = odds_row[chaos_factor]

    return odds_for_chaos_factor["xyes"], odds_for_chaos_factor["value"], odds_for_chaos_factor["xno"]


def consult_fate_chart(odds: str, dice_roll: int, chaos_factor: int) -> str:
    if odds not in ODDS:
        raise ValueError("odds value {} not in odds: [{}]".format(odds, ODDS))
    if chaos_factor not in [1,2,3,4,5,6,7,8,9]:
        raise ValueError("chaos factor {} must be a value between 1 and 9".format(chaos_factor))

    odds_row = FATE_CHART[odds]
    odds_for_chaos_factor = odds_row[chaos_factor]
    if dice_roll <= odds_for_chaos_factor["xyes"]:
        return "xyes"
    if dice_roll <= odds_for_chaos_factor["value"]:
        return "yes"
    if dice_roll < odds_for_chaos_factor["xno"]:
        return "no"
    return "xno"


def get_odd_vals() -> List[str]:
    return ODDS


def get_fate_chart() -> Dict:
    return FATE_CHART
