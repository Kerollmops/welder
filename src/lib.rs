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
        let welder = Welder::new(glue);
        welder.push_no_glue(start)
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

    /// This will add the element without any glue.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::with_start(' ', "foo");
    ///
    /// let welder = welder.push_no_glue("bar");
    /// let welder = welder.push_no_glue("baz");
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!("foobarbaz", &string);
    /// ```
    pub fn push_no_glue<E>(mut self, elem: E) -> Self
    where
        T: Extend<E>
    {
        self.welded.extend(once(elem));
        self
    }
}

impl<G, T> Welder<G, T>
where
    G: Clone,
    T: Extend<G>
{
    /// Push a new value to the already accumulated values.
    /// This function will add a glue element in front of the element.
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
    pub fn push<E>(self, elem: E) -> Self
    where
        T: Extend<E>
    {
        self.push_glue_left(elem)
    }

    /// It will add a glue only to right of the element.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::with_start(' ', "foo");
    ///
    /// let welder = welder.push_glue_right("bar");
    /// let welder = welder.push_glue_right("baz");
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!("foobar baz ", &string);
    /// ```
    pub fn push_glue_right<E>(mut self, elem: E) -> Self
    where
        T: Extend<E>
    {
        self.welded.extend(once(elem));
        self.welded.extend(once(self.glue.clone()));
        self
    }

    /// This is the default push function.
    /// It will add a glue only to left of the element.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::new(' ');
    ///
    /// let welder = welder.push_glue_left("foo");
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!(" foo", &string);
    /// ```
    pub fn push_glue_left<E>(mut self, elem: E) -> Self
    where
        T: Extend<E>
    {
        self.welded.extend(once(self.glue.clone()));
        self.welded.extend(once(elem));
        self
    }

    /// This will add a glue on both sides of the element.
    ///
    /// # Examples
    ///
    /// ```
    /// use welder::Welder;
    ///
    /// let welder = Welder::new(' ');
    ///
    /// let welder = welder.push_glue_both("foo");
    ///
    /// let string: String = welder.weld();
    ///
    /// assert_eq!(" foo ", &string);
    /// ```
    pub fn push_glue_both<E>(mut self, elem: E) -> Self
    where
        T: Extend<E>
    {
        self.welded.extend(once(self.glue.clone()));
        self.welded.extend(once(elem));
        self.welded.extend(once(self.glue.clone()));
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
