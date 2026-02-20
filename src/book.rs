use whiskers::{
    Sketch,
    prelude::{
        Color, Draw, Transforms, Unit, Widget,
        serde::{Deserialize, Serialize},
    },
};

#[derive(Deserialize, Serialize, Widget)]
pub struct Book {
    thickness: f64,
    width: f64,
    height: f64,
}

impl Default for Book {
    fn default() -> Self {
        // fall of hyperion
        Self {
            thickness: 36.0,
            width: 140.0,
            height: 217.0,
        }
    }
}

impl Book {
    pub(crate) fn update(
        &self,
        sketch: &mut Sketch,
        ctx: &mut whiskers::Context<'_>,
    ) -> anyhow::Result<()> {
        sketch.color(Color::DARK_RED).stroke_width(2.5);

        translate_to_page_center(sketch);
        self.draw_book_spine(sketch, ctx);
        self.draw_book_side(sketch, ctx);
        self.draw_spine_flaps(sketch, ctx);

        // TODO: draw_top_flaps()
        // TODO: draw_side_flaps()

        Ok(())
    }

    fn draw_book_spine(&self, sketch: &mut Sketch, _ctx: &mut whiskers::Context<'_>) {
        sketch.rect(0., 0., self.thickness, self.height);
    }

    fn draw_book_side(&self, sketch: &mut Sketch, _ctx: &mut whiskers::Context<'_>) {
        let side_center_point = self.thickness.half() + self.width.half();
        sketch.rect(side_center_point, 0., self.width, self.height);
        sketch.rect(-side_center_point, 0., self.width, self.height);
    }

    fn draw_spine_flaps(&self, sketch: &mut Sketch, _ctx: &mut whiskers::Context<'_>) {
        let flap_center_point = self.height.half() + self.thickness.half();
        sketch.rect(0, flap_center_point, self.thickness, self.thickness);
        sketch.rect(0, -flap_center_point, self.thickness, self.thickness);
    }
}

fn page_size(sketch: &Sketch) -> (f64, f64) {
    let page_w = sketch.width() / Unit::Mm.to_px::<f64>();
    let page_h = sketch.height() / Unit::Mm.to_px::<f64>();
    (page_w, page_h)
}

fn page_center(sketch: &Sketch) -> (f64, f64) {
    let (page_w, page_h) = page_size(sketch);
    (page_w.half(), page_h.half())
}
fn translate_to_page_center(sketch: &mut Sketch) {
    let (dx, dy) = page_center(sketch);
    sketch.translate(dx, dy);
}

trait Half {
    fn half(&self) -> Self;
}
impl Half for f64 {
    fn half(&self) -> f64 {
        self / 2.0
    }
}
