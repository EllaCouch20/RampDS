use ggez::graphics::{Canvas, Rect};
use ggez::Context;
use ggez::glam::Vec2;
use crate::traits::Drawable;
use either::Either;

pub fn min(x: f32, y: f32) -> f32 {
    std::cmp::min(x as i32, y as i32) as f32
}

pub fn max(x: f32, y: f32) -> f32 {
    std::cmp::max(x as i32, y as i32) as f32
}

#[derive(Clone, Debug)]
pub struct Child(Either<Box<dyn Drawable>, Component>, Rect);

impl Child {
    pub fn size(&self, ctx: &Context) -> Vec2 {
        match &self.0 {
            Either::Left(drawable) => drawable.size(ctx),
            Either::Right(component) => component.size(ctx)
        }
    }

    pub fn offset(&self) -> Vec2 {Vec2::new(self.1.x, self.1.y)}

    pub fn draw(&self, canvas: &mut Canvas, bound: Rect) {
        println!("bound: {:?}", bound);
        println!("offset: {:?}", self.1);
        let bound = Rect::new(
            bound.x+self.1.x, bound.y+self.1.y,
            min(bound.w-self.1.x, self.1.w), min(bound.h-self.1.y, self.1.h)
        );

        match &self.0 {
            Either::Left(drawable) => drawable.draw(canvas, bound),
            Either::Right(component) => component.draw(canvas, bound),
        }
    }
}

impl<V: Drawable + 'static> From<(V, Rect)> for Child {
    fn from(v: (V, Rect)) -> Self {
        Child(Either::Left(Box::new(v.0)), v.1)
    }
}

impl From<(Component, Rect)> for Child {
    fn from(v: (Component, Rect)) -> Self {
        Child(Either::Right(v.0), v.1)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Component(Vec<Child>);

impl Component {
    pub fn new() -> Self {
        Component(Vec::default())
    }

    pub fn size(&self, ctx: &Context) -> Vec2 {
        self.0.iter().fold(Vec2::new(0.0, 0.0), |old_size, c| {
            let size = c.size(ctx);
            let offset = c.offset();
            Vec2::new(max(old_size.x, size.x+offset.x), max(old_size.y, size.y+offset.y))
        })
    }

    pub fn draw(&self, canvas: &mut Canvas, bound: Rect) {
        for child in &self.0 {
            child.draw(canvas, bound);
        }
    }
}

impl<V: Into<Child>> From<Vec<V>> for Component {
    fn from(v: Vec<V>) -> Self {
        Component(v.into_iter().map(|v| v.into()).collect())
    }
}