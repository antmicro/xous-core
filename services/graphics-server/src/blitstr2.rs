mod blit;
pub use blit::*;
mod cliprect;
pub use cliprect::*;
pub(crate) mod fonts;
pub(crate) use fonts::*;

const LINES: usize = crate::backend::FB_LINES;
const WIDTH: usize = crate::backend::FB_WIDTH_PIXELS;
const WORDS_PER_LINE: usize = crate::backend::FB_WIDTH_WORDS;
pub type FrBuf = [u32; WORDS_PER_LINE * LINES];

// add more fonts (an example):
// https://github.com/samblenny/blitstr2/commit/bb7d4ab6a2d8913dcb520895a3c242c933413aae