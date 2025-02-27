use typemap::Key;

/// a struct for storing different info, currently only stores one info
#[derive(Clone)]
pub struct Status {
    pub popup_opened: bool,
    /// to prevent rerendering the same image
    pub render_image: bool,
}

impl Key for Status {
    type Value = Self;
}

impl Default for Status {
    fn default() -> Self {
        Self {
            popup_opened: false,
            render_image: true,
        }
    }
}

impl Status {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
