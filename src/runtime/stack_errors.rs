
pub struct SlotDoesNotExistError {
    pub(crate) stack_len: usize,
    pub(crate) slot: usize
}
