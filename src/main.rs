use whiskers::prelude::*;

mod book;

#[sketch_app]
struct BookSlipcaseSketch {
    book: book::Book,
}

impl Default for BookSlipcaseSketch {
    fn default() -> Self {
        Self {
            book: book::Book::default(),
        }
    }
}

impl App for BookSlipcaseSketch {
    fn update(&mut self, sketch: &mut Sketch, ctx: &mut Context) -> anyhow::Result<()> {
        sketch.scale(Unit::Mm);
        self.book.update(sketch, ctx)?;
        Ok(())
    }
}

fn main() -> Result {
    BookSlipcaseSketch::runner()
        .with_page_size_options(PageSize::A3H)
        .run()
}
