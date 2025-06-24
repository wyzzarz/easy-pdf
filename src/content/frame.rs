// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{ContentId, Layout};
use crate::geometry::Rect;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    id: ContentId,
    frame: Rect,
}

impl Layout for Frame {

    fn new(id: ContentId, frame: Rect) -> Self where Self: Sized {
        Self {
            id, 
            frame
        }
    }

    fn get_id(&self) -> ContentId {
        self.id
    }

    fn frame(&self) -> Rect {
        self.frame.clone()
    }

    fn set_frame(&mut self, rect: Rect) {
        self.frame = rect;
    }

}
