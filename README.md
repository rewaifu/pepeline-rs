[pypl](https://pypi.org/project/pepeline/)

[git](https://github.com/scanlate-wiki/pipeline-rs)


# pipeline-rs
Fast rust-python librarian for internal needs of an organization
```py
from pepeline import screentone, fast_color_level,save32,read32
img = read32(<"img path">,0)
img = fast_color_level(
    img,     
    in_low = 10,
    in_high = 240,
    out_low = 0,
    out_high = 255,
    gamma = 1.0
)
img = screenton(img, dot_size=7)
save32(img, "out.png")
```
# TODO:
- resize❓
- sharp♻️
- documentation ♻️
- refactoring ♻️
- add tests♻️
- add benchmarks scripts♻️
# Function:
- read - read img u8.(supports psd)
- read32 - read img float32.(supports psd)
- screentone - add screenton patern.
- fast_color_level - color levels correction
- save - fast save image
- save32 - fast save image float32
