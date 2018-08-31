//! Abstract arithmetic

pub trait ModuleMul {
    fn mul(self, other: Self, module: Self) -> Self;
}

pub trait ModuleAdd {
    fn add(self, other: Self, module: Self) -> Self;
}

pub trait ModuleNeg {
    fn neg(self, module: Self) -> Self;
}

pub trait Value: Sized + Clone + Copy + ModuleAdd + ModuleMul + ModuleNeg {
    fn one() -> Self;
    fn zero() -> Self;
}