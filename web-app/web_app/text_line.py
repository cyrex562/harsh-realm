from .text_type import TextType
import reflex as rx


class TextLine(rx.Model, table=True):
    text: str
    text_type: str

    def __init__(self, text:str="", text_type:str="none") -> None:
        self.text = text
        self.text_type = text_type