use super::SmallVec;
use borsh::{
    BorshSerialize,
    io::{
        Write,
        Result as Serial
    }
};

impl<Type: BorshSerialize, const N: usize> BorshSerialize for SmallVec<Type, N> {
    fn serialize<Writer: Write>(&self, writer: &mut Writer) -> Serial<()> {
        self.len.0.serialize(writer)?;
        for element in self {
            element.serialize(writer)?;
        };
        return Ok(());
    }
}