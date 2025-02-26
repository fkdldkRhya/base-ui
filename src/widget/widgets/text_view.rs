use crate::animation::animation::{ Animation, Vec2Animation, FadeAnimation };
use crate::style::color::Color;
use crate::widget::Widget;
use crate::graphics::Renderer;
use log::{ debug, info };
use std::sync::Arc;
use std::cell::RefCell;

pub struct TextView {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    font_size: f32,
    background_color: Color,
    text_color: Color,
    is_hovered: bool,
    hover_background_color: Color,
    hover_text_color: Color,
    on_hover: Option<Arc<RefCell<dyn FnMut(bool) + 'static>>>,
    is_pressed: bool,
    on_click: Option<Arc<RefCell<dyn FnMut() + 'static>>>,
    position_animation: Option<Vec2Animation>,
    fade_animation: Option<FadeAnimation>,
    opacity: f32,
}

impl TextView {
    pub fn new(text: &str, renderer: &Renderer) -> Self {
        debug!("Creating new TextView with text: '{}'", text);
        let background_color = Color::new(1.0, 1.0, 1.0, 0.5);
        let text_color = Color::new(0.0, 0.0, 0.0, 1.0);

        let mut tv = Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            text: text.to_string(),
            font_size: 32.0,
            background_color,
            text_color,
            is_hovered: false,
            hover_background_color: Color::new(0.9, 0.9, 0.9, 0.7),
            hover_text_color: Color::new(0.0, 0.0, 0.0, 1.0),
            on_hover: None,
            is_pressed: false,
            on_click: None,
            position_animation: None,
            fade_animation: None,
            opacity: 1.0,
        };
        tv.update_size(renderer);
        info!("TextView created with size: {}x{}", tv.width, tv.height);
        tv
    }

    pub fn with_style(
        text: String,
        font_size: f32,
        background_color: Color,
        text_color: Color,
        hover_background_color: Color,
        hover_text_color: Color,
        renderer: &Renderer
    ) -> Self {
        let mut tv = Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            text,
            font_size,
            background_color,
            text_color,
            hover_background_color,
            hover_text_color,
            is_hovered: false,
            opacity: 1.0,
            on_hover: None,
            is_pressed: false,
            on_click: None,
            position_animation: None,
            fade_animation: None,
        };
        tv.update_size(renderer);
        tv
    }

    fn update_size(&mut self, renderer: &Renderer) {
        let (width, height) = renderer
            .text_renderer()
            .font_renderer()
            .calculate_text_size(&self.text, self.font_size);
        self.width = width + 20.0; // 여백 추가
        self.height = height + 10.0;
        debug!("TextView size updated: {}x{}", self.width, self.height);
    }

    pub fn set_text(&mut self, text: &str, renderer: &Renderer) {
        debug!("TextView text changing from '{}' to '{}'", self.text, text);
        self.text = text.to_string();
        self.update_size(renderer);
    }

    pub fn set_font_size(&mut self, size: f32, renderer: &Renderer) {
        debug!("TextView font size changing from {} to {}", self.font_size, size);
        self.font_size = size;
        self.update_size(renderer);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_hover_background_color(&mut self, color: Color) {
        self.hover_background_color = color;
    }

    pub fn set_hover_text_color(&mut self, color: Color) {
        self.hover_text_color = color;
    }

    pub fn update_hover(&mut self, x: f32, y: f32) {
        let was_hovered = self.is_hovered;
        self.is_hovered = self.contains_point(x, y);

        if was_hovered != self.is_hovered {
            if let Some(callback) = &self.on_hover {
                callback.borrow_mut()(self.is_hovered);
            }
        }
    }

    pub fn has_fade_animation(&self) -> bool {
        self.fade_animation.is_some()
    }

    pub fn has_position_animation(&self) -> bool {
        self.position_animation.is_some()
    }
}

impl Widget for TextView {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn get_background_color(&self) -> Color {
        self.background_color
    }

    fn get_text_color(&self) -> Color {
        self.text_color
    }

    fn get_hover_background_color(&self) -> Color {
        self.hover_background_color
    }

    fn get_hover_text_color(&self) -> Color {
        self.hover_text_color
    }

    fn get_opacity(&self) -> f32 {
        self.opacity
    }

    fn get_is_hovered(&self) -> bool {
        self.is_hovered
    }

    fn get_is_pressed(&self) -> bool {
        self.is_pressed
    }

    fn draw(&self, renderer: &mut Renderer, screen_width: f32, screen_height: f32) {
        let current_background =
            (if self.is_hovered { self.hover_background_color } else { self.background_color }) *
            self.opacity;

        let current_text =
            (if self.is_hovered { self.hover_text_color } else { self.text_color }) * self.opacity;

        renderer.draw_rect(self.x, self.y, self.width, self.height, current_background.to_array());

        renderer
            .text_renderer_mut()
            .render_text(
                &self.text,
                self.x + 10.0,
                self.y + 5.0,
                self.font_size,
                screen_width,
                screen_height,
                current_text.to_array()
            );
    }

    fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn on_mouse_press(&mut self, x: f32, y: f32) -> bool {
        if self.contains_point(x, y) {
            self.is_pressed = true;
            true
        } else {
            false
        }
    }

    fn on_mouse_release(&mut self, x: f32, y: f32) -> bool {
        if self.is_pressed {
            self.is_pressed = false;
            if self.contains_point(x, y) {
                if let Some(callback) = &self.on_click {
                    callback.borrow_mut()();
                }
                return true;
            }
        }
        false
    }

    fn set_on_click<F>(&mut self, callback: F) where F: FnMut() + 'static {
        self.on_click = Some(Arc::new(RefCell::new(callback)));
    }

    fn set_on_hover<F>(&mut self, callback: F) where F: FnMut(bool) + 'static {
        self.on_hover = Some(Arc::new(RefCell::new(callback)));
    }

    fn set_position_animation(&mut self, animation: Vec2Animation) {
        self.position_animation = Some(animation);
    }

    fn animate_fade(&mut self, start: f32, end: f32, duration: f32) {
        self.set_fade_animation(FadeAnimation::new(start, end, duration));
    }

    fn set_fade_animation(&mut self, animation: FadeAnimation) {
        self.fade_animation = Some(animation);
    }

    fn update_animations(&mut self, delta_time: f32) {
        if let Some(ref mut anim) = self.position_animation {
            anim.update(delta_time);
            let pos = anim.value();
            self.x = pos.x;
            self.y = pos.y;

            if anim.is_finished() {
                self.position_animation = None;
            }
        }

        if let Some(ref mut anim) = self.fade_animation {
            anim.update(delta_time);
            self.opacity = anim.value();

            if anim.is_finished() {
                self.fade_animation = None;
            }
        }
    }
}
