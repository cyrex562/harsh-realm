import enum


class TextType(enum.Enum):
    NONE = "none"
    COMMAND_INPUT = "output"
    COMMAND_OUTPUT = "input"
    HELP = "help"
    ERROR = "error"
    QUESTION = "question"
    ANSWER = "answer"
