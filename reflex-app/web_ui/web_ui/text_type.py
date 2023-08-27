import enum


class TextType(enum.Enum):
    NONE=0,
    COMMAND_INPUT = 1,
    COMMAND_OUTPUT = 2,
    HELP = 3,
    ERROR = 4

