use crate::animation::animation::{ Animation, Vec2Animation, FadeAnimation };
use crate::widget::Widget;
use crate::graphics::Renderer;
use crate::style::color::Color;
use log::{ debug, info };
use std::sync::Arc;
use std::cell::RefCell;

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    font_size: f32,
    background_color: Color,
    text_color: Color,
    border_color: Color,
    border_width: f32, // 외곽선 두께
    padding: f32, // 텍스트와 버튼 경계 사이의 여백
    is_pressed: bool,
    on_click: Option<Arc<RefCell<dyn FnMut() + 'static>>>,
    pressed_background_color: Color,
    pressed_border_color: Color,
    pressed_text_color: Color,
    is_hovered: bool,
    hover_background_color: Color,
    hover_border_color: Color,
    hover_text_color: Color,
    on_hover: Option<Arc<RefCell<dyn FnMut(bool) + 'static>>>,
    position_animation: Option<Vec2Animation>,
    fade_animation: Option<FadeAnimation>,
    opacity: f32,
}

impl Button {
    pub fn new(text: &str, renderer: &Renderer) -> Self {
        debug!("Creating new Button with text: '{}'", text);
        let background_color = Color::new(0.7, 0.7, 0.7, 1.0);
        let border_color = Color::new(0.3, 0.3, 0.3, 1.0);
        let text_color = Color::new(0.0, 0.0, 0.0, 1.0);

        let mut btn = Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            text: text.to_string(),
            font_size: 32.0,
            background_color,
            text_color,
            border_color,
            border_width: 2.0, // 기본 테두리 두께
            padding: 10.0, // 기본 패딩
            is_pressed: false,
            on_click: None,
            pressed_background_color: Color::new(0.6, 0.6, 0.6, 1.0),
            pressed_border_color: Color::new(0.2, 0.2, 0.2, 1.0),
            pressed_text_color: Color::new(0.0, 0.0, 0.0, 1.0),
            is_hovered: false,
            hover_background_color: Color::new(0.8, 0.8, 0.8, 1.0),
            hover_border_color: Color::new(0.4, 0.4, 0.4, 1.0),
            hover_text_color: Color::new(0.0, 0.0, 0.0, 1.0),
            on_hover: None,
            position_animation: None,
            fade_animation: None,
            opacity: 1.0,
        };
        btn.update_size(renderer);
        info!("Button created with size: {}x{}", btn.width, btn.height);
        btn
    }

    fn update_size(&mut self, renderer: &Renderer) {
        let (text_width, text_height) = renderer
            .text_renderer()
            .font_renderer()
            .calculate_text_size(&self.text, self.font_size);

        // 텍스트 크기에 패딩을 추가하여 버튼 크기 설정
        self.width = text_width + self.padding * 2.0;
        self.height = text_height + self.padding * 2.0;
        debug!("Button size updated: {}x{}", self.width, self.height);
    }

    pub fn set_text(&mut self, text: &str, renderer: &Renderer) {
        debug!("Button text changing from '{}' to '{}'", self.text, text);
        self.text = text.to_string();
        self.update_size(renderer);
    }

    pub fn set_font_size(&mut self, size: f32, renderer: &Renderer) {
        debug!("Button font size changing from {} to {}", self.font_size, size);
        self.font_size = size;
        self.update_size(renderer);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    pub fn set_border_width(&mut self, width: f32) {
        self.border_width = width;
    }

    pub fn set_padding(&mut self, padding: f32, renderer: &Renderer) {
        self.padding = padding;
        self.update_size(renderer);
    }

    // 클릭 이벤트 핸들러 설정
    pub fn set_on_click<F>(&mut self, callback: F) where F: FnMut() + 'static {
        self.on_click = Some(Arc::new(RefCell::new(callback)));
    }

    // pressed 상태의 배경색 설정
    pub fn set_pressed_background_color(&mut self, color: Color) {
        self.pressed_background_color = color;
    }

    // pressed 상태의 테두리 색상 설정
    pub fn set_pressed_border_color(&mut self, color: Color) {
        self.pressed_border_color = color;
    }

    // pressed 상태의 텍스트 색상 설정
    pub fn set_pressed_text_color(&mut self, color: Color) {
        self.pressed_text_color = color;
    }

    // hover 상태의 배경색 설정
    pub fn set_hover_background_color(&mut self, color: Color) {
        self.hover_background_color = color;
    }

    // hover 상태의 테두리 색상 설정
    pub fn set_hover_border_color(&mut self, color: Color) {
        self.hover_border_color = color;
    }

    // hover 상태의 텍스트 색상 설정
    pub fn set_hover_text_color(&mut self, color: Color) {
        self.hover_text_color = color;
    }

    // hover 이벤트 핸들러 설정
    pub fn set_on_hover<F>(&mut self, callback: F) where F: FnMut(bool) + 'static {
        self.on_hover = Some(Arc::new(RefCell::new(callback)));
    }

    // hover 상태 업데이트
    pub fn update_hover(&mut self, x: f32, y: f32) {
        let was_hovered = self.is_hovered;
        self.is_hovered = self.contains_point(x, y);

        // hover 상태가 변경되었을 때만 콜백 호출
        if was_hovered != self.is_hovered {
            if let Some(callback) = &self.on_hover {
                callback.borrow_mut()(self.is_hovered);
            }
        }
    }

    pub fn has_fade_animation(&self) -> bool {
        self.fade_animation.is_some()
    }
}

impl Widget for Button {
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
        let (bg, border, text) = if self.is_pressed {
            (self.pressed_background_color, self.pressed_border_color, self.pressed_text_color)
        } else if self.is_hovered {
            (self.hover_background_color, self.hover_border_color, self.hover_text_color)
        } else {
            (self.background_color, self.border_color, self.text_color)
        };

        let current_background = bg * self.opacity;
        let current_border = border * self.opacity;
        let current_text = text * self.opacity;

        // 렌더링...
        if self.border_width > 0.0 {
            renderer.draw_rect(
                self.x - self.border_width,
                self.y - self.border_width,
                self.width + self.border_width * 2.0,
                self.height + self.border_width * 2.0,
                current_border.to_array()
            );
        }

        renderer.draw_rect(self.x, self.y, self.width, self.height, current_background.to_array());

        // ... 나머지 텍스트 렌더링 코드는 current_text 사용
        let (text_width, text_height) = renderer
            .text_renderer()
            .font_renderer()
            .calculate_text_size(&self.text, self.font_size);

        let text_x = self.x + (self.width - text_width) / 2.0;
        let text_y = self.y + (self.height - text_height) / 2.0;

        renderer
            .text_renderer_mut()
            .render_text(
                &self.text,
                text_x,
                text_y,
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

    fn on_mouse_press(&mut self, _x: f32, _y: f32) -> bool {
        if self.contains_point(_x, _y) {
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
                // 클릭 이벤트 발생
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
        // Update position animation
        if let Some(ref mut anim) = self.position_animation {
            anim.update(delta_time);
            let pos = anim.value();
            self.x = pos.x;
            self.y = pos.y;

            if anim.is_finished() {
                self.position_animation = None;
            }
        }

        // Update fade animation
        if let Some(ref mut anim) = self.fade_animation {
            anim.update(delta_time);
            self.opacity = anim.value();

            if anim.is_finished() {
                self.fade_animation = None;
            }
        }
    }
}
