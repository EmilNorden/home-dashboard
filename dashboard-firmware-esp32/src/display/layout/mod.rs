use buoyant::environment::LayoutEnvironment;
use buoyant::layout::{Layout, ResolvedLayout};
use buoyant::primitives::{Dimensions, Point, ProposedDimensions, Size};
use buoyant::render::Renderable;
use epd_waveshare::color::Color;

pub mod qr;

pub struct BigRect {
    pub origin: Point,
    pub size: Size,
    pub color: Color,
}

impl Layout for BigRect {
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let x = offer.resolve_most_flexible(0, 1);
        let width = x.width.0.min(self.size.width);
        let height = x.height.0.min(self.size.height);
        log::info!("BigRect is going for {}:{}", width, height);
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions::new(width, height),
        }
    }
}

impl Renderable for BigRect {
    type Renderables = crate::display::render::BigRect;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::display::render::BigRect {
            origin,
            size: layout.resolved_size.into(),
            color: self.color,
        }
    }
}
