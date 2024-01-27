mod traits;

impl<T> traits::RightNow<T> for std::option::Option<T> {
    fn runwrap(self) -> T {
        self.unwrap()
    }
    fn rexpect(self, msg: &str) -> T {
        self.expect(msg)
    }
}
