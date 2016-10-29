
use std::cmp::Ordering;

/// 11-bit ID mask
const SHORT_MASK: u16 = 0x7ff;
/// 29-bit ID mask
const EXTENDED_MASK: u32 = 0x1fffffff;

/// A CAN message identifier
///
/// Message identifiers can be compared and ordered by their underlying values, regardless of
/// whether they are short or extended.
#[derive(Debug, Clone)]
pub enum Id {
    /// A short CAN ID of up to 11 bits
    Short(u16),
    /// An extended CAN ID of up to 29 bits
    Extended(u32),
}

impl Id {
    /// Returns the value of this ID as a u32. If this ID is short, it is expanded.
    fn as_extended(&self) -> u32 {
        match *self {
            Id::Short(short) => short.into(),
            Id::Extended(extended) => extended,
        }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.as_extended().eq(&other.as_extended())
    }
}

impl Eq for Id { }

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_extended().partial_cmp(&other.as_extended())
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_extended().cmp(&other.as_extended())
    }
}

impl Id {
    /// Checks if this ID fits in the specified numbers of bits
    fn is_valid(&self) -> bool {
        match *self {
            Id::Short(id) => (id & !SHORT_MASK) == 0,
            Id::Extended(id) => (id & !EXTENDED_MASK) == 0,
        }
    }
}

impl From<u16> for Id {
    fn from(input: u16) -> Self {
        Id::Short(input)
    }
}
impl From<u32> for Id {
    fn from(input: u32) -> Self {
        Id::Extended(input)
    }
}

///
/// A CAN message
///
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// The message ID
    id: Id,
    /// The length of the message, up to 8
    length: u8,
    /// The data in this message
    data: [u8; 8],
}

impl Message {
    /// Creates a message with a provided ID and data
    pub fn new<I: Into<Id>>(id: I, data: &[u8]) -> Result<Self, RangeError> {
        let id = id.into();
        if data.len() <= 8 {
            if id.is_valid() {
                let mut message = Message {
                    id: id,
                    length: data.len() as u8,
                    data: [0; 8],
                };
                // Copy data
                for i in 0..data.len() {
                    message.data[i] = data[i];
                }
                Ok(message)
            } else {
                Err(RangeError::IdLength)
            }
        } else {
            Err(RangeError::DataLength)
        }
    }

    /// Creates a message with a provided short ID and data
    pub fn with_short_id(id: u16, data: &[u8]) -> Result<Self, RangeError> {
        Self::new(id, data)
    }
    /// Creates a message with a provided extended ID and data
    pub fn with_extended_id(id: u32, data: &[u8]) -> Result<Self, RangeError> {
        Self::new(id, data)
    }

    /// Returns the ID of this message
    pub fn id(&self) -> Id {
        self.id.clone()
    }

    /// Returns the length of this message
    pub fn len(&self) -> u8 {
        self.length
    }

    /// Sets the length of this message
    ///
    /// If the new length is greater than the current length, the new bytes are set to zero.
    /// Returns an error if length is greater than 8
    pub fn set_len(&mut self, length: u8) -> Result<(), RangeError> {
        if length <= 8 {
            // Fill with zeroes
            for i in self.length..length {
                self.data[usize::from(i)] = 0;
            }
            self.length = length;
            Ok(())
        } else {
            Err(RangeError::DataLength)
        }
    }

    /// Returns a reference to the data of this message
    pub fn data(&self) -> &[u8] {
        &self.data[..usize::from(self.length)]
    }
    /// Returns a mutable reference to the data of this message
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data[..usize::from(self.length)]
    }
}

quick_error! {
    /// Message length errors
    #[derive(Debug, PartialEq)]
    pub enum RangeError {
        /// A provided slice was too long, or a provided length was too great, to fit in a message
        DataLength {}
        /// A provided message ID was too long
        IdLength {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_valid_short() {
        assert!(Id::Short(0).is_valid());
    }
    #[test]
    fn test_zero_valid_extended() {
        assert!(Id::Extended(0).is_valid());
    }
    #[test]
    fn test_max_valid_short() {
        assert!(Id::Short(0b11111111111).is_valid());
    }
    #[test]
    fn test_max_valid_extended() {
        assert!(Id::Extended(0b11111111111111111111111111111).is_valid());
    }
    #[test]
    fn test_beyond_invalid_short() {
        assert!(!Id::Short(0b11111111111 + 1).is_valid());
    }
    #[test]
    fn test_beyond_invalid_extended() {
        assert!(!Id::Extended(0b11111111111111111111111111111 + 1).is_valid());
    }

    const ID: Id = Id::Short(1);

    #[test]
    fn test_data_empty() {
        let message = Message::new(ID, &[]).unwrap();
        assert_eq!(0, message.len());
        let expected_data: [u8; 0] = [];
        assert_eq!(&expected_data, message.data());
    }
    #[test]
    fn test_data_full() {
        let message = Message::new(ID, &[8, 7, 6, 5, 4, 3, 2, 1]).unwrap();
        assert_eq!(8, message.len());
        let expected_data: [u8; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(&expected_data, message.data());
    }
    #[test]
    fn test_data_too_long() {
        let message = Message::new(ID, &[8, 7, 6, 5, 4, 3, 2, 1, 0]);
        let expected: Result<Message, RangeError> = Err(RangeError::DataLength);
        assert_eq!(expected, message);
    }
}
