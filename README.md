[pypl](https://pypi.org/project/pepeline/)

[git](https://github.com/scanlate-wiki/pipeline-rs)


# pepeline-rs
Fast rust-python librarian for internal needs of an organization
```py
from pepeline import screentone, fast_color_level, read, save
img = read(<"img path">, 0, 0)
img = fast_color_level(
    img,     
    in_low = 10,
    in_high = 240,
    out_low = 0,
    out_high = 255,
    gamma = 1.0
)
img = screenton(img, dot_size=7)
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
