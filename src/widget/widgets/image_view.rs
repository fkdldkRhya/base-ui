use crate::style::color::Color;
use crate::widget::Widget;
use crate::graphics::Renderer;
use crate::animation::animation::{ Animation, Vec2Animation, FadeAnimation };
use image::{ DynamicImage, GenericImageView };
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;
use log::debug;
pub struct ImageView {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    texture_id: Option<u32>,
    position_animation: Option<Vec2Animation>,
    fade_animation: Option<FadeAnimation>,
    on_hover: Option<Arc<RefCell<dyn FnMut(bool) + 'static>>>,
    is_pressed: bool,
    is_hovered: bool,
    on_click: Option<Arc<RefCell<dyn FnMut() + 'static>>>,
    opacity: f32,
}

impl ImageView {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            texture_id: None,
            position_animation: None,
            fade_animation: None,
            opacity: 1.0,
            is_pressed: false,
            is_hovered: false,
            on_click: None,
            on_hover: None,
        }
    }

    pub fn load_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let img: DynamicImage = image::open(path).map_err(|e| e.to_string())?;
        self.load_image(img)
    }

    pub fn load_from_url(&mut self, url: &str) -> Result<(), String> {
        let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
        let bytes = response.bytes().map_err(|e| e.to_string())?;
        let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
        self.load_image(img)
    }

    fn load_image(&mut self, img: DynamicImage) -> Result<(), String> {
        let dimensions = img.dimensions();
        self.width = dimensions.0 as f32;
        self.height = dimensions.1 as f32;

        // Convert image to RGBA
        let rgba = img.to_rgba8();
        let raw = rgba.into_raw();

        // Create OpenGL texture
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Upload texture data
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                dimensions.0 as i32,
                dimensions.1 as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                raw.as_ptr() as *const _
            );
        }

        debug!("Loaded image from path: {:?}", img.dimensions());

        self.texture_id = Some(texture_id);
        Ok(())
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

impl Widget for ImageView {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn get_background_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn get_text_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn get_hover_background_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn get_hover_text_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
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

    fn draw(&self, renderer: &mut Renderer, _screen_width: f32, _screen_height: f32) {
        if let Some(texture_id) = self.texture_id {
            renderer.draw_textured_rect(
                self.x,
                self.y,
                self.width,
                self.height,
                texture_id,
                self.opacity
            );
        }
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
