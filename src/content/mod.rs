// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod frame;

pub use frame::Frame;
use crate::geometry::{POINT0, Point, Rect, Unit};

/// Content that can be layed out across one or more `Page` objects and rendered.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    /// Simple framed content.
    Frame(Frame),
}

/// Trait used to layout content.
pub trait Layout {
        
    /// Creates a new instance.
    fn new(frame: Rect) -> Self where Self: Sized;

    /// The location and size of the content with respect to its parent.
    fn frame(&self) -> Rect;

    /// Sets the location and size of the content with respect to its parent.
    fn set_frame(&mut self, rect: Rect);

    /// The size of the content in its own coordinate system with the origin at `(0, 0)`.
    fn bounds(&self) -> Rect {
        let mut rect = self.frame();
        rect.origin = POINT0;
        rect
    }

    /// Changing the size will grow/shrink the size of the frame from it's center and adjust origin accordingly.
    fn set_bounds(&mut self, rect: Rect) {
        let mut new_frame = rect.clone();
        let delta_width = &new_frame.size.width - &self.bounds().size.width;
        let delta_height = &new_frame.size.height - &self.bounds().size.height;
        new_frame.origin.x = &self.frame().origin.x - &(&delta_width / &Unit::from(2));
        new_frame.origin.y = &self.frame().origin.y - &(&delta_height / &Unit::from(2));
        self.set_frame(new_frame);
    }

    /// The center of the frame.
    fn center(&self) -> Point {
        self.frame().center()
    }

    /// Moving the center of the frame will adjust the origin of the frame.
    fn set_center(&mut self, new_center: Point) {
        let old_center = self.center();
        let delta_x = &new_center.x - &old_center.x;
        let delta_y = &new_center.y - &old_center.y;
        let mut rect = self.frame();
        rect.origin.x = &rect.origin.x + &delta_x;
        rect.origin.y = &rect.origin.y + &delta_y;
        self.set_frame(rect);
    }

}

impl Layout for Content {

    fn new(frame: Rect) -> Self {
        Content::Frame(Frame::new(frame))
    }

    fn frame(&self) -> Rect {
        match self {
            Content::Frame(frame) => frame.frame(),
        }
    }

    fn set_frame(&mut self, rect: Rect) {
        match self {
            Content::Frame(frame) => frame.set_frame(rect),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bounds() {
       let mut content = Frame::new(Rect::from((10, 20, 300, 4000)));
       assert_eq!(content.bounds(), Rect::from((0, 0, 300, 4000)));
       content.set_bounds(Rect::from((100, 200, 400, 5000)));
       assert_eq!(content.bounds(), Rect::from((0, 0, 400, 5000)));
       assert_eq!(content.frame(), Rect::from((-40, -480, 400, 5000)));
    }

    #[test]
    fn test_set_center() {
       let mut content = Frame::new(Rect::from((10, 20, 300, 4000)));
       assert_eq!(content.center(), Point::from((160, 2020)));
       content.set_center(Point::from((100, 200)));
       assert_eq!(content.frame(), Rect::from((-50, -1800, 300, 4000)));
    }

}
