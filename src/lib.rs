mod traits;

impl<T> traits::RightNow<T> for std::option::Option<T> {
    fn runwrap(self) -> T {
        self.unwrap()
    }
    fn rexpect(self, msg: &str) -> T {
        self.expect(msg)
    }
}

impl<T, E: std::fmt::Debug> traits::RightNow<T> for std::result::Result<T, E> {
    fn runwrap(self) -> T {
        self.unwrap()
    }
    fn rexpect(self, msg: &str) -> T {
        self.expect(msg)
    }
}
