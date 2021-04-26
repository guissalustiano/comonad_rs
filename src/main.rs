#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use comonad::traids::{Comonad, Functor};

#[derive(Debug, Clone)]
struct Strip<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> Strip<T>
where
    T: Clone,
{
    pub fn shift_left(&self) -> Strip<T> {
        Strip {
            data: self.data.clone(),
            index: if self.index == 0 {
                self.data.len() - 1
            } else {
                self.index - 1
            },
        }
    }

    pub fn shift_right(&self) -> Strip<T> {
        Strip {
            data: self.data.clone(),
            index: if self.index == self.data.len() - 1 {
                0
            } else {
                self.index + 1
            },
        }
    }
}

impl Strip<bool> {
    fn pretty(&self) -> String {
        self.data
            .iter()
            .map(|i| if *i { "██" } else { "  " })
            .collect()
    }
}

impl Strip<bool> {}

impl<A> Functor<A> for Strip<A>
where
    A: Clone,
{
    type F<T> = Strip<T>;

    fn fmap<B>(self, f: impl Fn(A) -> B) -> Self::F<B> {
        Strip {
            data: self.data.iter().map(|i| f(i.clone())).collect::<Vec<B>>(),
            index: self.index,
        }
    }
}

impl<A> Comonad<A> for Strip<A>
where
    A: Clone + Copy,
{
    type W<T> = Strip<T>;

    fn extract(self) -> A {
        self.data[self.index]
    }

    fn extend<B>(self, f: impl Fn(Self::W<A>) -> B) -> Self::W<B> {
        Strip {
            index: self.index,
            data: std::iter::repeat(self.clone())
                .take(self.data.len())
                .enumerate()
                .map(|(i, mut s)| {
                    s.index = i;
                    s
                })
                .map(|s| f(s))
                .collect(),
        }
    }
}
fn main() {
    let rule = |s: Strip<bool>| -> bool {
        let l = s.clone().shift_left().extract();
        let m = s.clone().extract();
        let r = s.clone().shift_right().extract();

        match (l, m, r) {
            (true, false, false) | (false, false, true) => true,
            (_, _, _) => false,
        }
    };
    let mut layer = Strip {
        index: 0,
        data: vec![false; 51],
    };
    layer.data[25] = true;
    for _ in 0..30 {
        println!("{}", layer.pretty());
        layer = layer.extend(rule);
    }
}
