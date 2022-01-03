use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscalatorLocation(pub [TileGridCoord; 2]);

impl EscalatorLocation {
    pub fn rotate(&mut self, spin: SpinDirection) {
        for point in self.0.iter_mut() {
            point.rotate(spin);
        }
    }

    /// If self contains `coord`, returns `Some(neighbor_coord)`.
    /// Otherwise, returns `None`.
    pub fn coord_neighbor(&self, coord: TileGridCoord) -> Option<TileGridCoord> {
        if self.0[0] == coord {
            Some(self.0[1])
        } else if self.0[1] == coord {
            Some(self.0[0])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neighbor() {
        let a = TileGridCoord { x: 0, y: 1 };
        let b = TileGridCoord { x: 2, y: 3 };
        let loc = EscalatorLocation([a, b]);

        assert_eq!(loc.coord_neighbor(a), Some(b));
        assert_eq!(loc.coord_neighbor(b), Some(a));
        assert_eq!(loc.coord_neighbor(TileGridCoord { x: 3, y: 3 }), None);
    }
}
