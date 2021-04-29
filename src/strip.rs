use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct Strip<T> {
    data: Rc<Vec<T>>,
    index: isize,
}

 impl<T> Strip<T> {
    #[inline(always)]
    fn len(&self) -> isize {
        self.data.len() as isize
    }

    #[inline(always)]
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    #[inline(always)]
    pub fn new(data: Vec<T>, index: isize) -> Strip<T> {
        Strip {
            data: Rc::new(data),
            index,
        }
    }

    #[inline(always)]
    pub fn get(&self, offset: isize) -> Strip<T> {
        let len = self.len();
        let future_index = self.index + offset;

        // TODO: discutir checagem de borda
        if future_index < 0 {
            return self.get(offset + len);
        }
        if future_index >= self.len() {
            return self.get(offset - len);
        }

        // TODO: implemente get_extract() evitando criar essa strip extra aqui
        Strip {
            data: self.data.clone(),
            index: self.index + offset,
        }
    }

    #[inline(always)]
    fn get_all(&self) -> Vec<Strip<T>> {
        (0..(self.len()))
            .map(|index| Strip {
                data: self.data.clone(),
                index,
            })
            .collect()
    }
}

// Comonad
impl<A> Strip<A>
where
    A: Copy + std::fmt::Debug,
{
    #[inline(always)]
    pub fn extract(self) -> A {
        self.data[self.index as usize]
    }

    // TODO: Como fazer isso no mesmo array se eu não sei o quanto que tenho que guardar pra
    // sobrescrever?
    pub fn extend<B>(self, f: impl Fn(&Strip<A>) -> B) -> Strip<B> {
        let data: Vec<B> = self.get_all().iter().map(|s| f(s)).collect();
        Strip {
            index: self.index,
            data: Rc::new(data),
        }
    }
}
