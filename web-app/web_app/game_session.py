import reflex as rx
from typing import List

from sqlmodel import Field, Relationship


class GameSession(rx.Model, table=True):
    id: int = Field(primary_key=True)
    name: str
    messages: List["SessionMessage"] = Relationship(back_populates="game_session")


class SessionMessage(rx.Model, table=True):
    id: int = Field(primary_key=True)
    message: str
    message_type: str
    game_session_id: int = Field(foreign_key="gamesession.id")
    game_session: GameSession = Relationship(back_populates="messages")
