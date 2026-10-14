    fn get(&self, i: usize) -> Option<Match>;

    /// Return the overall match for the capture.
    ///
    /// This returns the match for index `0`. That is it is equivalent to
    /// `get(0).unwrap()`
    #[inline]
    fn as_match(&self) -> Match {
        self.get(0).unwrap()
    }
