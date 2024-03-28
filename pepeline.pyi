from __future__ import annotations

import numpy as np


def read(
        path: str,
        mode: int | None,
        format: int | None
) -> np.ndarray: ("The function to read the image. "
                  "\nInput parameters: "
                  "\n    path -> str file path "
                  "\n    mode -> uint 0 -> gray 1-> rgb 2-> psd dynamic format, and in other cases rgb, None = 2 "
                  "\n    format -> uint 0 -> f32 0-1 img, 1+ -> u8 0-255, None = 1")


def screentone(
        array: np.ndarray,
        dot_size: int,
        ly_plus: int | None,
        lx_plus: int | None,
) -> np.ndarray: ("screentone overlay function:"
                  "\n     input -> array only 2D f32 0-1"
                  "\n     dot_size -> uint screenton size in pixels"
                  "\n     lx_plus and ly_plus -> uint offset of the pattern by the number of pixels specified by these parameters. None=dot_size/2")


def fast_color_level(
        array: np.ndarray,
        in_low: int | None,
        in_high: int | None,
        out_low: int | None,
        out_high: int | None,
        gamma: float | None,
) -> np.ndarray: " array:np.float32 \n in_low...out_high:uint8 \n gamma:float32"


def save(
        array: np.ndarray,
        path: str
) -> np.ndarray: ("function to save an image, currently supports:"
                  "\n     f32 0-1 array"
                  "\n     u8 0-255 array")
