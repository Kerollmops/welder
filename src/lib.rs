use std::iter::once;

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
    pub fn from<U>(glue: G, base: &U) -> Self
    where
        U: ToOwned + ?Sized,
        U::Owned: Into<T>
    {
        Welder {
            glue: glue,
            welded: Into::into(ToOwned::to_owned(base)),
        }
    }
}

impl<G, T> Welder<G, T> {
    pub fn weld(self) -> T {
        self.welded
    }
}

impl<G, T> Welder<G, T>
where
    G: Clone,
    T: Extend<G>
{
    pub fn push<E>(&mut self, elem: E) -> &mut Self
    where
        T: Extend<E>
    {
        self.welded.extend(once(self.glue.clone()));
        self.welded.extend(once(elem));

        self
    }
}

#[cfg(test)]
mod tests {
    use super::Welder;

    #[test]
    fn string_welder() {
        let mut welder = Welder::new(' ');

        welder.push("foo");

        let string: String = welder.weld();

        assert_eq!(" foo", &string);
    }

    #[test]
    fn string_welder_from_base() {
        let mut welder = Welder::from(' ', "foo");

        welder.push("bar");

        let string: String = welder.weld();

        assert_eq!("foo bar", &string);
    }
}
