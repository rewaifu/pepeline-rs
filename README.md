# pipeline-rs
Fast rust-python librarian for internal needs of an organization
```py
from pepeline import read_gray, screenton, fast_color_level,normolize,save #read - read color image rgb8
import numpy as np
img = np.array(read_gray(<"img path">)).astype(np.float32)/255
img = normolize(img) #img - only array f32 
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
