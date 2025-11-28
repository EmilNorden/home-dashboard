use crate::display::qr_component::QrMatrix;
use buoyant::primitives::geometry::Rectangle;
use buoyant::primitives::{Dimensions, Interpolate, Point, Size};
use buoyant::render::{AnimatedJoin, AnimationDomain, Render};
use buoyant::render_target::{RenderTarget, SolidBrush};
use embedded_graphics::prelude::PixelColor;

#[derive(Clone)]
pub struct Qr<C: PixelColor, Q: QrMatrix> {
    pub origin: Point,
    pub size: Dimensions,
    pub qr: Q,
    pub dark: C,
    pub light: C,
    pub quiet_zone_modules: u32,
}

impl<C, Q: QrMatrix> AnimatedJoin for Qr<C, Q>
where
    C: Clone + Interpolate + PixelColor,
{
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        todo!()
    }
}

impl<C, Q: QrMatrix> Render<C> for Qr<C, Q>
where
    C: PixelColor + Interpolate + Clone,
{
    fn render(&self, rt: &mut impl RenderTarget<ColorFormat = C>, style: &C, offset: Point) {
        //let mut origin = self.origin + offset;
        //origin.x -= origin.x.rem_euclid(8);
        //log::info!("HERE IS MY ORIGIN =========> {:?}", origin);
        log::info!("HERE IS MY SIZE ==========> {:?}", self.size);
        rt.fill(
            offset,
            &SolidBrush::new(self.dark),
            None,
            &Rectangle {
                origin: Point::zero(),
                size: Size::from(self.size),
            },
        );

        let modules = self.qr.size();
        let modules_with_quiet_zone = modules as u32 + (self.quiet_zone_modules * 2);
        let module_size = self.size.width.0 / modules_with_quiet_zone;
        log::info!(
            "Size is {:?} modules+quiet is {} so module_size is {}",
            self.size.width.0,
            modules_with_quiet_zone,
            module_size
        );

        let quiet_zone_offset = (self.quiet_zone_modules * module_size) as i32;

        let dark = SolidBrush::new(self.dark.clone());

        /*for y in 0..modules {
            for x in 0..modules {
                if self.qr.get_module(x, y) {
                    let point = Point::new(
                        quiet_zone_offset + (x * module_size),
                        quiet_zone_offset + (y * module_size),
                    );
                    rt.fill(
                        point + origin,
                        &dark,
                        None,
                        &Rectangle::new(
                            Point::zero(),
                            Size::new(self.module_px as u32, self.module_px as u32),
                        ),
                    );
                }
            }
        }*/
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        todo!()
    }
}
