use super::SmallVec;
use borsh::{
    BorshSerialize,
    io::{
        Write,
        Result as Serial
    }
};

impl<T: BorshSerialize, const N: usize> BorshSerialize for SmallVec<T, N> {
    fn serialize<W: Write>(&self, writer: &mut W) -> Serial<()> {
        self.len.0.serialize(writer)?;
        for element in self {
            element.serialize(writer)?;
        };
        return Ok(());
    }
}