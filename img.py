from PIL import ImageGrab
from pyautogui import *
from pynput import mouse, keyboard
from enum import Enum
import os

class MouseState(Enum):
    IDLE = 0
    CLK1 = 1
    CLK2 = 2

    def __add__(self, other):
        if self.value >= 2:
            return self
        if isinstance(other, int):
            return MouseState((self.value + other) % len(MouseState))
        return NotImplemented


MOUSE = MouseState.IDLE
X0, Y0 = (0, 0)
RECT_WIDTH = 825
Q = "Q_"
QA = "QA_"
PREFIX = "20240224_"
POSTFIX = "_七年级|第十四章|三角形|解答题"
FILE_DIRECTORY = "./"

def on_click(x, y, button, pressed):
    global MOUSE
    if MOUSE.value >= 2:
        return 
    if pressed:
        # Your code here
        
        MOUSE = MOUSE + 1
        print("pressed, now MOUSE is {}".format(MOUSE))

def on_move(x, y):
    global MOUSE
    if MOUSE.value >= 2:
        return 
    print("x:{:>4}, y:{:>4}\r".format(x, y), end='', flush=True)


def on_scroll(x, y, _dx, _dy):
    global MOUSE
    if MOUSE.value >= 2:
        return 
    MOUSE = MOUSE + 1
    print("scrolled, now MOUSE is {}".format(MOUSE))
    

def save_picture(save_path, x1, y1):
    print("save picture : {}", (X0, Y0, X0+RECT_WIDTH, y1))
    im = ImageGrab.grab(bbox=(X0, Y0, X0+RECT_WIDTH, y1))
    im.save(save_path)

def on_press(key):
    global MOUSE
    if MOUSE.value >= 2:
        return
    print(f"key {key} pressed")
    MOUSE = MOUSE + 1
    print("pressed, now MOUSE is {}".format(MOUSE))


def main():
    listener = mouse.Listener(on_move=on_move,on_click=None, on_scroll=None)
    kb_listener = keyboard.Listener(on_press=on_press)
    listener.start()
    kb_listener.start()
    global X0, Y0
    X0, Y0 = position()
    while MOUSE == MouseState.IDLE:
        pass
    X0, Y0 = position()
    print("x:{:>4}, y:{:>4}".format(X0, Y0))
    while MOUSE == MouseState.CLK1:
        pass
    x1, y1 = position()
    print("x:{:>4}, y:{:>4}".format(x1, y1))

    tp = input("Q(1) or QA(2)?: ")
    print(tp)
    tp = 'Q_' if '1' in tp else 'QA_'
    num = input("\n\n\n\n\n\n\n\nplease enter the number: ")
    file_path = tp + PREFIX + str(num) + POSTFIX + ".png"
    file_dir = "./tmp"
    file_path = os.path.join(file_dir, file_path)
    save_picture(file_path, x1, y1)


if __name__ == "__main__":

    main()

