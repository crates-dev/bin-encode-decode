/// 字符集
pub struct CharacterSet<'a> {
    pub(crate) str: Vec<&'a str>,
    pub(crate) bin: Vec<u8>,
}
