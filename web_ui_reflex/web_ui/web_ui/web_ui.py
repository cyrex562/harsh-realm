"""Welcome to Reflex! This file outlines the steps to create a basic app."""
from string import whitespace
from rxconfig import config
import reflex as rx
from .text_line import TextLine

from .text_type import TextType

from .command import process_gen_command, process_roll_command


docs_url = "https://reflex.dev/docs/getting-started/introduction"
filename = f"{config.app_name}/{config.app_name}.py"

def process_question(question: str) -> TextLine:
    """Process the question and return an answer."""
    if question.startswith("/"):
        raw_command = question[1:]
        if raw_command.startswith("help"):
            return TextLine("Help",TextType.HELP)
        elif raw_command.startswith("roll"):
            return process_roll_command(raw_command)
        elif raw_command.startswith("gen"):
            return process_gen_command(raw_command)
    return TextLine("I don't know",TextType.ERROR)


class State(rx.State):
    """The app state."""
    question: str = ""
    chat_history: list[tuple[str,str]]
    
    def answer(self):
        # process question
        answer_text_line = process_question(self.question)
        answer = answer_text_line.text
        self.chat_history.append((self.question,answer))


def qa(question: str, answer: str) -> rx.Component:
    return rx.box(
        rx.box(question,text_align="right"),
        rx.box(answer,text_align="left",whitespace="break-spaces"),
        margin_y="1em"
    )
    
def action_bar() -> rx.Component:
    return rx.hstack(
        rx.input(placeholder="Type a message...",
                 on_blur=State.set_question),
        rx.button("Send", margin_left="1em", on_click=State.answer),
    )


def chat() -> rx.Component:
    return rx.box(
        rx.foreach(
            State.chat_history,
            lambda messages: qa(messages[0], messages[1]),
        )
    )

def index() -> rx.Component:
    return rx.container(chat(),action_bar())


# Add state and page to the app.
app = rx.App()
app.add_page(index)
app.compile()
