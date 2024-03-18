# pipeline-rs
Fast rust-python librarian for internal needs of an organization
```py
from pepeline import screenton, fast_color_level, save, read
import numpy as np
img = np.array(read(<"img path">,0)).astype(np.float32)/255
img = fast_color_level(
    img,     
    in_low = 10,
    in_high = 240,
    out_low = 0,
    out_high = 255,
    gamma = 1.0
)
img = screenton(img, dot_size=7)
save((img * 255).astype(np.uint8), "out2.png")
```
# TODO:
- resize❓
- sharp♻️
- documentation ♻️
- refactoring ♻️
# Function:
- read - read rgb img.
- read_gray - reag gray img.
- screenton - add screenton patern.
- fast_color_level - color levels correction
- save - fast save image
