pub trait RightNow<T> {
    fn runwrap(self) -> T;
    fn rexpect(self, msg: &str) -> T;
}
