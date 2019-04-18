use glyph_brush::rusttype::{point, Font as RTFont, Scale};

use glyph_brush::{self, GlyphCalculatorBuilder, GlyphCruncher, Layout, Section};

static ROBOTO_REGULAR: &[u8] = include_bytes!("../../resources/Roboto-Regular.ttf");

pub struct TestTool {
    pub string: String,
    pub font_size: f32,
    pub origin: (f32, f32),
    pub dimensions: (f32, f32),
}

impl TestTool {
    pub fn new() -> Self {
        let text = "Lorem ipsum dolor sit amet, ferri simul omittantur eam eu, no debet doming dolorem ius. Iriure vocibus est te, natum delicata dignissim pri ea. Purto docendi definitiones no qui. Vel ridens instructior ad, vidisse percipitur et eos. Alienum ocurreret laboramus mei cu, usu ne meliore nostrum, usu tritani luptatum electram ad.";
        TestTool {
            string: text.to_string(),
            font_size: 14.0,
            origin: (0.0, 0.0),
            dimensions: (180.0, 180.0),
        }
    }

    pub fn evaluate_glyph_brush(&self) {
        let glyph_calc = GlyphCalculatorBuilder::using_font_bytes(ROBOTO_REGULAR).build();
        let mut glyph_calc = glyph_calc.cache_scope();
        let layout = Layout::default();

        let section = Section {
            screen_position: self.origin,
            text: &self.string,
            scale: Scale::uniform(self.font_size),
            bounds: self.dimensions,
            layout: layout,
            ..Section::default()
        };
        let glyphs = glyph_calc.glyphs(&section);

        for (i, glyph) in glyphs.enumerate() {
            let c = self.string[i..].chars().next().unwrap();
            log::debug!(
                "[{}: {} : {:?} ] {:?} // bb={:?}",
                i,
                c,
                glyph.id(),
                glyph.position(),
                glyph.pixel_bounding_box()
            );
        }
    }

    pub fn evaluate_rusttype(&self) {
        let font: RTFont<'static> = RTFont::from_bytes(ROBOTO_REGULAR).unwrap();
        let scale = Scale::uniform(self.font_size);
        let glyphs: Vec<_> = font.layout(&self.string, scale, point(0.0, 0.0)).collect();
        for (i, glyph) in glyphs.iter().enumerate() {
            let c = self.string[i..].chars().next().unwrap();
            log::debug!(
                "[{}: {} : {:?} ] {:?} // bb={:?}",
                i,
                c,
                glyph.id(),
                glyph.position(),
                glyph.pixel_bounding_box()
            );
        }
    }
}
