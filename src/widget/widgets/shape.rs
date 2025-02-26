use log::debug;

use crate::animation::animation::{ FadeAnimation, Vec2Animation };
use crate::graphics::Renderer;
use crate::style::color::Color;
use crate::widget::Widget;
use crate::Animation;
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub enum ShapeType {
    Rectangle,
    Circle,
    Triangle,
}

pub struct Shape {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    shape_type: ShapeType,
    fill_color: Color,
    border_color: Color,
    border_width: f32,
    opacity: f32,
    is_hovered: bool,
    is_pressed: bool,
    position_animation: Option<Vec2Animation>,
    fade_animation: Option<FadeAnimation>,
    on_hover: Option<Arc<RefCell<dyn FnMut(bool) + 'static>>>,
    on_click: Option<Arc<RefCell<dyn FnMut() + 'static>>>,
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        debug!("Creating new shape: {:?}", shape_type);
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            shape_type,
            fill_color: Color::new(1.0, 1.0, 1.0, 1.0),
            border_color: Color::new(0.0, 0.0, 0.0, 1.0),
            border_width: 0.0,
            opacity: 1.0,
            is_hovered: false,
            is_pressed: false,
            position_animation: None,
            fade_animation: None,
            on_hover: None,
            on_click: None,
        }
    }

    pub fn set_fill_color(&mut self, color: Color) {
        debug!("Setting fill color: {:?}", color);
        self.fill_color = color;
    }

    pub fn set_border_color(&mut self, color: Color) {
        debug!("Setting border color: {:?}", color);
        self.border_color = color;
    }

    pub fn set_border_width(&mut self, width: f32) {
        debug!("Setting border width: {}", width);
        self.border_width = width;
    }

    // hover 상태 업데이트
    pub fn update_hover(&mut self, x: f32, y: f32) {
        // 이전 상태를 저장
        let was_hovered = self.is_hovered;

        // 현재 위치가 이전과 같은 상태면 계산 스킵
        let is_now_hovered = self.contains_point(x, y);
        if was_hovered == is_now_hovered {
            return;
        }

        self.is_hovered = is_now_hovered;
        if let Some(callback) = &self.on_hover {
            callback.borrow_mut()(self.is_hovered);
        }
    }

    pub fn has_fade_animation(&self) -> bool {
        self.fade_animation.is_some()
    }

    pub fn has_position_animation(&self) -> bool {
        self.position_animation.is_some()
    }
}

impl Widget for Shape {
    fn draw(&self, renderer: &mut Renderer, _screen_width: f32, _screen_height: f32) {
        // opacity를 적용한 색상 계산
        let fill_color = self.fill_color.with_opacity(self.opacity);
        let border_color = self.border_color.with_opacity(self.opacity);

        match self.shape_type {
            ShapeType::Rectangle => {
                // 테두리 그리기
                if self.border_width > 0.0 {
                    renderer.draw_rect(
                        self.x - self.border_width,
                        self.y - self.border_width,
                        self.width + self.border_width * 2.0,
                        self.height + self.border_width * 2.0,
                        border_color.to_array()
                    );
                }
                // 사각형 내부 그리기
                renderer.draw_rect(self.x, self.y, self.width, self.height, fill_color.to_array());
            }
            ShapeType::Circle => {
                self.draw_circle_with_opacity(renderer, fill_color, border_color)
            }
            ShapeType::Triangle => {
                self.draw_triangle_with_opacity(renderer, fill_color, border_color)
            }
        }
    }

    // Widget trait의 나머지 필수 메서드들 구현
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn get_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    fn get_background_color(&self) -> Color {
        self.fill_color
    }
    fn get_text_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }
    fn get_hover_background_color(&self) -> Color {
        self.fill_color
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
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
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

    fn set_position_animation(&mut self, animation: Vec2Animation) {
        self.position_animation = Some(animation);
    }

    fn set_fade_animation(&mut self, animation: FadeAnimation) {
        self.fade_animation = Some(animation);
    }

    fn set_on_click<F>(&mut self, callback: F) where F: FnMut() + 'static {
        self.on_click = Some(Arc::new(RefCell::new(callback)));
    }

    fn set_on_hover<F>(&mut self, callback: F) where F: FnMut(bool) + 'static {
        self.on_hover = Some(Arc::new(RefCell::new(callback)));
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
        if self.is_pressed && self.contains_point(x, y) {
            if let Some(callback) = &self.on_click {
                callback.borrow_mut()();
            }
            true
        } else {
            false
        }
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

impl Shape {
    const CIRCLE_SEGMENTS: usize = 32;

    fn draw_circle_with_opacity(
        &self,
        renderer: &mut Renderer,
        fill_color: Color,
        border_color: Color
    ) {
        let center_x = self.x + self.width / 2.0;
        let center_y = self.y + self.height / 2.0;
        let radius = self.width.min(self.height) / 2.0;

        // 테두리가 있을 때만 테두리 그리기
        if self.border_width > 0.0 {
            renderer.draw_circle(
                center_x,
                center_y,
                radius + self.border_width,
                border_color.to_array()
            );
        }

        // 내부 채우기
        renderer.draw_circle(center_x, center_y, radius, fill_color.to_array());
    }

    fn draw_triangle_with_opacity(
        &self,
        renderer: &mut Renderer,
        fill_color: Color,
        border_color: Color
    ) {
        let vertices = [
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
            (self.x + self.width / 2.0, self.y),
        ];

        // 테두리가 있을 때만 테두리 그리기
        if self.border_width > 0.0 {
            let border_vertices = [
                (vertices[0].0 - self.border_width, vertices[0].1 + self.border_width),
                (vertices[1].0 + self.border_width, vertices[1].1 + self.border_width),
                (vertices[2].0, vertices[2].1 - self.border_width),
            ];
            renderer.draw_triangle(border_vertices, border_color.to_array());
        }

        renderer.draw_triangle(vertices, fill_color.to_array());
    }
}
