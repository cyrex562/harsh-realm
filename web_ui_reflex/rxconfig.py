import reflex as rx

class WebuireflexConfig(rx.Config):
    pass

config = WebuireflexConfig(
    app_name="web_ui_reflex",
    db_url="sqlite:///reflex.db",
    env=rx.Env.DEV,
)