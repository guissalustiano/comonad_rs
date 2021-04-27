#![feature(in_band_lifetimes)]

use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug, Clone)]
struct Strip<T> {
    data: Rc<Vec<T>>,
    index: isize,
}

impl<T> Strip<T> {
    #[inline(always)]
    fn len(&self) -> isize {
        self.data.len() as isize
    }

    #[inline(always)]
    fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    #[inline(always)]
    fn new(data: Vec<T>, index: isize) -> Strip<T> {
        Strip {
            data: Rc::new(data),
            index,
        }
    }

    #[inline(always)]
    fn get(&self, offset: isize) -> Strip<T> {
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
    fn extract(self) -> A {
        self.data[self.index as usize]
    }

    // TODO: Como fazer isso no mesmo array se eu não sei o quanto que tenho que guardar pra
    // sobrescrever?
    fn extend<B>(self, f: impl Fn(&Strip<A>) -> B) -> Strip<B> {
        let data: Vec<B> = self.get_all().iter().map(|s| f(s)).collect();
        Strip {
            index: self.index,
            data: Rc::new(data),
        }
    }
}
fn main() {
    let rule = |s: &Strip<bool>| -> bool {
        let l = s.get(-1).extract();
        let m = s.get(0).extract();
        let r = s.get(1).extract();

        match (l, m, r) {
            (true, false, false) | (false, false, true) => true,
            (_, _, _) => false,
        }
    };

    let mut data = vec![false; 50];
    data[25] = true;

    let mut layer = Strip::new(data, 0);

    for _ in 0..30 {
        let pretty: String = layer.iter().map(|i| if *i { "██" } else { "  " }).collect();
        println!("{}", pretty);
        layer = layer.extend(rule);
    }
}
