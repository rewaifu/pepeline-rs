# pipeline-rs
Fast rust-python librarian for internal needs of an organization
```py
from pepeline import screenton, fast_color_level,save32,read32
import numpy as np
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
- add benchmarks screepts♻️
# Function:
- read - read img u8.
- read32 - read img float32.
- screenton - add screenton patern.
- fast_color_level - color levels correction
- save - fast save image
- save32 - fast save image float32
