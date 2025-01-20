use macroquad::math::Vec2;

#[derive(PartialEq)]
pub enum Item {
    // Tools
    Hoe,
    Axe,
    // Materials
    Wood,
}

pub struct ItemStack {
    pub item: Item,
    pub count: u32,
}

#[derive(PartialEq)]
pub struct WorldItem {
    pub item: Item,
    pub pos: Vec2,
}

impl Eq for WorldItem {}

impl PartialOrd for WorldItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.pos.y.partial_cmp(&other.pos.y)
    }
}

impl Ord for WorldItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
