[pypl](https://pypi.org/project/pepeline/)

[git](https://github.com/scanlate-wiki/pipeline-rs)


# pepeline-rs
Fast rust-python librarian for internal needs of an organization
```py
from pepeline import screentone, fast_color_level, read, save, cvt_color, CvtType
img = read(<"img path">, 0, 0)
img = fast_color_level(
    img,     
    in_low = 10,
    in_high = 240,
    out_low = 0,
    out_high = 255,
    gamma = 1.0
)
img = cvt_color(img, CvtType.RGB2CMYK)
img[:, :, 0] = screenton(img[:, :, 0], dot_size=7, angle=-15)
img[:, :, 1] = screenton(img[:, :, 1], dot_size=7, angle=0)
img[:, :, 2] = screenton(img[:, :, 2], dot_size=7, angle=15)
img[:, :, 3] = screenton(img[:, :, 3], dot_size=7, angle=30)
img = cvt_color(img, CvtType.CMYK2RGB)
save(img, "out.png")
```
# TODO:
- resize❓
- documentation ♻️
- refactoring ♻️
- add tests ♻️
- add benchmarks scripts ♻️
- simd ❓
# Function:
- read - read img (supports psd)
- screentone - add screenton patern.
- fast_color_level - color levels correction
- noise_generate - ganerate noise array
- save - fast save image
- cvt_color - converts color extensions, currently only supports f32 and in some places 0-1
