use rusttype::{ Font, Scale, point, PositionedGlyph };

pub struct FontRenderer {
    font: Font<'static>,
}

impl FontRenderer {
    pub fn new(font_data: Vec<u8>) -> Self {
        let font_data = font_data.into_boxed_slice();
        let font_data: &'static [u8] = Box::leak(font_data);
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

        Self { font }
    }

    pub fn render_text(&self, text: &str, scale: f32) -> Vec<PositionedGlyph<'static>> {
        let scale = Scale::uniform(scale * 0.75);
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        self.font.layout(text, scale, offset).collect()
    }

    pub fn calculate_text_size(&self, text: &str, scale: f32) -> (f32, f32) {
        let scale = Scale::uniform(scale * 0.75);
        let v_metrics = self.font.v_metrics(scale);
        let glyphs: Vec<_> = self.font.layout(text, scale, point(0.0, v_metrics.ascent)).collect();

        if glyphs.is_empty() {
            return (0.0, 0.0);
        }

        let min_x = glyphs
            .first()
            .and_then(|g| g.pixel_bounding_box())
            .map(|bb| bb.min.x as f32)
            .unwrap_or(0.0);
        let max_x = glyphs
            .last()
            .and_then(|g| g.pixel_bounding_box())
            .map(|bb| bb.max.x as f32)
            .unwrap_or(0.0);

        let width = max_x - min_x;
        let height = v_metrics.ascent - v_metrics.descent;

        (width, height.abs())
    }
}
