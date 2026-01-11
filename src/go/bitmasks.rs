use crate::go::coordinate::Coordinate;

pub const fn lhs() -> [u64; 6] {
    let mut out = [0; 6];
    let mut i = 0u16;
    while i < 19 {
        let index = {
            let index = i * 19;
            debug_assert!(index < (19 * 19));
            Coordinate { index }
        };
        let (a, b) = {
            let this = &index;
            ((this.index / 64) as usize, (this.index % 64) as usize)
        };
        out[a] |= 1 << b;
        i += 1;
    }
    out
}

pub const fn rhs() -> [u64; 6] {
    let mut out = [0; 6];
    let mut i = 1u16;
    while i < 20 {
        let index = {
            let index = i * 19 - 1;
            debug_assert!(index < (19 * 19));
            Coordinate { index }
        };
        let (a, b) = {
            let this = &index;
            ((this.index / 64) as usize, (this.index % 64) as usize)
        };
        out[a] |= 1 << b;
        i += 1;
    }
    out
}

pub const fn filled() -> [u64; 6] {
    let mut out = [0; 6];
    let mut i = 0u16;
    while i < 19 {
        let mut j = 0u16;
        while j < 19 {
            let index = {
                let index = i * 19 + j;
                debug_assert!(index < (19 * 19));
                Coordinate { index }
            };
            let (a, b) = {
                let this = &index;
                ((this.index / 64) as usize, (this.index % 64) as usize)
            };
            out[a] |= 1 << b;
            j += 1;
        }
        i += 1;
    }
    out
}
