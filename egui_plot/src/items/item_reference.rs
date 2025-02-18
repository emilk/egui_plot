/// A reference to an item.
///
/// Since item ids are optional, we need to keep the name as well, using it for identification if needed.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ItemReference {
    pub name: String,
    pub item_id: Option<egui::Id>,
}

impl PartialOrd for ItemReference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Only sorted by name, id is not considred.
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for ItemReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
