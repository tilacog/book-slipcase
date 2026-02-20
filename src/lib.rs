use whiskers::prelude::*;

pub mod book;

#[sketch_app]
#[derive(Default)]
pub struct BookSlipcaseSketch {
    book: book::Book,
}

impl App for BookSlipcaseSketch {
    fn update(&mut self, sketch: &mut Sketch, ctx: &mut Context) -> anyhow::Result<()> {
        sketch.scale(Unit::Mm);
        self.book.update(sketch, ctx)?;
        Ok(())
    }
}

wasm_sketch!(BookSlipcaseSketch::runner().with_page_size_options(PageSize::A3H));
