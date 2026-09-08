pub trait Cap {
    const MAX: u32;
}
impl<T> Cap for T {
    const MAX: u32 = 500;
}
