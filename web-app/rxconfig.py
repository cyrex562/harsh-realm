from sqlite3 import dbapi2
import reflex as rx


class WebappConfig(rx.Config):
    pass


config = WebappConfig(app_name="web_app", db_url="sqlite:///../data/reflex.db")
