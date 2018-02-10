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

    pub fn start<E>(glue: G, start: E) -> Self
    where
        T: Extend<E>
    {
        let mut base = <T as Default>::default();
        base.extend(once(start));

        Welder {
            glue: glue,
            welded: base,
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
    pub fn push<E>(mut self, elem: E) -> Self
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
        let string: String = Welder::new(' ').push("foo").weld();

        assert_eq!(" foo", &string);
    }

    #[test]
    fn string_welder_from_base() {
        let string: String = Welder::start(' ', "foo").push("bar").weld();

        assert_eq!("foo bar", &string);
    }

    #[test]
    fn string_welder_multiple() {
        let welder = Welder::start(' ', "foo");

        let welder = welder.push("bar");
        let welder = welder.push("baz");
        let welder = welder.push("boat");

        let string: String = welder.weld();

        assert_eq!("foo bar baz boat", &string);
    }

    #[test]
    fn vec_welder_multiple() {
        let welder = Welder::start(0, 12);

        let welder = welder.push(14);
        let welder = welder.push(16);
        let welder = welder.push(18);

        let vec: Vec<_> = welder.weld();

        assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
    }

    #[test]
    fn vec_welder_chain() {
        let welder = Welder::start(0, 12);

        let vec: Vec<_> = welder.push(14)
                                .push(16)
                                .push(18)
                                .weld();

        assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
    }

    #[test]
    fn string_welder_chain() {
        let string: String = Welder::start(' ', "foo")
                                .push("bar")
                                .push("baz")
                                .push("boat")
                                .weld();

        assert_eq!("foo bar baz boat", &string);
    }
}
