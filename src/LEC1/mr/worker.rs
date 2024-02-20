/// Go: map function return a slice of KeyValue
/// Rust: returns a vector with owned KeyValue
pub struct KeyValue {
    /// Key of this pair
    pub key: String,
    /// Value of this pair
    pub value: String,
}
