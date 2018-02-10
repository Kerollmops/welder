//! A tool to help concatenate, implemented with a consuming builder pattern.
//!
//! ## Examples
//!
//! ```
//! use welder::Welder;
//!
//! let welder = Welder::with_start(' ', "foo");
//!
//! let welder = welder.push("bar");
//! let welder = welder.push("baz");
//! let welder = welder.push("boat");
//!
//! let string: String = welder.weld();
//!
//! assert_eq!("foo bar baz boat", &string);
//! ```
//!
//! ```
//! use welder::Welder;
//!
//! let welder = Welder::with_start(0, 12);
//!
//! let vec: Vec<_> = welder.push(14)
//!                         .push(16)
//!                         .push(18)
//!                         .weld();
//!
//! assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
//! ```

use std::iter::once;

/// An helper struct to accumalate elements.
pub struct Welder<G, T> {
    glue: G,
    welded: T,
}

impl<G, T: Default> Welder<G, T> {
    /// Create an empty `Welder` just by defining the glue used.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::new(' ');
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!("", &string);
    /// ```
    pub fn new(glue: G) -> Self {
        Welder {
            glue: glue,
            welded: <T as Default>::default(),
        }
    }

    /// Create a `Welder` with a first value and the glue it will use.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::with_start(' ', "foo");
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!("foo", &string);
    /// ```
    pub fn with_start<E>(glue: G, start: E) -> Self
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
    /// Retrieve the accumulated values from the `Welder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::with_start(' ', "foo");
    ///
    /// let welder = welder.push("bar").push("baz").push("foo");
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!("foo bar baz foo", &string);
    /// ```
    pub fn weld(self) -> T {
        self.welded
    }
}

impl<G, T> Welder<G, T>
where
    G: Clone,
    T: Extend<G>
{
    /// Push a new value to the already accumulated values.
    /// This function will add a glue element in front of the elem pushed.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::new(' ');
    ///
    /// let welder = welder.push("foo");
    /// let welder = welder.push("bar");
    /// let welder = welder.push("baz");
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!(" foo bar baz", &string);
    /// ```
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
        let string: String = Welder::with_start(' ', "foo").push("bar").weld();

        assert_eq!("foo bar", &string);
    }

    #[test]
    fn string_welder_multiple() {
        let welder = Welder::with_start(' ', "foo");

        let welder = welder.push("bar");
        let welder = welder.push("baz");
        let welder = welder.push("boat");

        let string: String = welder.weld();

        assert_eq!("foo bar baz boat", &string);
    }

    #[test]
    fn vec_welder_multiple() {
        let welder = Welder::with_start(0, 12);

        let welder = welder.push(14);
        let welder = welder.push(16);
        let welder = welder.push(18);

        let vec: Vec<_> = welder.weld();

        assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
    }

    #[test]
    fn vec_welder_chain() {
        let welder = Welder::with_start(0, 12);

        let vec: Vec<_> = welder.push(14)
                                .push(16)
                                .push(18)
                                .weld();

        assert_eq!(&[12, 0, 14, 0, 16, 0, 18], vec.as_slice());
    }

    #[test]
    fn string_welder_chain() {
        let string: String = Welder::with_start(' ', "foo")
                                .push("bar")
                                .push("baz")
                                .push("boat")
                                .weld();

        assert_eq!("foo bar baz boat", &string);
    }
}
