//! Apply background blur to layer surfaces.

use crate::core::window::Id as SurfaceId;
use crate::core::Rectangle;
use iced_runtime::{window, Task};

pub fn blur<Message>(id: SurfaceId, rects: Option<Vec<Rectangle>>) -> Task<Message> {
    if rects.is_some() {
        window::enable_blur(id)
    } else {
        window::disable_blur(id)
    }
}
