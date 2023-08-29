"""Welcome to Reflex! This file outlines the steps to create a basic app."""
from typing import Optional
from rxconfig import config

import reflex as rx
from .text_line import TextLine

from .text_type import TextType
from .command import process_command
from .game_session import GameSession, SessionMessage


def process_question(question: str) -> TextLine:
    """Process the question and return an answer."""
    if question.startswith("/"):
        raw_command = question[1:]
        return process_command(raw_command)
    return TextLine("I don't know", TextType.ANSWER.value)


class State(rx.State):
    """The app state."""

    question: str = ""
    new_session_name: str = ""
    chat_history: list[TextLine]
    session_names: list[str]
    selected_session_name: str = ""
    game_sessions: list[GameSession] = []
    loaded_game_session: Optional[GameSession] = None
    loaded_session_name: str = "not loaded"
    not_loaded_show: bool = False

    @rx.var
    def get_chat_history(self):
        self.chat_history = []
        print("get chat history")
        if self.loaded_game_session is None:
            return self.chat_history
        with rx.session() as session:
            messages = (
                session.query(SessionMessage)
                .filter(SessionMessage.game_session_id == self.loaded_game_session.id)
                .all()
            )
            for msg in messages:
                mtype = (
                    TextType.QUESTION.value
                    if msg.message_type == "question"
                    else TextType.ANSWER.value
                )

                self.chat_history.append(TextLine(msg.message, mtype))
            print(f"chat history={self.chat_history}")
            return self.chat_history

    @rx.var
    def get_game_sessions(self):
        print("get game sessions")
        with rx.session() as session:
            self.game_sessions = session.query(GameSession).all()
            return self.game_sessions

    @rx.var
    def get_session_names(self):
        print("get session names")
        with rx.session() as session:
            self.session_names = [gs.name for gs in session.query(GameSession).all()]
            return self.session_names

    def not_loaded_toggle(self):
        self.not_loaded_show = not self.not_loaded_show

    def get_gsession_by_name(self):
        print("get session by name")
        with rx.session() as session:
            gs = (
                session.query(GameSession)
                .filter(GameSession.name == self.selected_session_name)
                .first()
            )
            return gs

    def answer(self):
        if self.loaded_game_session is None:
            print("No session loaded")
            self.not_loaded_show = True
            return
        print(f"answering query: {self.question}")
        # process question
        answer_text_line = process_question(self.question)
        print(f'answer_text_line: "{answer_text_line}"')
        question_text_line = TextLine(self.question, TextType.QUESTION.value)

        with rx.session() as session:
            session.add(
                SessionMessage(
                    message=self.question,
                    message_type="question",
                    game_session_id=self.loaded_game_session.id,
                )
            )
            session.add(
                SessionMessage(
                    message=answer_text_line.text,
                    message_type="answer",
                    game_session_id=self.loaded_game_session.id,
                )
            )
            session.commit()
            self.loaded_game_session = (
                session.query(GameSession)
                .filter(GameSession.name == self.loaded_game_session.name)
                .first()
            )
            messages = (
                session.query(SessionMessage)
                .filter(SessionMessage.game_session_id == self.loaded_game_session.id)
                .all()
            )
            for msg in messages:
                mtype = (
                    TextType.QUESTION.value
                    if msg.message_type == "question"
                    else TextType.ANSWER.value
                )

                self.chat_history.append(TextLine(msg.message, mtype))
            print(f"chat history={self.chat_history}")
            # return self.chat_history
        


    def add_game_session(self):
        print(f"add_session: {self.new_session_name}")
        self.session_names.append(self.new_session_name)
        game_session = GameSession()
        game_session.name = self.new_session_name
        # self.sessions.append(game_session)
        # self.selected_game_session = game_session
        with rx.session() as session:
            session.add(GameSession(name=self.new_session_name))
            session.commit()

    def load_selected_session(self):
        print(f"load_selected_session: {self.selected_session_name}")
        with rx.session() as session:
            self.loaded_game_session = (
                session.query(GameSession)
                .filter(GameSession.name == self.selected_session_name)
                .first()
            )
            self.loaded_session_name = self.loaded_game_session.name
            # update chat history
            print("get chat history")
            self.chat_history = []
            messages = (
                session.query(SessionMessage)
                .filter(SessionMessage.game_session_id == self.loaded_game_session.id)
                .all()
            )
            for msg in messages:
                self.chat_history.append(
                    TextLine(
                        msg.message,
                        TextType.QUESTION.value
                        if msg.message_type == "question"
                        else TextType.ANSWER.value,
                    )
                )

        print(f"loaded session: {self.loaded_session_name}")

    def delete_selected_session(self):
        print(f"delete_selected_session: {self.selected_session_name}")
        self.session_names.remove(self.selected_session_name)
        # self.sessions.remove(self.selected_game_session)
        with rx.session() as session:
            session.query(GameSession).filter(
                GameSession.name == self.selected_session_name
            ).delete()
            session.commit()
        if self.loaded_session_name == self.selected_session_name:
            self.loaded_session_name = "not loaded"
        self.selected_session_name = ""
        self.loaded_game_session = None


# def qa(question: str, answer: str) -> rx.Component:
#     return rx.box(
#         rx.box(question, text_align="right"),
#         rx.box(answer, text_align="left", whitespace="break-spaces"),
#         margin_y="1em",
#     )


def render_chat_msg(msg: TextLine) -> rx.Component:
    return rx.cond(
        msg.text_type == "question",
        rx.box(msg.text, text_align="left", margin_y="0.5em", bg="lightgreen"),
        rx.box(msg.text, text_align="right", margin_y="0.5em", bg="lightblue"),
    )


def action_bar() -> rx.Component:
    return rx.hstack(
        rx.input(placeholder="Type a message...", on_blur=State.set_question),
        rx.button("Send", margin_left="1em", on_click=State.answer),
    )


def chat() -> rx.Component:
    return rx.box(
        rx.foreach(State.chat_history, render_chat_msg),
        rx.alert_dialog(
            rx.alert_dialog_overlay(
                rx.alert_dialog_content(
                    rx.alert_dialog_header("No Session Loaded"),
                    rx.alert_dialog_body(
                        "Please load a session before sending messages."
                    ),
                    rx.alert_dialog_footer(
                        rx.button(
                            "Ok",
                            on_click=State.not_loaded_toggle,
                        )
                    ),
                )
            ),
            is_open=State.not_loaded_show,
        ),
    )


def session_mgr_ui() -> rx.Component:
    return rx.box(
        # text label
        rx.text("Session Manager"),
        # session name input for new
        rx.input(placeholder="session name...", on_blur=State.set_new_session_name),
        # add button
        rx.button("New Session", on_click=State.add_game_session),
        # dropdown list
        rx.select(
            State.session_names,
            placeholder="select a session...",
            on_change=State.set_selected_session_name,
        ),
        # load button
        rx.button(
            "Load Session",
            on_click=State.load_selected_session,
        ),
        # delete button
        rx.button(
            "Delete Selected Session",
            on_click=State.delete_selected_session,
        ),
        rx.box(rx.text(f"Loaded Session: {State.loaded_session_name}")),
    )


def index() -> rx.Component:
    return rx.container(chat(), action_bar(), session_mgr_ui())


# Add state and page to the app.
app = rx.App()
app.add_page(index)
app.compile()
