import pygame
import os
import sys
from typing import Tuple

SCREEN_HEIGHT = 1200
SCREEN_WIDTH = 1600

LVL_INFO: Tuple = (1, "Info")
LVL_DEBUG = (2, "Debug")
LVL_WARNING = (3, "Warning")
LVL_ERROR = (4, "Error")

PRINT_EVENTS = True


def run() -> int:
    print("starting")
    try:

        pygame.init()

        all_fonts = pygame.font.get_fonts()

        def_font_64 = pygame.font.Font('/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf', 48)
        text = def_font_64.render("Test 1 2 3 ", True, (0, 128, 0))

        resources_path = os.path.join(os.getcwd(), 'resources/')

        screen: pygame.Surface = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
        done: bool = False
        clock: pygame.time.Clock = pygame.time.Clock()

        ball_1_img: pygame.Surface = pygame.image.load(os.path.join(resources_path, 'ball_1.png'))

        is_blue = True
        x = 60
        y = 60

        while not done:
            # event handling
            for event in pygame.event.get():
                if PRINT_EVENTS is True:
                    print("event retrieved: {event}".format(event=event))
                if event.type == pygame.QUIT:
                    done = True
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_SPACE:
                        is_blue = not is_blue
                    else:
                        pressed = pygame.key.get_pressed()
                        if pressed[pygame.K_UP]: y -= 3
                        if pressed[pygame.K_DOWN]: y += 3
                        if pressed[pygame.K_LEFT]: x -= 3
                        if pressed[pygame.K_RIGHT]: x += 3



            # draw stuff
            if is_blue is True:
                color = (0,128,255)
            else:
                color = (255, 100, 0)

            # bottom
            screen.fill((0, 0, 0))

            # 0
            screen.blit(ball_1_img, (20, 20))

            # 1
            pygame.draw.rect(screen, color, pygame.Rect(x, y, 60, 60))

            # 2
            pygame.draw.circle(screen, (228, 242, 110), (80,80), 120)

            # 3
            screen.blit(text, ((SCREEN_WIDTH / 2)- text.get_width() // 2, (SCREEN_HEIGHT / 2) - text.get_height() // 2))

            pygame.display.flip()
            clock.tick(60)
        # END OF LOOP
    except KeyboardInterrupt:
        print("keyboard interrupt, exiting...")
    except Exception as e:
        print("exception occurred: {}".format(e))
        return 1

    print("exiting")

    return 0


if __name__ == "__main__":
    sys.exit(run())

# END OF FILE
