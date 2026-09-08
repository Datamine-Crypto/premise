pub struct Vault;

impl Vault {
    pub fn squash<T: PartialOrd>(v: T, hi: T, lo: T) -> T {
        if v > hi {
            lo
        } else {
            v
        }
    }
}
