import random
import re

from .text_type import TextType

from .text_line import TextLine

def proc_saving_throw_cmd(cmd: str) -> TextLine:
    # /roll saving throw score=10
    re1 = re.compile(r"saving\s*throw\s*score\s*=\s*(\d+)")
    if re1.match(cmd) is not None:
        score = int(re1.match(cmd).group(1))
        roll = random.randint(1, 20)
        result = roll + score
        if roll == 20:
            result_txt = f"roll={roll}, score={score}, result=\"critical success\""
        elif roll == 1:
            result_txt = f"roll={roll}, score={score}, result=\"critical failure\""
        elif roll >= score:
            result_txt = f"roll={roll}, score={score}, result=\"success\""
        else:
            result_txt = f"roll={roll}, score={score}, result=\"failure\""
        return TextLine(result_txt, TextType.COMMAND_OUTPUT)
    else:
        raise ValueError(f'unknown command "{cmd}"')