#[derive(Clone, Copy)]
pub enum BVE<'bv> {
    BVV(&'bv BVV),
    BVS(&'bv BVS),
}

#[derive(Clone, Copy)]
pub enum BV<'bv> {
    Concrete(BVV),
    Symbolic(BVS),
    Expression(BVE<'bv>),
}

#[derive(Clone, Copy)]
pub struct BVV {
    size: usize,
}

impl BVV {
    pub fn new(size: usize) -> BVV {
        return BVV {
            size: size
        };
    }

    pub fn from_u64(num: u64) -> BVV {
        return BVV {
            size: 64
        };
    }
}

#[derive(Clone, Copy)]
pub struct BVS {
    size: usize,
}

impl BVS {
    pub fn new(size: usize) -> BVS {
        return BVS {
            size: size
        };
    }

    pub fn from_u64(num: u64) -> BVS {
        return BVS {
            size: 64
        }
    }
}
