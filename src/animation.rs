use raylib::{ffi::Vector2, texture::Texture2D};

pub struct Animation {
    pub origin: Vector2,
    pub frame_number: u32,
    pub frame_width: u32,
    pub texture: Texture2D,
}
