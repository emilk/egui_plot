/// Placement of the horizontal X-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VPlacement {
    Top,
    Bottom,
}

/// Placement of the vertical Y-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HPlacement {
    Left,
    Right,
}

/// Placement of an axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// Bottom for X-axis, or left for Y-axis.
    LeftBottom,

    /// Top for x-axis and right for y-axis.
    RightTop,
}

impl From<HPlacement> for Placement {
    #[inline]
    fn from(placement: HPlacement) -> Self {
        match placement {
            HPlacement::Left => Self::LeftBottom,
            HPlacement::Right => Self::RightTop,
        }
    }
}

impl From<Placement> for HPlacement {
    #[inline]
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::LeftBottom => Self::Left,
            Placement::RightTop => Self::Right,
        }
    }
}

impl From<VPlacement> for Placement {
    #[inline]
    fn from(placement: VPlacement) -> Self {
        match placement {
            VPlacement::Top => Self::RightTop,
            VPlacement::Bottom => Self::LeftBottom,
        }
    }
}

impl From<Placement> for VPlacement {
    #[inline]
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::LeftBottom => Self::Bottom,
            Placement::RightTop => Self::Top,
        }
    }
}

/// Where to place the plot legend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Corner {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

impl Corner {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::LeftTop, Self::RightTop, Self::LeftBottom, Self::RightBottom]
            .iter()
            .copied()
    }
}
