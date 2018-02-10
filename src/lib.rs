#[derive(Debug)]
pub struct Welder<G, T> {
    glue: G,
    welded: T,
}

impl<G, T: Default> Welder<G, T> {
    pub fn new(glue: G) -> Self {
        Welder {
            glue: glue,
            welded: <T as Default>::default(),
        }
    }
}

impl<G, T> Welder<G, T> {
    pub fn weld(self) -> T {
        self.welded
    }
}

impl<G, T> Welder<G, T> {
    pub fn push<E>(&mut self, elem: E) -> &mut Self
    where
        T: Extend<E> + Extend<G>
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
