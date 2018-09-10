
use field::{FieldValue, MulScalar, Scalar};
use {Curve, JacobianPoint};

#[derive(Clone, PartialEq, Debug)]
pub struct Point<C: Curve> {
    x: C::Value,
    y: C::Value,
}

impl<C: Curve> Point<C> {
    pub fn infinity() -> Self {
        Point {
            x: C::Value::zero(),
            y: C::Value::zero(),
        }
    }

    pub fn is_infinity(&self) -> bool {
        self.x == C::Value::zero() && self.y == C::Value::zero()
    }

    pub fn new(x: C::Value, y: C::Value) -> Self {
        Point { x: x, y: y }
    }

    pub fn x(&self) -> C::Value {
        self.x
    }

    pub fn y(&self) -> C::Value {
        self.y
    }

    pub fn into_parts(self) -> (C::Value, C::Value) {
        (self.x, self.y)
    }
}

impl<I, C: Curve> From<(I, I)> for Point<C>
    where I: Into<C::Value>
{
    fn from(p: (I, I)) -> Self {
        Point {
            x: p.0.into(),
            y: p.1.into(),
        }
    }
}

impl<C: Curve> From<JacobianPoint<C>> for Point<C> {
    fn from(p: JacobianPoint<C>) -> Self {
        let (x, y, z) = p.into_parts();

        if z == C::Value::zero() {
            return Self::infinity();
        }

        let z2 = z.squared();
        let z3 = z2 * z;

        (x / z2, y / z3).into()
    }
}

impl<C: Curve> ::std::ops::Add for Point<C>
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.is_infinity() && other.is_infinity() { return Self::infinity(); }
        if self.is_infinity() { return other; }
        if other.is_infinity() { return self; }

        let (x1, y1) = self.into_parts();
        let (x2, y2) = other.into_parts();

        if x1 == x2 {
            // point doubling
            if y1 != y2 {
                return Self::infinity();
            }

            if y1 == C::Value::zero() {
                return Self::infinity();
            }

            let l = (x1.squared().mul_scalar(3) + C::a()) / y1.mul_scalar(2);
            let x = l.squared() - x1.mul_scalar(2);

            return (x, l * (x1 - x) - y1).into();
        }

        let l = (y2 - y1) / (x2 - x1);
        let x3 = l.squared() - x1 - x2;
        let y3 = l * (x1 - x3) - y1;

        (x3, y3).into()
    }
}

impl<I: Scalar, C: Curve> ::std::ops::Mul<I> for Point<C>
{
    type Output = Self;

    fn mul(self, other: I) -> Self {
        let mut r0 = Self::infinity();
        let mut r1 = self;
        for i in 0..I::max_bits() {
            let b = Scalar::bit(&other, i);
            if b { r0 = r0 + r1.clone() }
            r1 = r1.clone() + r1;
        }
        r0
    }
}

#[cfg(test)]
mod tests {

    use test::{U64Curve, U64MontgomeryCurve};
    use Curve;

    #[test]
    fn double() {
        let p = U64Curve::generator();
        let dp = p.clone() + p;

        // 570768668753918, 222182780873386
        assert_eq!(dp, (570768668753918, 222182780873386).into());
    }

    #[test]
    fn add() {
        let p = U64Curve::generator() + U64Curve::generator();
        let np = p.clone() + U64Curve::generator();

        assert_eq!(np, (537613624567015, 945163207984607).into());
    }

    #[test]
    fn double_mont() {
        let p = U64MontgomeryCurve::generator() + U64MontgomeryCurve::generator();

        println!("U64MontgomeryCurve::generator() = {:?}", U64MontgomeryCurve::generator());

        assert_eq!(p, (570768668753918, 222182780873386).into());
    }

    #[test]
    fn mul() {
        let p = U64Curve::generator();
        let dp = p.clone() * 2;
        assert_eq!(dp, (570768668753918, 222182780873386).into());
        let bp = p * 570768668753918;
        assert_eq!(bp, (210159848059198, 473433224346301).into());
    }

    // #[test]
    // fn mul_montgomery() {
    //     let p = U64MontgomeryCurve::generator();

    //     let dp = p.clone() * 2;
    //     assert_eq!(dp, (570768668753918, 222182780873386).into());

    //     let bp = p * 570768668753918;
    //     assert_eq!(bp, (210159848059198, 473433224346301).into());
    // }
}