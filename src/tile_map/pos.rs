use ordered_float::OrderedFloat;

/// Pos uses OrderedFloats so that it can be a key in a hashmap. Implementing Ord will
/// also be important for pathfinding later.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Pos {
    pub(crate) x: OrderedFloat<f32>,
    pub(crate) y: OrderedFloat<f32>,
    pub(crate) z: OrderedFloat<f32>,
}

impl Pos {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Pos {
        Pos {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            z: OrderedFloat(z),
        }
    }
}
