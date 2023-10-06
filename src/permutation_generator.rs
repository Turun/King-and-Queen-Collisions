use crate::NUMBER_OF_CARDS;

pub struct PermutationGenerator {
    pub pos1: u8,
    pub pos2: u8,
    pub pos3: u8,
    pub pos4: u8,
}

impl Iterator for PermutationGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos4 += 1;
        if self.pos4 < NUMBER_OF_CARDS {
            return Some(self.into());
        }

        self.pos3 += 1;
        self.pos4 = self.pos3 + 1;
        if self.pos4 < NUMBER_OF_CARDS {
            return Some(self.into());
        }

        self.pos2 += 1;
        self.pos3 = self.pos2 + 1;
        self.pos4 = self.pos3 + 1;
        if self.pos4 < NUMBER_OF_CARDS {
            return Some(self.into());
        }

        self.pos1 += 1;
        self.pos2 = self.pos1 + 1;
        self.pos3 = self.pos2 + 1;
        self.pos4 = self.pos3 + 1;
        if self.pos4 < NUMBER_OF_CARDS {
            return Some(self.into());
        }

        None
    }
}
impl Default for PermutationGenerator {
    fn default() -> Self {
        Self {
            pos1: 0,
            pos2: 1,
            pos3: 2,
            pos4: 3,
        }
    }
}

impl From<&PermutationGenerator> for u64 {
    fn from(val: &PermutationGenerator) -> Self {
        let mut re = 0u64;
        re |= 1 << val.pos1;
        re |= 1 << val.pos2;
        re |= 1 << val.pos3;
        re |= 1 << val.pos4;
        re
    }
}
impl From<PermutationGenerator> for u64 {
    fn from(val: PermutationGenerator) -> Self {
        let mut re = 0u64;
        re |= 1 << val.pos1;
        re |= 1 << val.pos2;
        re |= 1 << val.pos3;
        re |= 1 << val.pos4;
        re
    }
}
impl From<&mut PermutationGenerator> for u64 {
    fn from(val: &mut PermutationGenerator) -> Self {
        let mut re = 0u64;
        re |= 1 << val.pos1;
        re |= 1 << val.pos2;
        re |= 1 << val.pos3;
        re |= 1 << val.pos4;
        re
    }
}
