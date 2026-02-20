use whiskers::prelude::{
    Color, Draw, Transforms, Unit, Widget,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Widget)]
pub struct Book {
    thickness: f64,
    width: f64,
    height: f64,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            thickness: 50.0,
            width: 160.0,
            height: 210.0,
        }
    }
}

impl Book {
    pub(crate) fn update(
        &self,
        sketch: &mut whiskers::Sketch,
        _ctx: &mut whiskers::Context<'_>,
    ) -> anyhow::Result<()> {
        let page_w = sketch.width() / Unit::Mm.to_px::<f64>();
        let page_h = sketch.height() / Unit::Mm.to_px::<f64>();

        sketch.color(Color::DARK_RED).stroke_width(0.5);
        sketch
            .translate(page_w / 2.0, page_h / 2.0)
            .rect(0., 0., self.width, self.height);

        Ok(())
    }
}
