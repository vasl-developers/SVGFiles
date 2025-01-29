/* Do not need get_text_width() at this time, keep code around in case we need it later.
use cosmic_text::{
    Attrs, Buffer, Color, Edit, Editor, Family, FontSystem, Metrics, Shaping, SwashCache,
};
//
// Local defines.
//
use crate::defines::*;
use crate::text_field::*;

pub const TEST_FONT_SYSTEM: bool =	false;

pub struct CosmicTextContext {
	pub font_system: FontSystem,
	pub swash_cache: SwashCache,
}
//
// Gets the width of the given `text` in pixels.
//
// Arguments:
//
// * `text`: The text that should be measured.
// * `font_size`: The font size in pixels.
// * `font_family`: The optional font family.
//
// Returns:
//
// The width of the text in pixels, or [None] if the given `text` does not have a width.
//
pub fn get_text_width(ctc: &mut CosmicTextContext, text: &str, font_size: u32, font_family: Option<&str>) -> Option<u32> {
	let my_font_size: f32 = font_size as f32;

    if text == "" {
        return None;
    }

    let black = Color::rgb(0, 0, 0);

    // The line height does not matter here.
    let metrics = Metrics::new(my_font_size, my_font_size);
    let mut editor = Editor::new(Buffer::new_empty(metrics.scale(1f32)));
    let mut editor = editor.borrow_with(&mut ctc.font_system);
    editor.with_buffer_mut(|buffer| buffer.set_size(f32::INFINITY, f32::INFINITY));
    editor.with_buffer_mut(|buffer| {
        let mut attrs = Attrs::new();

        if let Some(font_family) = font_family {
            attrs = attrs.family(Family::Name(font_family));
        }

        let spans: &[(&str, Attrs)] = &[(text, attrs)];

        buffer.set_rich_text(spans.iter().copied(), attrs, Shaping::Advanced);
    });

    let mut width: Option<u32> = None;
    editor.draw(&mut ctc.swash_cache, black, black, black, |x, _, w, _, _| {
        width = if let Some(width) = width {
            Some(width.max(x as u32 + w as u32))
        } else {
            Some(w as u32 - x as u32)
        };
    });

    width
}

use std::time::{ Instant };

pub fn test_font_system_new(ctc: &mut CosmicTextContext) {
	let start = Instant::now();

	println!("'B'    -> '{0}'", get_text_width(ctc, "B",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'X'    -> '{0}'", get_text_width(ctc, "X",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'12'   -> '{0}'", get_text_width(ctc, "12",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'11'   -> '{0}'", get_text_width(ctc, "11",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'10'   -> '{0}'", get_text_width(ctc, "10",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'9'    -> '{0}'", get_text_width(ctc, "9",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'8'    -> '{0}'", get_text_width(ctc, "8",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'7'    -> '{0}'", get_text_width(ctc, "7",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'6'    -> '{0}'", get_text_width(ctc, "6",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'5'    -> '{0}'", get_text_width(ctc, "5",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'4'    -> '{0}'", get_text_width(ctc, "5",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'3'    -> '{0}'", get_text_width(ctc, "3",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'2'    -> '{0}'", get_text_width(ctc, "2",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'1'    -> '{0}'", get_text_width(ctc, "1",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'0'    -> '{0}'", get_text_width(ctc, "0",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'*'    -> '{0}'", get_text_width(ctc, "*",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'*X'   -> '{0}'", get_text_width(ctc, "*X",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'*B'   -> '{0}'", get_text_width(ctc, "*B",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'12*'  -> '{0}'", get_text_width(ctc, "12*",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));
	println!("'B 11' -> '{0}'", get_text_width(ctc, "B 11",	BREAKDOWN_FONTS[FONT_NORMAL][FA_SIZE], Some(&FONT_MAIN.to_string())).expect("REASON"));

    let duration = start.elapsed();

    println!("Time elapsed in test_font_system_old() is: {:?}", duration);
}

pub fn initialize_cosmic_text() -> CosmicTextContext {
	let font_system = FontSystem::new();
    let swash_cache = SwashCache::new();
    let mut ctc = CosmicTextContext { font_system, swash_cache };

	if TEST_FONT_SYSTEM {
		test_font_system_new(&mut ctc);
	}

	return ctc;
}
Do not need get_text_width() at this time, keep code around in case we need it later. */