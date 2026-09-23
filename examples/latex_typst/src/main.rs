use typst::diag::{FileError, FileResult, SourceDiagnostic, SourceResult};
use typst::ecow::EcoVec;
use typst::foundations::{Bytes, Datetime, Smart};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source, Span};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

struct SimpleWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    source: Source,
}

impl SimpleWorld {
    fn new(text: &str) -> Self {
        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data.to_vec())))
            .collect();

        assert!(!fonts.is_empty(), "typst-assets failed to load any fonts");

        let input = format!("#set page(width: auto, height: auto, margin: 0cm)\n{text}");

        Self {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(FontBook::from_fonts(&fonts)),
            fonts,
            source: Source::detached(input),
        }
    }

    fn get_svg(&self) -> SourceResult<String> {
        let mut page = typst::compile::<PagedDocument>(&self)
            .output?
            .pages
            .first()
            .ok_or_else(|| EcoVec::from_iter([SourceDiagnostic::error(Span::detached(), "document contains no pages")]))
            .cloned()?;
        page.fill = Smart::Custom(None);
        Ok(typst_svg::svg(&page))
    }
}

impl World for SimpleWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

fn main() {
    println!("{}", SimpleWorld::new("$ f(x) = x^2 $").get_svg().unwrap());
}
