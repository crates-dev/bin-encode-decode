pub trait Character<'a> {
    fn set(list: &'a Vec<&'a str>);
    fn update(list: &'a Vec<&'a str>);
}
