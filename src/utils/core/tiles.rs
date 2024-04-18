use ndarray::{ArrayD, Axis, s};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{Py, pyclass, pymethods, PyResult, Python};

#[pyclass]
pub struct Tiles {
    tiles: Vec<ArrayD<f32>>,
    tile_n: usize,
    tiles_size: usize,
    shape: [usize; 2],
}


fn img2tiles(img: &ArrayD<f32>, tile_size: usize) -> (Vec<ArrayD<f32>>, [usize; 2])
{
    let mut tiles_list: Vec<ArrayD<f32>> = Vec::new();
    let shape = img.shape();
    let mut shape_it = [1, 1];
    match shape.len() {
        2 => {
            let (x, y) = (shape[0], shape[1]);
            let nx = x / tile_size;
            let ny = y / tile_size;

            if nx == 0 && ny == 0 {
                tiles_list.push(img.clone());
            } else if nx == 0 && ny != 0 {
                for j in 0..ny {
                    let tile = img.slice(s![0..x, tile_size * j..tile_size * (j + 1)]);
                    tiles_list.push(tile.to_owned().into_dyn());
                }
                shape_it = [1, ny]
            } else if nx != 0 && ny == 0 {
                for j in 0..nx {
                    let tile = img.slice(s![tile_size * j..tile_size * (j + 1),0..x]);
                    tiles_list.push(tile.to_owned().into_dyn());
                }
                shape_it = [nx, 1]
            } else {
                for i in 0..x / tile_size {
                    for j in 0..y / tile_size {
                        let tile = img.slice(s![tile_size * i..tile_size * (i + 1), tile_size * j..tile_size * (j + 1)]);
                        tiles_list.push(tile.to_owned().into_dyn());
                    }
                }
                shape_it = [nx, ny]
            }
            (tiles_list, shape_it)
        }
        3 => {
            let (x, y) = (shape[0], shape[1]);
            let nx = x / tile_size;
            let ny = y / tile_size;

            if nx == 0 && ny == 0 {
                tiles_list.push(img.clone());
            } else if nx == 0 && ny != 0 {
                for j in 0..ny {
                    let tile = img.slice(s![0..x, tile_size * j..tile_size * (j + 1),..]);
                    tiles_list.push(tile.to_owned().into_dyn());
                }
                shape_it = [1, ny]
            } else if nx != 0 && ny == 0 {
                for j in 0..nx {
                    let tile = img.slice(s![tile_size * j..tile_size * (j + 1),0..x,..]);
                    tiles_list.push(tile.to_owned().into_dyn());
                }
                shape_it = [nx, 1]
            } else {
                for i in 0..x / tile_size {
                    for j in 0..y / tile_size {
                        let tile = img.slice(s![tile_size * i..tile_size * (i + 1), tile_size * j..tile_size * (j + 1),..]);
                        tiles_list.push(tile.to_owned().into_dyn());
                    }
                }
                shape_it = [nx, ny]
            }
            (tiles_list, shape_it)
        }
        _ => panic!("Unsupported number of dimensions"),
    }
}

#[pymethods]
impl Tiles {
    #[new]
    pub fn new<'py>(img: PyReadonlyArrayDyn<f32>,
                    tile_size: usize,
                    // py:Python
    ) -> Self {
        let img = img.as_array().to_owned();
        let tiles_and_shape = img2tiles(&img, tile_size);
        let tiles = tiles_and_shape.0;
        let tile_n = 0usize;
        let tiles_size = tiles.len();
        let shape = tiles_and_shape.1;
        Self {
            tiles,
            tile_n,
            tiles_size,
            shape,
        }
    }
    pub fn right(&mut self) {
        if self.tile_n < self.tiles_size - 1 {
            self.tile_n += 1
        }
    }
    pub fn left(&mut self) {
        if self.tile_n > 0 {
            self.tile_n -= 1;
        }
    }
    pub fn get<'py>(&self, py: Python) -> PyResult<Py<PyArrayDyn<f32>>> {
        let tile = &self.tiles[self.tile_n];
        Ok(tile.to_pyarray(py).to_owned())
    }
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.tiles_size)
    }
    pub fn update<'py>(&mut self, img: PyReadonlyArrayDyn<f32>) -> PyResult<bool> {
        let tile = img.as_array().to_owned();
        self.tiles[self.tile_n] = tile;
        Ok(true)
    }
    pub fn to_img<'py>(&self, py: Python) -> PyResult<Py<PyArrayDyn<f32>>> {
        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut combinet = self.tiles[0].to_owned();
        let mut n = 1;
        for _ in 1..cols {
            let result = ndarray::concatenate(Axis(1), &[combinet.view(), self.tiles[n].view()]).to_owned();
            match result {
                Ok(concatenated) => {
                    combinet = concatenated.to_owned();
                }
                Err(err) => {
                    eprintln!("Error concatenating arrays: {:?}", err);
                }
            }
            n += 1;
        }
        for _ in 1..rows {
            let mut line = self.tiles[n].to_owned();
            n += 1;
            for _ in 1..cols {
                let result = ndarray::concatenate(Axis(1), &[line.view(), self.tiles[n].view()]);
                match result {
                    Ok(concatenated) => {
                        line = concatenated.to_owned();
                    }
                    Err(err) => {
                        eprintln!("Error concatenating arrays: {:?}", err);
                    }
                }
                n += 1
            }
            let result = ndarray::concatenate(Axis(0), &[combinet.view(), line.view()]);
            match result {
                Ok(concatenated) => {
                    combinet = concatenated.to_owned();
                }
                Err(err) => {
                    eprintln!("Error concatenating arrays: {:?}", err);
                }
            }
        }
        Ok((combinet.to_pyarray(py).to_owned()))
    }
    fn __repr__(&self) -> String {
        format!("Tiles({:?}),len({}),shape({:?})", self.tiles, self.tiles_size, self.shape)
    }
}
