


def proc_skill_check_cmd(cmd: str) -> TextLine:
    # /roll skill check skill=10 difficulty=10 modifier=10
    re1 = re.compile(r"skill\s*check\s*skill\s*=\s*(\d+)\s*difficulty\s*=\s*(\d+)\s*modifier\s*=\s*(\d+)")
    if re1.match(cmd) is not None:
        skill = int(re1.match(cmd).group(1))
        difficulty = int(re1.match(cmd).group(2))
        modifier = int(re1.match(cmd).group(3))
        # roll = random.randint(1, 20)
        roll_res = roll_dice(2, 6, 0, "+")
        roll = roll_res.total
        # result = roll + skill + modifier
        
        
        
        if roll == 20:
            result_txt = f"roll={roll}, skill={skill}, difficulty={difficulty}, modifier={modifier}, result=\"critical success\""
        elif roll == 1:
            result_txt = f"roll={roll}, skill={skill}, difficulty={difficulty}, modifier={modifier}, result=\"critical failure\""
        elif result >= difficulty:
            result_txt = f"roll={roll}, skill={skill}, difficulty={difficulty}, modifier={modifier}, result=\"success\""
        else:
            result_txt = f"roll={roll}, skill={skill}, difficulty={difficulty}, modifier={modifier}, result=\"failure\""
        return TextLine(result_txt, TextType.COMMAND_OUTPUT)