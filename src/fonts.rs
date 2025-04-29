use fontdue::{
    layout::{Layout, TextStyle},
    Font, FontSettings,
};
use fontdue_sdl2::FontTexture;
use glam::Vec2;
use sdl2::pixels::Color;

use crate::CanvasResources;

pub struct FontRenderer<const N: usize> {
    pub font: [Font; N],
    pub layout: Layout<Color>,
}

impl<const N: usize> FontRenderer<N> {
    pub fn new(fonts: [Font; N]) -> Result<Self, String> {
        let layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
        Ok(FontRenderer {
            font: fonts,
            layout,
        })
    }

    pub fn render_text(
        &mut self,
        canvas_resources: &mut CanvasResources,
        text: &str,
        at: Vec2,
        font_size: f32,
        font_color: Color,
        font_index: usize,
    ) -> Result<(), String> {
        self.layout.reset(&fontdue::layout::LayoutSettings {
            x: at.x,
            y: at.y,
            ..Default::default()
        });
        self.layout.append(
            &self.font,
            &TextStyle::with_user_data(text, font_size, font_index, font_color),
        );
        let mut texture = FontTexture::new(&canvas_resources.texture_creator)?;
        texture.draw_text(
            &mut canvas_resources.canvas,
            &self.font,
            self.layout.glyphs(),
        )?;

        Ok(())
    }
}

pub fn load_fonts() -> [Font; 1] {
    let font_data = include_bytes!("../truetype/OpenSans-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font_data, FontSettings::default()).expect("Failed to load font");
    [font]
}
