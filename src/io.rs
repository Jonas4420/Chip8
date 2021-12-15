mod screen;

pub use screen::Screen;

#[derive(Debug)]
pub struct IO<'a> {
    pub screen: &'a mut dyn Screen,
    pub pad: &'a [bool],
    pub audio: &'a mut bool,
}
