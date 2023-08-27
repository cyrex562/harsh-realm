# TODO: use argparse to parse and rsponse to chat messages

def help_msg() -> str:
    return """
    /help, /h, /? - this message
    """

def process_chat_msg(message: str) -> str:
    """
    Given a message, return a response.
    """
    if message.lower() in ["/help", "/h", "/?"]:
        return help_msg()
    else:
        return help_msg()
