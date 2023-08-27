import reflex as rx

class WebuiConfig(rx.Config):
    pass

config = WebuiConfig(
    app_name="web_ui",
    db_url="sqlite:///reflex.db",
    env=rx.Env.DEV,
)