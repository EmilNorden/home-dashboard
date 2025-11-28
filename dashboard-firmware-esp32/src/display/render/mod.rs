use buoyant::primitives::geometry::Rectangle;
use buoyant::primitives::{Point, Size};
use buoyant::render::{AnimatedJoin, AnimationDomain, Render};
use buoyant::render_target::RenderTarget;
use epd_waveshare::color::Color;

pub mod qr;

// Minimal render node that only fills a rect using Buoyant's RenderTarget.
pub struct BigRect {
    pub origin: Point,
    pub size: Size,
    pub color: Color,
}

impl AnimatedJoin for BigRect {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        todo!()
    }
}

impl Render<Color> for BigRect {
    fn render(&self, rt: &mut impl RenderTarget<ColorFormat = Color>, _style: &Color, _off: Point) {
        use buoyant::render_target::SolidBrush;
        rt.fill(
            self.origin,
            &SolidBrush::new(self.color),
            None,
            &Rectangle {
                origin: self.origin,
                size: self.size,
            },
        );
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        todo!()
    }
}
