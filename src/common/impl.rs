use super::{r#trait::Character, r#type::CharacterSet};

impl<'a> Character<'a> for CharacterSet<'a> {
    fn set(list: &'a Vec<&'a str>) {}
    fn update(list: &'a Vec<&'a str>) {}
}
