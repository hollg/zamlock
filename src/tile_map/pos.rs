use ordered_float::OrderedFloat;

/// Pos uses OrderedFloats so that it can be a key in a hashmap. Implementing Ord will
/// also be important for pathfinding later.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    pub(crate) x: OrderedFloat<f32>,
    pub(crate) y: OrderedFloat<f32>,
    pub(crate) z: OrderedFloat<f32>,
}

impl Pos {
    pub(crate) fn new<
        T: Into<OrderedFloat<f32>>,
        U: Into<OrderedFloat<f32>>,
        V: Into<OrderedFloat<f32>>,
    >(
        x: T,
        y: U,
        z: V,
    ) -> Pos {
        Pos {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

/// Sometimes it's easier to work with the f32 directly
pub(crate) struct UnorderedPos {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl From<Pos> for UnorderedPos {
    fn from(pos: Pos) -> Self {
        UnorderedPos {
            x: f32::from(pos.x),
            y: f32::from(pos.y),
            z: f32::from(pos.z),
        }
    }
}
