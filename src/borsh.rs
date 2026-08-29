use super::SmallVec;
use borsh::{
    io::{Result as Serial, Write},
    BorshSerialize,
};

impl<Type: BorshSerialize, const CAPACITY: usize> BorshSerialize for SmallVec<Type, CAPACITY> {
    fn serialize<Writer: Write>(&self, writer: &mut Writer) -> Serial<()> {
        self.len.0.serialize(writer)?;
        for element in self {
            element.serialize(writer)?;
        }
        return Ok(());
    }
}
