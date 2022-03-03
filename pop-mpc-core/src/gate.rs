use std::usize;

/// Basic components of a circuit.
///
/// `id` represents the gate id.
/// `xref` and `yref` are the wire ids of the gate inputs
/// `zref` is the wire id of the gate output
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gate {
    Xor {
        id: usize,
        level: usize,
        xref: usize,
        yref: usize,
        zref: usize,
    },
    And {
        id: usize,
        level: usize,
        xref: usize,
        yref: usize,
        zref: usize,
    },
    Inv {
        id: usize,
        level: usize,
        xref: usize,
        zref: usize,
    },
}

impl Gate {
    pub fn id(&self) -> usize {
        match self {
            Self::Xor { id, .. } => *id,
            Self::And { id, .. } => *id,
            Self::Inv { id, .. } => *id,
        }
    }

    pub fn level(&self) -> usize {
        match self {
            Self::Xor { level, .. } => *level,
            Self::And { level, .. } => *level,
            Self::Inv { level, .. } => *level,
        }
    }

    pub fn set_level(&mut self, level: usize) {
        let new_level = level;
        match self {
            Self::Xor { level, .. } => *level = new_level,
            Self::And { level, .. } => *level = new_level,
            Self::Inv { level, .. } => *level = new_level,
        }
    }

    pub fn xref(&self) -> usize {
        match self {
            Self::Xor { xref, .. } => *xref,
            Self::And { xref, .. } => *xref,
            Self::Inv { xref, .. } => *xref,
        }
    }

    pub fn yref(&self) -> Option<usize> {
        match self {
            Self::Xor { yref, .. } => Some(*yref),
            Self::And { yref, .. } => Some(*yref),
            Self::Inv { .. } => None,
        }
    }

    pub fn zref(&self) -> usize {
        match self {
            Self::Xor { zref, .. } => *zref,
            Self::And { zref, .. } => *zref,
            Self::Inv { zref, .. } => *zref,
        }
    }
}
