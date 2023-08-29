"""Welcome to Reflex! This file outlines the steps to create a basic app."""
from multiprocessing import process
from string import whitespace
from rxconfig import config
import reflex as rx
from .text_line import TextLine

from .text_type import TextType
from .command import process_command


docs_url = "https://reflex.dev/docs/getting-started/introduction"
filename = f"{config.app_name}/{config.app_name}.py"


def process_question(question: str) -> TextLine:
    """Process the question and return an answer."""
    if question.startswith("/"):
        raw_command = question[1:]
        # if raw_command.startswith("help"):
        #     return TextLine("Help", TextType.HELP)
        # elif raw_command.startswith("roll"):
        #     return proc_roll_dice_cmd(raw_command)
        # elif raw_command.startswith("gen"):
        #     return process_gen_command(raw_command)
        return process_command(raw_command)
    return TextLine("I don't know", TextType.ERROR)


class State(rx.State):
    """The app state."""

    question: str = ""
    new_session_name: str = ""
    chat_history: list[tuple[str, str]]
    session_names: list[str]
    selected_session_name: str = ""

    def answer(self):
        # process question
        answer_text_line = process_question(self.question)
        print(f"answer_text_line: \"{answer_text_line}\"")
        answer = answer_text_line.text
        self.chat_history.append((self.question, answer))
    
    def add_session(self):
        print(f"add_session: {self.new_session_name}")
        self.session_names.append(self.new_session_name)
        
    def load_selected_session(self):
        print(f"load_selected_session: {self.selected_session_name}")
    
    def delete_selected_session(self):
        print(f"delete_selected_session: {self.selected_session_name}")
        self.session_names.remove(self.selected_session_name)
        self.selected_session_name = ""


def qa(question: str, answer: str) -> rx.Component:
    return rx.box(
        rx.box(question, text_align="right"),
        rx.box(answer, text_align="left", whitespace="break-spaces"),
        margin_y="1em",
    )


def action_bar() -> rx.Component:
    return rx.hstack(
        rx.input(placeholder="Type a message...", on_blur=State.set_question),
        rx.button("Send", margin_left="1em", on_click=State.answer),
    )


def chat() -> rx.Component:
    return rx.box(
        rx.foreach(
            State.chat_history,
            lambda messages: qa(messages[0], messages[1]),
        )
    )
    
def session_mgr_ui() -> rx.Component:
    return rx.box(
        # text label
        rx.text("Session Manager"),
        # session name input for new
        rx.input(placeholder="session name...", on_blur=State.set_new_session_name),
        # add button
        rx.button("New Session", on_click=State.add_session),
        # dropdown list
        rx.select(
            State.session_names,
            placeholder="select a session...",
            on_change=State.set_selected_session_name,
        ),
        # load button
        rx.button(
            "Load Session",
            on_click=State.load_selected_session,),
        # delete button
        rx.button(
            "Delete Selected Session",
            on_click=State.delete_selected_session,
        )
    )


def index() -> rx.Component:
    return rx.container(chat(), action_bar(), session_mgr_ui())


# Add state and page to the app.
app = rx.App()
app.add_page(index)
app.compile()
