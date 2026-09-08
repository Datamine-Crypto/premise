pub trait Bounded {
    type Of: Copy;
    const LO: Self::Of;
    const HI: Self::Of;
}

pub trait HasMax {
    type Of: Copy;
    const MAX: Self::Of;
}

pub trait HasMin {
    type Of: Copy;
    const MIN: Self::Of;
}

impl<T: Bounded> HasMax for T {
    type Of = <T as Bounded>::Of;
    const MAX: Self::Of = <T as Bounded>::HI;
}

impl<T: Bounded> HasMin for T {
    type Of = <T as Bounded>::Of;
    const MIN: Self::Of = <T as Bounded>::LO;
}
