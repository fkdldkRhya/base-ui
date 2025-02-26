use crate::graphics::Renderer;
use crate::style::color::Color;
use std::sync::Arc;
use std::cell::RefCell;
use log::debug;

pub struct MenuItem {
    text: String,
    on_click: Option<Arc<RefCell<dyn FnMut() + 'static>>>,
    background_color: Color,
    text_color: Color,
    hover_background_color: Color,
    hover_text_color: Color,
    is_hovered: bool,
}

impl MenuItem {
    pub fn new(text: &str) -> Self {
        debug!("Creating new menu item: {}", text);
        Self {
            text: text.to_string(),
            on_click: None,
            background_color: Color::new(0.9, 0.9, 0.9, 1.0),
            text_color: Color::new(0.1, 0.1, 0.1, 1.0),
            hover_background_color: Color::new(0.7, 0.7, 0.9, 1.0),
            hover_text_color: Color::new(1.0, 1.0, 1.0, 1.0),
            is_hovered: false,
        }
    }

    pub fn set_on_click<F>(&mut self, callback: F) where F: FnMut() + 'static {
        debug!("Setting click handler for menu item: {}", self.text);
        self.on_click = Some(Arc::new(RefCell::new(callback)));
    }
}

pub struct ContextMenu {
    x: f32,
    y: f32,
    width: f32,
    items: Vec<MenuItem>,
    visible: bool,
    item_height: f32,
    padding: f32,
    border_color: Color,
    border_width: f32,
}

impl ContextMenu {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            items: Vec::new(),
            visible: false,
            item_height: 30.0,
            padding: 5.0,
            border_color: Color::new(0.8, 0.8, 0.8, 1.0),
            border_width: 1.0,
        }
    }

    pub fn add_item(&mut self, item: MenuItem) {
        debug!("Adding menu item: {}", item.text);
        self.items.push(item);
    }

    pub fn show(&mut self, x: f32, y: f32) {
        debug!("Showing context menu at ({}, {})", x, y);
        self.x = x;
        self.y = y;
        self.visible = true;
    }

    pub fn hide(&mut self) {
        debug!("Hiding context menu");
        self.visible = false;
    }

    pub fn draw(&self, renderer: &mut Renderer, screen_width: f32, screen_height: f32) {
        if !self.visible {
            return;
        }

        let total_height = (self.items.len() as f32) * self.item_height;

        // Draw border
        renderer.draw_rect(
            self.x - self.border_width,
            self.y - self.border_width,
            self.width + self.border_width * 2.0,
            total_height + self.border_width * 2.0,
            self.border_color.to_array()
        );

        // Draw menu items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = self.y + (i as f32) * self.item_height;
            let bg_color = if item.is_hovered {
                item.hover_background_color
            } else {
                item.background_color
            };
            let text_color = if item.is_hovered { item.hover_text_color } else { item.text_color };

            // Draw item background
            renderer.draw_rect(self.x, item_y, self.width, self.item_height, bg_color.to_array());

            // Draw item text
            renderer
                .text_renderer_mut()
                .render_text(
                    &item.text,
                    self.x + self.padding,
                    item_y + (self.item_height - 20.0) / 2.0,
                    20.0,
                    screen_width,
                    screen_height,
                    text_color.to_array()
                );
        }
    }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        if !self.visible {
            return;
        }

        for (i, item) in self.items.iter_mut().enumerate() {
            let item_y = self.y + (i as f32) * self.item_height;
            item.is_hovered =
                x >= self.x &&
                x <= self.x + self.width &&
                y >= item_y &&
                y <= item_y + self.item_height;
        }
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        if !self.visible {
            return false;
        }

        let total_height = (self.items.len() as f32) * self.item_height;
        if x < self.x || x > self.x + self.width || y < self.y || y > self.y + total_height {
            debug!("Click outside menu bounds, hiding menu");
            self.hide();
            return false;
        }

        for (i, item) in self.items.iter_mut().enumerate() {
            let item_y = self.y + (i as f32) * self.item_height;
            if y >= item_y && y <= item_y + self.item_height {
                debug!("Menu item clicked: {}", item.text);
                if let Some(callback) = &item.on_click {
                    callback.borrow_mut()();
                }
                self.hide();
                return true;
            }
        }
        false
    }
}
