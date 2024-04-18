from __future__ import annotations

from enum import Enum

import numpy as np


def read(
        path: str,
        mode: None | int,
        format: None | int
) -> np.ndarray: """ The function to read the image. input parameters: 
\n path -> str file path 
\n mode -> uint 0 -> gray 1-> rgb 2-> psd dynamic format, and in other cases rgb, None = 2 
\n format -> uint 0 -> f32 0-1 img, 1+ -> u8 0-255, None = 1"""


def screentone(
        array: np.ndarray,
        dot_size: int,
        ly_plus: None | int,
        lx_plus: None | int,
) -> np.ndarray: """screentone overlay function:
                  \n     input -> array only 2D f32 0-1
                  \n     dot_size -> uint screenton size in pixels
                  \n     lx_plus and ly_plus -> uint offset of the pattern by the number of pixels specified by these parameters. None=dot_size/2"""


class TypeNoise(Enum):
    PERLIN = 0,
    SIMPLEX = 1,
    OPENSIMPLEX = 2,
    SUPERSIMPLEX = 3,
    PERLINSURFLET = 4


class Tiles:
    def __init__(self, img: np.ndarray, tile_size: int) -> None: ...

    def right(self) -> None: ...

    def __len__(self) -> int: ...

    def left(self) -> None: ...

    def get(self) -> np.ndarray: ...

    def to_img(self) -> np.ndarray: ...

    def update(self, img: np.ndarray) -> bool: ...


def fast_color_level(
        array: np.ndarray,
        in_low: None | int,
        in_high: None | int,
        out_low: None | int,
        out_high: None | int,
        gamma: None | int,
) -> np.ndarray: """ array:np.float32 
\n in_low...out_high:uint8 
\n gamma:float32"""


def noise_generate(
        size: tuple[int, int] | tuple[int, int, int],
        type_noise: TypeNoise,
        octaves: int,
        frequency: float,
        lacunarity: float,
        seed: int | None,
) -> np.ndarray: """ size:tuple 2d or 3d
\n type_noise: TypeNoise
\n octaves: uint 
\n frequency:float32
\n lacunarity:float32
\n seed: uint"""


def save(
        array: np.ndarray,
        path: str
) -> np.ndarray: """function to save an image, currently supports:
                  \n     f32 0-1 array
                  \n     u8 0-255 array"""
