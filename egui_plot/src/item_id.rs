//! Identifiers for the items within a plot.

use core::hash::Hash;
use core::num::NonZeroU64;

/// The seeds used when hashing an [`ItemId`] source.
///
/// Fixed so that the same source always produces the same [`ItemId`],
/// which is what lets [`crate::PlotMemory`] be persisted between runs.
const HASH_SEEDS: (u64, u64, u64, u64) = (9, 10, 11, 12);

/// An `ItemIdSet` is a `HashSet<ItemId>` that skips hashing,
/// since an [`ItemId`] already is a high-entropy hash.
pub type ItemIdSet = nohash_hasher::IntSet<ItemId>;

/// An `ItemIdMap<V>` is a `HashMap<ItemId, V>` that skips hashing,
/// since an [`ItemId`] already is a high-entropy hash.
pub type ItemIdMap<V> = nohash_hasher::IntMap<ItemId, V>;

/// Identifies a [`crate::PlotItem`] within a plot.
///
/// An `ItemId` only has to be unique within the plot it is used in.
///
/// By default each item derives its `ItemId` from the name it was created with,
/// but you can set one explicitly, e.g. with [`crate::Line::id`]. Do that when
/// the name changes between frames, or when several items share a name.
///
/// This is niche-optimized, so that `Option<ItemId>` is the same size as `ItemId`.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ItemId(NonZeroU64);

impl nohash_hasher::IsEnabled for ItemId {}

impl ItemId {
    /// Hash any source (e.g. a string, an integer, or a tuple of those) into an [`ItemId`].
    ///
    /// Prefer a tuple over formatting a string:
    ///
    /// ```
    /// # use egui_plot::ItemId;
    /// # let (row, column) = (0, 0);
    /// let good = ItemId::new(("my_cell", row, column)); // No allocation
    /// let bad = ItemId::new(format!("my_cell {row} {column}")); // Allocates
    /// # let _ = (good, bad);
    /// ```
    pub fn new(source: impl Hash) -> Self {
        let (a, b, c, d) = HASH_SEEDS;
        let hash = ahash::RandomState::with_seeds(a, b, c, d).hash_one(source);
        Self(NonZeroU64::new(hash).unwrap_or(NonZeroU64::MIN)) // The hash was exactly zero (very bad luck)
    }

    /// The inner value, which is a high-entropy hash.
    #[inline(always)]
    pub fn value(self) -> u64 {
        self.0.get()
    }
}

impl core::fmt::Debug for ItemId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ItemId({:04X})", self.value() as u16)
    }
}
