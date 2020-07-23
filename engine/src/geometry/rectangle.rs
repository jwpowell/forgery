pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Rectangle {
        Rectangle { width, height }
    }
}
