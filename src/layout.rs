use super::ids::Id;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Size {
    MatchParent,
    WrapContent,
    Exact(u16),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Alignment {
    None,
    Above(Id),
    Below(Id),
    ToLeftOf(Id),
    ToRightOf(Id),
    AlignTop(Id),
    AlignBottom(Id),
    AlignLeft(Id),
    AlignRight(Id),
    AlignParentLeft,
    AlignParentRight,
    AlignParentTop,
    AlignParentBottom,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BoundarySize {
    AllTheSame(i32),
    OrthoDirections(i32, i32),
    Distinct(i32, i32, i32, i32),
}
impl From<(i32, i32, i32, i32)> for BoundarySize {
    fn from(a: (i32, i32, i32, i32)) -> BoundarySize {
        let h = a.0 == a.2;
        let v = a.1 == a.3;
        if h && v {
            if a.0 == a.1 {
                BoundarySize::AllTheSame(a.0)
            } else {
                BoundarySize::OrthoDirections(a.0, a.1)
            }
        } else {
            BoundarySize::Distinct(a.0, a.1, a.2, a.3)
        }
    }
}
impl From<BoundarySize> for (i32, i32, i32, i32) {
    fn from(a: BoundarySize) -> (i32, i32, i32, i32) {
        match a {
            BoundarySize::AllTheSame(v) => (v, v, v, v),
            BoundarySize::OrthoDirections(lr, tb) => (lr, tb, lr, tb),
            BoundarySize::Distinct(l, t, r, b) => (l, t, r, b),
        }
    }
}
pub enum BoundarySizeParam {
    OrthoHorizontal(i32),
    OrthoVertical(i32),
    Left(i32),
    Top(i32),
    Right(i32),
    Bottom(i32),
}
impl ::std::ops::BitOr<BoundarySizeParam> for BoundarySizeParam {
    type Output = BoundarySize;
    fn bitor(self, rhs: BoundarySizeParam) -> Self::Output {
        let mut left = 0;
        let mut top = 0;
        let mut right = 0;
        let mut bottom = 0;
        match self {
            BoundarySizeParam::OrthoHorizontal(lr) => {
                left = lr;
                right = lr;
            }
            BoundarySizeParam::OrthoVertical(tb) => {
                top = tb;
                bottom = tb;
            }
            BoundarySizeParam::Left(l) => left = l,
            BoundarySizeParam::Top(t) => {
                top = t;
            }
            BoundarySizeParam::Right(r) => {
                right = r;
            }
            BoundarySizeParam::Bottom(b) => {
                bottom = b;
            }
        }
        match rhs {
            BoundarySizeParam::OrthoHorizontal(lr) => {
                left = lr;
                right = lr;
            }
            BoundarySizeParam::OrthoVertical(tb) => {
                top = tb;
                bottom = tb;
            }
            BoundarySizeParam::Left(l) => left = l,
            BoundarySizeParam::Top(t) => {
                top = t;
            }
            BoundarySizeParam::Right(r) => {
                right = r;
            }
            BoundarySizeParam::Bottom(b) => {
                bottom = b;
            }
        }
        (left, top, right, bottom).into()
    }
}
impl ::std::ops::BitOr<BoundarySizeParam> for BoundarySize {
    type Output = BoundarySize;
    fn bitor(self, rhs: BoundarySizeParam) -> Self::Output {
        let (mut left, mut top, mut right, mut bottom) = self.into();
        match rhs {
            BoundarySizeParam::OrthoHorizontal(lr) => {
                left = lr;
                right = lr;
            }
            BoundarySizeParam::OrthoVertical(tb) => {
                top = tb;
                bottom = tb;
            }
            BoundarySizeParam::Left(l) => left = l,
            BoundarySizeParam::Top(t) => {
                top = t;
            }
            BoundarySizeParam::Right(r) => {
                right = r;
            }
            BoundarySizeParam::Bottom(b) => {
                bottom = b;
            }
        }
        (left, top, right, bottom).into()
    }
}
pub enum BoundarySizeArgs {
    Param(BoundarySizeParam),
    Set(BoundarySize),
}
impl From<BoundarySizeParam> for BoundarySizeArgs {
    fn from(a: BoundarySizeParam) -> BoundarySizeArgs {
        BoundarySizeArgs::Param(a)
    }
}
impl From<BoundarySizeParam> for BoundarySize {
    fn from(a: BoundarySizeParam) -> BoundarySize {
        BoundarySize::AllTheSame(0) | a
    }
}
impl From<BoundarySize> for BoundarySizeArgs {
    fn from(a: BoundarySize) -> BoundarySizeArgs {
        BoundarySizeArgs::Set(a)
    }
}
impl From<BoundarySizeArgs> for BoundarySize {
    fn from(a: BoundarySizeArgs) -> BoundarySize {
        match a {
            BoundarySizeArgs::Param(param) => param.into(),
            BoundarySizeArgs::Set(set) => set,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct Attributes {
    pub width: Size,
    pub height: Size,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            width: Size::MatchParent,
            height: Size::WrapContent,
        }
    }
}
