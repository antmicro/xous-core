use std::collections::HashMap;

use crate::{Canvas, GlyphStyle};

use graphics_server::*;

use crate::{LayoutApi, LayoutBehavior};

use crate::contexts::MISC_CONTEXT_DEFAULT_TRUST;
const TRUST_OFFSET: u8 = 2;

#[derive(Debug, Copy, Clone)]
pub(crate) struct ChatLayout {
    // a set of GIDs to track the elements of the chat layout
    pub content: Gid,
    pub predictive: Gid,
    pub input: Gid,

    // my internal bookkeeping records. Allow input area to grow into content area
    min_content_height: i16,
    min_input_height: i16,
    screensize: Point,
    _small_height: i16,
    _regular_height: i16,
    visible: bool,
}
impl ChatLayout {
    // pass in the status canvas so we can size around it, but we can't draw on it
    pub fn init(gfx: &graphics_server::Gfx, trng: &trng::Trng,
        status_canvas: &Canvas, canvases: &mut HashMap<Gid, Canvas>) -> Result<ChatLayout, xous::Error> {
        let screensize = gfx.screen_size().expect("Couldn't get screen size");
        // get the height of various text regions to compute the layout
        let small_height: i16 = gfx.glyph_height_hint(GlyphStyle::Small).expect("couldn't get glyph height") as i16;
        let regular_height: i16 = gfx.glyph_height_hint(GlyphStyle::Regular).expect("couldn't get glyph height") as i16;
        let margin = 4;

        // allocate canvases in structures, and record their GID for future reference
        // base trust - 2 so that main menu + status bar always ride on top
        let predictive_canvas = Canvas::new(
            Rectangle::new_coords(0, screensize.y - regular_height - margin*2, screensize.x, screensize.y),
            MISC_CONTEXT_DEFAULT_TRUST - TRUST_OFFSET,
            &trng, None, crate::api::CanvasType::ChatPreditive
        ).expect("couldn't create predictive text canvas");
        canvases.insert(predictive_canvas.gid(), predictive_canvas);

        let min_input_height = regular_height + margin*2;
        let input_canvas = Canvas::new(
            Rectangle::new_v_stack(predictive_canvas.clip_rect(), -min_input_height),
            MISC_CONTEXT_DEFAULT_TRUST - TRUST_OFFSET, &trng, None, crate::api::CanvasType::ChatInput
        ).expect("couldn't create input text canvas");
        canvases.insert(input_canvas.gid(), input_canvas);

        let content_canvas = Canvas::new(
            Rectangle::new_v_span(status_canvas.clip_rect(), input_canvas.clip_rect()),
            (MISC_CONTEXT_DEFAULT_TRUST - TRUST_OFFSET) / 2, &trng, None, crate::api::CanvasType::ChatContent
        ).expect("couldn't create content canvas");
        canvases.insert(content_canvas.gid(), content_canvas);

        Ok(ChatLayout {
            content: content_canvas.gid(),
            predictive: predictive_canvas.gid(),
            input: input_canvas.gid(),
            min_content_height: 64,
            min_input_height,
            screensize,
            _small_height: small_height,
            _regular_height: regular_height,
            visible: true,
        })
    }
}
impl LayoutApi for ChatLayout {
    fn behavior(&self) -> LayoutBehavior {
        LayoutBehavior::App
    }
    fn clear(&self, gfx: &graphics_server::Gfx, canvases: &mut HashMap<Gid, Canvas>) -> Result<(), xous::Error> {
        let input_canvas = canvases.get(&self.input).expect("couldn't find input canvas");
        let content_canvas = canvases.get(&self.content).expect("couldn't find content canvas");
        let predictive_canvas = canvases.get(&self.predictive).expect("couldn't find predictive canvas");

        let mut rect = content_canvas.clip_rect();
        rect.style = DrawStyle {fill_color: Some(PixelColor::Light), stroke_color: None, stroke_width: 0,};
        gfx.draw_rectangle(rect).expect("can't clear canvas");

        let mut rect = predictive_canvas.clip_rect();
        rect.style = DrawStyle {fill_color: Some(PixelColor::Light), stroke_color: None, stroke_width: 0,};
        gfx.draw_rectangle(rect).expect("can't clear canvas");

        let mut rect = input_canvas.clip_rect();
        rect.style = DrawStyle {fill_color: Some(PixelColor::Light), stroke_color: None, stroke_width: 0,};
        gfx.draw_rectangle(rect).expect("can't clear canvas");
        Ok(())
    }
    fn resize_height(&mut self, gfx: &graphics_server::Gfx, new_height: i16, status_canvas: &Canvas, canvases: &mut HashMap<Gid, Canvas>) -> Result<Point, xous::Error> {
        let input_canvas = canvases.get(&self.input).expect("couldn't find input canvas");
        let predictive_canvas = canvases.get(&self.predictive).expect("couldn't find predictive canvas");

        let height: i16 = if new_height < self.min_input_height {
            self.min_input_height
        } else {
            new_height
        };
        let mut new_input_rect = Rectangle::new_v_stack(predictive_canvas.clip_rect(), -height);
        let mut new_content_rect = Rectangle::new_v_span(status_canvas.clip_rect(), new_input_rect);
        if (new_content_rect.br.y - new_content_rect.tl.y) > self.min_content_height {
            {
                let input_canvas_mut = canvases.get_mut(&self.input).expect("couldn't find input canvas");
                input_canvas_mut.set_clip(new_input_rect);
                new_input_rect.style = DrawStyle {fill_color: Some(PixelColor::Light), stroke_color: None, stroke_width: 0,};
                gfx.draw_rectangle(new_input_rect).expect("can't clear canvas");
                    }
            {
                let content_canvas_mut = canvases.get_mut(&self.content).expect("couldn't find content canvas");
                content_canvas_mut.set_clip(new_content_rect);
                new_content_rect.style = DrawStyle {fill_color: Some(PixelColor::Light), stroke_color: None, stroke_width: 0,};
                gfx.draw_rectangle(new_content_rect).expect("can't clear canvas");
            }
            // we resized to this new height
            Ok(new_content_rect.br)
        } else {
            // we didn't resize anything, height unchanged
            Ok(input_canvas.clip_rect().br)
        }
    }
    fn get_gids(&self) ->Vec<crate::api::GidRecord> {
        vec![
            crate::api::GidRecord {
                gid: self.content,
                canvas_type: crate::api::CanvasType::ChatContent
            },
            crate::api::GidRecord {
                gid: self.input,
                canvas_type: crate::api::CanvasType::ChatInput
            },
            crate::api::GidRecord {
                gid: self.predictive,
                canvas_type: crate::api::CanvasType::ChatPreditive
            },
        ]
    }
    fn set_visibility_state(&mut self, onscreen: bool, canvases: &mut HashMap<Gid, Canvas>) {
        log::debug!("request modal to onscreen {} from self.visible {}", onscreen, self.visible);
        if onscreen == self.visible {
            log::trace!("chatlayout: no change to visibility, moving on");
            // nothing to do
            return
        }
        let offscreen = if !onscreen && self.visible {
            // move canvases off-screen
            Point::new(self.screensize.x*2, 0)
        } else if onscreen && !self.visible {
            // undo the off-screen move
            Point::new(-self.screensize.x*2, 0)
        } else {
            // should actually never reach this because of the identity check at the very top
            Point::new(0, 0)
        };
        log::debug!("chatlayout: shifting canvas for input by {:?}", offscreen);
        let input_canvas = canvases.get_mut(&self.input).expect("couldn't find input canvas");
        input_canvas.set_clip(input_canvas.clip_rect().translate_chain(offscreen));

        let content_canvas = canvases.get_mut(&self.content).expect("couldn't find content canvas");
        content_canvas.set_clip(content_canvas.clip_rect().translate_chain(offscreen));

        let predictive_canvas = canvases.get_mut(&self.predictive).expect("couldn't find predictive canvas");
        predictive_canvas.set_clip(predictive_canvas.clip_rect().translate_chain(offscreen));

        self.visible = onscreen;
    }
}