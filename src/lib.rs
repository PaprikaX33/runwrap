mod traits;
pub use traits::RightNow as rn;

impl<T> rn<T> for std::option::Option<T> {
    fn runwrap(self) -> T {
        self.unwrap()
    }
    fn rexpect(self, msg: &str) -> T {
        self.expect(msg)
    }
}

impl<T, E: std::fmt::Debug> rn<T> for std::result::Result<T, E> {
    fn runwrap(self) -> T {
        self.unwrap()
    }
    fn rexpect(self, msg: &str) -> T {
        self.expect(msg)
    }
}
