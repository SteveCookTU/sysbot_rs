pub(crate) struct ThreadMessage {
    pub returns: bool,
    pub message: String,
    pub close: bool,
    pub size: usize,
}
