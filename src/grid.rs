use nalgebra::{DMatrix, Scalar};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Rc<DMatrix<T>>,
    index: (isize, isize),
}

impl<T> Grid<T>
where
    T: Scalar,
{
    #[inline(always)]
    pub fn new(data: DMatrix<T>, index: (isize, isize)) -> Grid<T> {
        Grid {
            data: Rc::new(data),
            index,
        }
    }

    #[inline(always)]
    pub fn get(&self, offset: (isize, isize)) -> Grid<T> {
        let nrows = (*self.data).nrows() as isize;
        let ncols = (*self.data).ncols() as isize;
        let future_index = (self.index.0 + offset.0, self.index.1 + offset.1);

        // TODO: discutir checagem de borda
        if future_index.0 < 0
            || future_index.1 < 0
            || future_index.0 >= ncols
            || future_index.1 >= nrows
        {
            return self.clone();
        }

        // TODO: implemente get_extract() evitando criar essa strip extra aqui
        Grid {
            data: self.data.clone(),
            index: future_index,
        }
    }

    #[inline(always)]
    fn get_all(&self) -> Vec<Grid<T>> {
        let nrows = (*self.data).nrows() as isize;
        let ncols = (*self.data).ncols() as isize;

        (0..ncols).flat_map(|y| (0..nrows).map(move |x| (x, y)))
            .map(|index| Grid {
                data: self.data.clone(),
                index,
            })
            .collect()
    }
}

// Comonad
impl<A> Grid<A>
where
    A: Scalar + Copy + std::fmt::Debug,
{
    #[inline(always)]
    pub fn extract(self) -> A {
        let index = (self.index.0 as usize, self.index.1 as usize);
        self.data[index]
    }

    // TODO: Como fazer isso no mesmo array se eu não sei o quanto que tenho que guardar pra
    // sobrescrever?
    pub fn extend<B: Scalar>(self, f: impl Fn(&Grid<A>) -> B) -> Grid<B> {
        let nrows = (*self.data).nrows();
        let ncols = (*self.data).ncols();
        let data = DMatrix::from_iterator(nrows, ncols, self.get_all().iter().map(|s| f(s)));
        Grid {
            index: self.index,
            data: Rc::new(data),
        }
    }
}
