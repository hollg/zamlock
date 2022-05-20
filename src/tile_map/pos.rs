use ordered_float::OrderedFloat;
// #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
// pub(crate) struct Pos(pub(crate) u32, pub(crate) u32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Pos(
    pub(crate) OrderedFloat<f32>,
    pub(crate) OrderedFloat<f32>,
    pub(crate) OrderedFloat<f32>,
);

impl Pos {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Pos {
        Pos(OrderedFloat(x), OrderedFloat(y), OrderedFloat(z))
    }
}
