from __future__ import annotations

import numpy as np

def read(
        path:str,
        mode:int|None
)->np.ndarray: "open image\n mode 0 = Gray 1 = Color"
def screenton(
        array:np.ndarray,
        dot_size:int,
        ly_plus:int|None,
        lx_plus:int|None,
)->np.ndarray:"array:np.float32\n dot_size:uint,ly and lx_plus: uint, if None == dot_size//2 "
def fast_color_level(
        array: np.ndarray,
        in_low: int,
        in_high: int,
        out_low: int,
        out_high: int,
        gamma: float,
)->np.ndarray: " array:np.float32 \n in_low...out_high:uint8 \n gamma:float32"

def save(
        array: np.ndarray,
        path:str
)->np.ndarray:"array:np.uint8"