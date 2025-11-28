use crate::display::qr_component::QrMatrix;
use buoyant::environment::LayoutEnvironment;
use buoyant::layout::{Layout, ResolvedLayout};
use buoyant::primitives::ProposedDimensions;
use buoyant::render::Renderable;
use embedded_graphics::prelude::*;

#[derive(Clone, Copy)]
pub struct Qr<C: PixelColor, Q: QrMatrix> {
    pub qr: Q,
    pub dark: C,
    pub light: C,
    pub quiet_zone_modules: u32,
}

impl<C: PixelColor, Q: QrMatrix> Layout for Qr<C, Q> {
    type Sublayout = ();
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let mut min_dimension = offer.width.min(offer.height).resolve_most_flexible(0, 1).0;
        let modules = self.qr.size() as u32 + (self.quiet_zone_modules * 2);
        min_dimension -= min_dimension.rem_euclid(modules);

        /*let modules = self.qr.size() as u32;
        let want_side = (modules + (self.quiet_zone_modules as u32) * 2) * (self.module_px as u32);
        let side = offer.resolve_most_flexible(0, want_side);
        let smallest_side = side.height.0.min(side.width.0).min(want_side);

        log::info!(
            "QR ===> modules: {}, Want_side: {:?}, side: {:?}",
            modules,
            want_side,
            side
        );*/
        log::info!("QR min dimension: {:?}", min_dimension);
        //let side = core::cmp::min(want_side, core::cmp::min(offer.max.width, offer.max.height));
        buoyant::layout::ResolvedLayout {
            sublayouts: (),
            resolved_size: buoyant::primitives::Dimensions::new(min_dimension, min_dimension),
        }
    }

    fn priority(&self) -> i8 {
        1
    }
}

impl<C: PixelColor, Q: QrMatrix> Renderable for Qr<C, Q> {
    type Renderables = crate::display::render::qr::Qr<C, Q>;

    fn render_tree(
        &self,
        layout: &buoyant::layout::ResolvedLayout<Self::Sublayout>,
        origin: buoyant::primitives::Point,
        _env: &impl buoyant::environment::LayoutEnvironment,
    ) -> Self::Renderables {
        log::info!("QR size: {:?}", layout.resolved_size);
        log::info!("QR Origin: {:?}", origin);
        crate::display::render::qr::Qr {
            origin,
            size: layout.resolved_size,
            qr: self.qr.clone(), // &'a dyn QrMatrix flows through
            dark: self.dark,
            light: self.light,
            quiet_zone_modules: self.quiet_zone_modules,
        }
    }
}
