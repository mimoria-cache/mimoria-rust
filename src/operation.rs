#[derive(PartialEq, Debug)]
pub enum Operation {
    /// Used to authenticate with a password before further operations are accepted.
    Login = 0,
    /// Gets a string value based on a key.
    GetString = 1,
    /// Sets a string value for a key.
    SetString = 2,
    /// Sets a binary serialized object value for a key.
    SetObjectBinary = 3,
    /// Gets a binary serialized object value based on a key.
    GetObjectBinary = 4,
    /// Gets the list stored at a key.
    GetList = 5,
    /// Adds an element to the list stored at a key.
    AddList = 6,
    /// Removes an element from a list stored at a key.
    RemoveList = 7,
    /// Gets a byte flag indicating if a value in a list exists.
    ContainsList = 8,
    /// Gets a byte flag indicating if a key exists.
    Exists = 9,
    /// Deletes the value stored at a key.
    Delete = 10,
    /// Gets the server stats which contains information about uptime, connections and cache details (size, hits, misses, hit ratio).
    GetStats = 11,
    /// Gets a byte array based on a key.
    GetBytes = 12,
    /// Sets a byte array for a key.
    SetBytes = 13,
    /// Sets a counter value for a key.
    SetCounter = 14,
    /// Increments a counter by a given value for a key.
    IncrementCounter = 15,
    /// Executes multiple operations in one request.
    Bulk = 16,
    /// Gets a map value of a map under the given key.
    GetMapValue = 17,
    /// Sets a map value of a map under the given key.
    SetMapValue = 18,
    /// Gets the entire map under the given key.
    GetMap = 19,
    /// Sets the entire map under the given key.
    SetMap = 20,
    /// Subscribes to a channel.
    Subscribe = 21,
    /// Unsubscribes from a channel.
    Unsubscribe = 22,
    /// Published a payload to a channel.
    Publish = 23,
}

impl From<u8> for Operation {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Operation::Login,
            1 => Operation::GetString,
            2 => Operation::SetString,
            3 => Operation::SetObjectBinary,
            4 => Operation::GetObjectBinary,
            5 => Operation::GetList,
            6 => Operation::AddList,
            7 => Operation::RemoveList,
            8 => Operation::ContainsList,
            9 => Operation::Exists,
            10 => Operation::Delete,
            11 => Operation::GetStats,
            12 => Operation::GetBytes,
            13 => Operation::SetBytes,
            14 => Operation::SetCounter,
            15 => Operation::IncrementCounter,
            16 => Operation::Bulk,
            17 => Operation::GetMapValue,
            18 => Operation::SetMapValue,
            19 => Operation::GetMap,
            20 => Operation::SetMap,
            21 => Operation::Subscribe,
            22 => Operation::Unsubscribe,
            23 => Operation::Publish,
            _ => panic!("Invalid byte '{}' for Operation", byte),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_from_u8_valid() {
        assert_eq!(Operation::from(0), Operation::Login);
        assert_eq!(Operation::from(1), Operation::GetString);
        assert_eq!(Operation::from(2), Operation::SetString);
        assert_eq!(Operation::from(3), Operation::SetObjectBinary);
        assert_eq!(Operation::from(4), Operation::GetObjectBinary);
        assert_eq!(Operation::from(5), Operation::GetList);
        assert_eq!(Operation::from(6), Operation::AddList);
        assert_eq!(Operation::from(7), Operation::RemoveList);
        assert_eq!(Operation::from(8), Operation::ContainsList);
        assert_eq!(Operation::from(9), Operation::Exists);
        assert_eq!(Operation::from(10), Operation::Delete);
        assert_eq!(Operation::from(11), Operation::GetStats);
        assert_eq!(Operation::from(12), Operation::GetBytes);
        assert_eq!(Operation::from(13), Operation::SetBytes);
        assert_eq!(Operation::from(14), Operation::SetCounter);
        assert_eq!(Operation::from(15), Operation::IncrementCounter);
        assert_eq!(Operation::from(16), Operation::Bulk);
        assert_eq!(Operation::from(17), Operation::GetMapValue);
        assert_eq!(Operation::from(18), Operation::SetMapValue);
        assert_eq!(Operation::from(19), Operation::GetMap);
        assert_eq!(Operation::from(20), Operation::SetMap);
        assert_eq!(Operation::from(21), Operation::Subscribe);
        assert_eq!(Operation::from(22), Operation::Unsubscribe);
        assert_eq!(Operation::from(23), Operation::Publish);
    }

    #[test]
    #[should_panic(expected = "Invalid byte '24' for Operation")]
    fn test_operation_from_u8_invalid() {
        let _ = Operation::from(24);
    }
}
