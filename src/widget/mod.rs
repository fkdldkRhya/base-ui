pub mod widgets;

use crate::animation::animation::{ Vec2Animation, FadeAnimation };
use crate::graphics::Renderer;
use crate::style::color::Color;
use nalgebra_glm as glm;

pub trait Widget {
    // 위젯 정보 반환
    fn get_position(&self) -> (f32, f32);
    fn get_size(&self) -> (f32, f32);
    fn get_background_color(&self) -> Color;
    fn get_text_color(&self) -> Color;
    fn get_hover_background_color(&self) -> Color;
    fn get_hover_text_color(&self) -> Color;
    fn get_opacity(&self) -> f32;
    fn get_is_hovered(&self) -> bool;
    fn get_is_pressed(&self) -> bool;

    fn draw(&self, renderer: &mut Renderer, screen_width: f32, screen_height: f32);
    fn set_position(&mut self, x: f32, y: f32);
    fn set_size(&mut self, width: f32, height: f32);
    fn position(&self) -> (f32, f32);
    fn size(&self) -> (f32, f32);

    // 클릭 이벤트 핸들러 설정
    fn set_on_click<F>(&mut self, _callback: F) where F: FnMut() + 'static {
        // 기본 구현은 아무것도 하지 않음
    }

    // hover 이벤트 핸들러 설정
    fn set_on_hover<F>(&mut self, _callback: F) where F: FnMut(bool) + 'static {
        // 기본 구현은 아무것도 하지 않음
    }

    // 새로운 마우스 이벤트 처리 메서드들
    fn on_mouse_press(&mut self, _x: f32, _y: f32) -> bool {
        false // 기본적으로는 이벤트를 처리하지 않음
    }

    fn on_mouse_release(&mut self, _x: f32, _y: f32) -> bool {
        false
    }

    // 마우스 포인터가 위젯 영역 내에 있는지 확인하는 헬퍼 메서드
    fn contains_point(&self, x: f32, y: f32) -> bool {
        let (widget_x, widget_y) = self.position();
        let (width, height) = self.size();

        x >= widget_x && x <= widget_x + width && y >= widget_y && y <= widget_y + height
    }

    fn animate_position(&mut self, target_x: f32, target_y: f32, duration: f32) {
        let current_pos = self.position();
        let start = glm::vec2(current_pos.0, current_pos.1);
        let end = glm::vec2(target_x, target_y);
        self.set_position_animation(Vec2Animation::new(start, end, duration));
    }

    fn animate_fade(&mut self, start: f32, end: f32, duration: f32) {
        self.set_fade_animation(FadeAnimation::new(start, end, duration));
    }

    fn set_position_animation(&mut self, animation: Vec2Animation);
    fn set_fade_animation(&mut self, animation: FadeAnimation);
    fn update_animations(&mut self, delta_time: f32);

    fn resize_by_window_size(&mut self, scale_x: f32, scale_y: f32) {
        let (old_width, old_height) = self.size();
        let (old_x, old_y) = self.position();

        // 위치와 크기를 화면 크기에 대한 비율로 유지
        let relative_x = old_x / (1.0 / scale_x);
        let relative_y = old_y / (1.0 / scale_y);
        let relative_width = old_width / (1.0 / scale_x);
        let relative_height = old_height / (1.0 / scale_y);

        self.set_position(relative_x, relative_y);
        self.set_size(relative_width, relative_height);
    }
}
