use whiskers::prelude::{
    Color, Draw, Transforms, Widget,
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
            height: 270.0,
        }
    }
}

impl Book {
    pub(crate) fn upate(
        &self,
        sketch: &mut whiskers::Sketch,
        _ctx: &mut whiskers::Context<'_>,
    ) -> anyhow::Result<()> {
        sketch.color(Color::DARK_RED).stroke_width(3.0);
        sketch
            .translate(sketch.width() / 2.0, sketch.height() / 2.0)
            .rect(0., 0., self.width, self.height);

        Ok(())
    }
}
