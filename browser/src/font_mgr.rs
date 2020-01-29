use crate::types::*;
use rusttype::{point, FontCollection, PositionedGlyph, Scale};
use std::io::Write;

impl FontMgr<'_> {
    pub fn new() -> FontMgr<'static> {
        let font_data = include_bytes!("../../media/font/routed-gothic.ttf");
        let collection = FontCollection::from_bytes(font_data as &[u8]).unwrap_or_else(|e| {
                             panic!("error constructing a FontCollection from bytes: {}", e);
                         });

        let font = collection.into_font() // only succeeds if collection consists of one font
                             .unwrap_or_else(|e| {
                                 panic!("error turning FontCollection into a Font: {}", e);
                             });

        FontMgr(font)
    }

    pub fn font(&self) -> &rusttype::Font {
        &self.0
    }
}
