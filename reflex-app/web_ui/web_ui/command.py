from .saving_throw import proc_saving_throw_cmd
from .text_type import TextType
from .dice import proc_roll_dice_cmd

from .character_stats import gen_character_stats
from .text_line import TextLine


def process_command(raw_command: str) -> TextLine:
    print(f'raw_command: "{raw_command}"')
    # help
    if raw_command.startswith("help"):
        return TextLine("Help", TextType.HELP)
    # roll commands
    elif raw_command.startswith("roll"):
        roll_cmd = raw_command.removeprefix("roll").strip()
        # roll dice
        if roll_cmd.startswith("dice"):
            return proc_roll_dice_cmd(roll_cmd)
        # roll saving throw
        elif roll_cmd.startswith("saving throw"):
            return proc_saving_throw_cmd(roll_cmd)
        # invalid roll command
        else:
            raise ValueError(f'unknown command "{raw_command}"')
    # gen commands
    elif raw_command.startswith("gen"):
        gen_cmd = raw_command.removeprefix("gen").strip()
        # gen character stats
        if gen_cmd.startswith("character stats"):
            stats = gen_character_stats()
            return TextLine(str(stats), TextType.COMMAND_OUTPUT)
        # invalid gen command
        else:
            raise ValueError(f'unknown command "{raw_command}"')
    # invalid command
    else:
        raise ValueError(f'unknown command "{raw_command}"')
