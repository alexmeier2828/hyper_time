pub trait serializable {

    /// serialize an item into a json string
    fn serialize(&self) -> &'static str
}


