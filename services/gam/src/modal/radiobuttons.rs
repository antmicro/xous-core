use crate::*;

use graphics_server::api::*;

use xous_ipc::Buffer;

use core::fmt::Write;
use locales::t;

#[derive(Debug)]
pub struct RadioButtons {
    pub items: Vec::<ItemName>,
    pub action_conn: xous::CID,
    pub action_opcode: u32,
    pub action_payload: RadioButtonPayload, // the current "radio button" selection
    pub select_index: i16, // the current candidate to be selected
    pub is_password: bool,
}
impl RadioButtons {
    pub fn new(action_conn: xous::CID, action_opcode: u32) -> Self {
        RadioButtons {
            items: Vec::new(),
            action_conn,
            action_opcode,
            action_payload: RadioButtonPayload::new(""),
            select_index: 0,
            is_password: false,
        }
    }
    pub fn add_item(&mut self, new_item: ItemName) {
        if self.action_payload.as_str().len() == 0 {
            // default to the first item added
            self.action_payload = RadioButtonPayload::new(new_item.as_str());
        }
        self.items.push(new_item);
    }
    pub fn clear_items(&mut self) {
        self.items.clear();
        self.action_payload.clear();
    }
}
impl ActionApi for RadioButtons {
    fn set_action_opcode(&mut self, op: u32) {self.action_opcode = op}
    fn height(&self, glyph_height: i16, margin: i16) -> i16 {
        // total items, then +1 for the "Okay" message
        (self.items.len() as i16 + 1) * glyph_height + margin * 2 + margin * 2 + 5 // +4 for some bottom margin slop
    }
    fn redraw(&self, at_height: i16, modal: &Modal) {
        let color = if self.is_password {
            PixelColor::Light
        } else {
            PixelColor::Dark
        };

        // prime a textview with the correct general style parameters
        let mut tv = TextView::new(
            modal.canvas,
            TextBounds::BoundingBox(Rectangle::new_coords(0, 0, 1, 1))
        );
        tv.ellipsis = true;
        tv.style = modal.style;
        tv.invert = self.is_password;
        tv.draw_border= false;
        tv.margin = Point::new(0, 0,);
        tv.insertion = None;

        let cursor_x = modal.margin;
        let select_x = modal.margin + 20;
        let text_x = modal.margin + 20 + 20;

        //let mut emoji_slop = (36 - modal.line_height) / 2;
        //if emoji_slop < 0 { emoji_slop = 0; }
        let emoji_slop = 2; // tweaked for a non-emoji glyph

        let mut cur_line = 0;
        let mut do_okay = true;
        for item in self.items.iter() {
            let cur_y = at_height + cur_line * modal.line_height + modal.margin * 2;
            if cur_line == self.select_index {
                // draw the cursor
                tv.text.clear();
                tv.bounds_computed = None;
                tv.bounds_hint = TextBounds::BoundingBox(Rectangle::new(
                    Point::new(cursor_x, cur_y - emoji_slop), Point::new(cursor_x + 36, cur_y - emoji_slop + 36)
                ));
                write!(tv, "\u{25B6}").unwrap();
                modal.gam.post_textview(&mut tv).expect("couldn't post tv");
                do_okay = false;
            }
            if item.as_str() == self.action_payload.as_str() {
                // draw the radio dot
                tv.text.clear();
                tv.bounds_computed = None;
                tv.bounds_hint = TextBounds::BoundingBox(Rectangle::new(
                    Point::new(select_x, cur_y), Point::new(select_x + 36, cur_y + modal.line_height)
                ));
                write!(tv, "•").unwrap();
                modal.gam.post_textview(&mut tv).expect("couldn't post tv");
            }
            // draw the text
            tv.text.clear();
            tv.bounds_computed = None;
            tv.bounds_hint = TextBounds::BoundingBox(Rectangle::new(
                Point::new(text_x, cur_y), Point::new(modal.canvas_width - modal.margin, cur_y + modal.line_height)
            ));
            write!(tv, "{}", item.as_str()).unwrap();
            modal.gam.post_textview(&mut tv).expect("couldn't post tv");

            cur_line += 1;
        }
        cur_line += 1;
        let cur_y = at_height + cur_line * modal.line_height + modal.margin * 2;
        if do_okay {
            tv.text.clear();
            tv.bounds_computed = None;
            tv.bounds_hint = TextBounds::BoundingBox(Rectangle::new(
                Point::new(cursor_x, cur_y - emoji_slop), Point::new(cursor_x + 36, cur_y - emoji_slop + 36)
            ));
            write!(tv, "\u{25B6}").unwrap(); // right arrow emoji. use unicode numbers, because text editors do funny shit with emojis
            modal.gam.post_textview(&mut tv).expect("couldn't post tv");
        }
        // draw the "OK" line
        tv.text.clear();
        tv.bounds_computed = None;
        tv.bounds_hint = TextBounds::BoundingBox(Rectangle::new(
            Point::new(text_x, cur_y), Point::new(modal.canvas_width - modal.margin, cur_y + modal.line_height)
        ));
        write!(tv, "{}", t!("radio.select_and_close", xous::LANG)).unwrap();
        modal.gam.post_textview(&mut tv).expect("couldn't post tv");

        // divider lines
        modal.gam.draw_line(modal.canvas, Line::new_with_style(
            Point::new(modal.margin, at_height + modal.margin),
            Point::new(modal.canvas_width - modal.margin, at_height + modal.margin),
            DrawStyle::new(color, color, 1))
            ).expect("couldn't draw entry line");
    }
    fn key_action(&mut self, k: char) -> (Option<ValidatorErr>, bool) {
        log::trace!("key_action: {}", k);
        match k {
            '←' | '→' => {
                // ignore these navigation keys
            },
            '↑' => {
                if self.select_index > 0 {
                    self.select_index -= 1;
                }
            }
            '↓' => {
                if self.select_index < self.items.len() as i16 + 1 { // +1 is the "OK" button
                    self.select_index += 1;
                }
            }
            '∴' | '\u{d}' => {
                if self.select_index < self.items.len() as i16 {
                    self.action_payload = RadioButtonPayload::new(self.items[self.select_index as usize].as_str());
                } else {  // the OK button select
                    let buf = Buffer::into_buf(self.action_payload).expect("couldn't convert message to payload");
                    buf.send(self.action_conn, self.action_opcode).map(|_| ()).expect("couldn't send action message");
                    return (None, true)
                }
            }
            '\u{0}' => {
                // ignore null messages
            }
            _ => {
                // ignore text entry
            }
        }
        (None, false)
    }
}