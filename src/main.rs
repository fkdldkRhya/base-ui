// src/main.rs

use base_ui::widget::widgets::context_menu::{ ContextMenu, MenuItem };
use base_ui::widget::widgets::{ Button, TextView, ImageView };
use base_ui::style::color::Color;
use glutin::event::{ ElementState, Event, MouseButton, WindowEvent };
use glutin::event_loop::ControlFlow;
use std::time::Instant;
use log::{ info, debug };

use base_ui::core::{ initialize_error_handler, Window };
use base_ui::graphics::Renderer;
use base_ui::widget::Widget;
use base_ui::widget::widgets::shape::{ Shape, ShapeType };

fn main() {
    initialize_error_handler();

    let mut loop_count = 0;

    let (mut window, event_loop) = Window::new("Text Rendering Demo", 1200, 1000);

    // initialize()가 GLContext를 반환하도록 수정
    let gl_context = base_ui::initialize(&window);

    info!("Window created: {}x{}", 1200, 1000);
    debug!("OpenGL context initialized");

    let font_data = include_bytes!("assets/FiraCode-VariableFont_wght.ttf").to_vec();
    let mut renderer = Renderer::new(font_data);

    // 배경색 설정 (예: 짙은 회색)
    renderer.set_background_color(1.0, 1.0, 1.0, 1.0);

    let mut screen_size = window.size();

    let mut label = TextView::new("Hello, Widget!", &renderer);
    label.set_position(200.0, 200.0);
    label.set_font_size(50.0, &renderer);
    label.set_background_color(Color::new(1.0, 0.3, 0.8, 0.3));
    label.set_text_color(Color::new(0.5, 0.4, 1.0, 1.0));
    label.set_hover_background_color(Color::new(0.5, 0.4, 1.0, 0.3));
    label.set_hover_text_color(Color::new(1.0, 1.0, 1.0, 1.0));
    label.set_on_click(|| {
        println!("Label clicked!");
    });
    label.set_on_hover(|is_hovered| {
        if is_hovered {
            println!("Label hovered!");
        } else {
            println!("Label unhovered!");
        }
    });

    let mut button = Button::new("Click Me!", &renderer);
    button.set_position(300.0, 300.0);
    button.set_size(200.0, 50.0);
    button.set_font_size(24.0, &renderer);
    button.set_background_color(Color::new(0.2, 0.6, 1.0, 1.0));
    button.set_text_color(Color::new(1.0, 1.0, 1.0, 1.0));
    button.set_hover_text_color(Color::new(0.9, 0.9, 0.9, 0.0));
    button.set_pressed_text_color(Color::new(0.7, 0.7, 0.7, 1.0));
    button.set_border_color(Color::new(0.1, 0.3, 0.8, 1.0));
    button.set_hover_border_color(Color::new(0.2, 0.4, 0.9, 1.0));
    button.set_pressed_border_color(Color::new(0.05, 0.2, 0.6, 1.0));
    button.set_border_width(2.0);

    // hover 이벤트 핸들러 설정
    button.set_on_hover(|is_hovered| {
        if is_hovered {
            println!("Button hovered!");
        } else {
            println!("Button unhovered!");
        }
    });

    // 클릭 이벤트 핸들러 설정
    button.set_on_click(|| {
        println!("Button clicked!");
    });

    let mut image_view = ImageView::new();
    image_view.set_position(500.0, 200.0);

    // 로컬 이미지 로드 (예시 경로)
    match
        image_view.load_from_url(
            "https://blog.kakaocdn.net/dn/bPZNUl/btqNH1ERpNt/ytnoU8PkkkFi1Kw81jx1Y0/img.png"
        )
    {
        Ok(_) => info!("Image loaded successfully"),
        Err(e) => info!("Failed to load image: {}", e),
    }

    // 이미지 크기 조정 (선택사항)
    image_view.set_size(500.0, 400.0);

    // 클릭 이벤트 핸들러
    image_view.set_on_click(|| {
        println!("Image clicked!");
    });

    // hover 이벤트 핸들러
    image_view.set_on_hover(|is_hovered| {
        if is_hovered {
            println!("Image hovered!");
        } else {
            println!("Image unhovered!");
        }
    });

    // 컨텍스트 메뉴 생성
    let mut context_menu = ContextMenu::new();

    // 메뉴 아이템 추가
    let mut item1 = MenuItem::new("Open");
    item1.set_on_click(|| {
        println!("Open clicked!");
    });
    context_menu.add_item(item1);

    let mut item2 = MenuItem::new("Save");
    item2.set_on_click(|| {
        println!("Save clicked!");
    });
    context_menu.add_item(item2);

    // 사각형 생성
    let mut rect = Shape::new(ShapeType::Rectangle);
    rect.set_position(100.0, 100.0);
    rect.set_size(200.0, 100.0);
    rect.set_fill_color(Color::new(1.0, 0.0, 0.0, 1.0));
    rect.set_border_color(Color::new(0.0, 0.0, 0.0, 1.0));
    rect.set_border_width(2.0);
    rect.set_on_click(|| {
        println!("Rectangle clicked!");
    });
    rect.set_on_hover(|is_hovered| {
        if is_hovered {
            println!("Rectangle hovered!");
        } else {
            println!("Rectangle unhovered!");
        }
    });

    // 원 생성
    let mut circle = Shape::new(ShapeType::Circle);
    circle.set_position(400.0, 100.0);
    circle.set_size(100.0, 100.0);
    circle.set_fill_color(Color::new(0.0, 1.0, 0.0, 1.0));
    circle.set_on_click(|| {
        println!("Circle clicked!");
    });
    circle.set_on_hover(|is_hovered| {
        if is_hovered {
            println!("Circle hovered!");
        } else {
            println!("Circle unhovered!");
        }
    });

    // 삼각형 생성
    let mut triangle = Shape::new(ShapeType::Triangle);
    triangle.set_position(600.0, 100.0);
    triangle.set_size(100.0, 100.0);
    triangle.set_fill_color(Color::new(0.0, 0.0, 1.0, 1.0));
    triangle.set_on_click(|| {
        println!("Triangle clicked!");
    });
    triangle.set_on_hover(|is_hovered| {
        if is_hovered {
            println!("Triangle hovered!");
        } else {
            println!("Triangle unhovered!");
        }
    });

    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } =>
                match event {
                    WindowEvent::CloseRequested => {
                        info!("Application closing...");
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        let (scale_x, scale_y) = window.resize(physical_size);
                        screen_size = window.size();

                        // 모든 위젯의 크기와 위치를 비율에 맞게 조정
                        label.resize_by_window_size(scale_x, scale_y);
                        button.resize_by_window_size(scale_x, scale_y);
                        image_view.resize_by_window_size(scale_x, scale_y);

                        rect.resize_by_window_size(scale_x, scale_y);
                        circle.resize_by_window_size(scale_x, scale_y);
                        triangle.resize_by_window_size(scale_x, scale_y);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        window.resize(*new_inner_size);
                        screen_size = window.size();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        window.set_cursor_position(position);
                        if let Some((x, y)) = window.get_cursor_position() {
                            button.update_hover(x as f32, y as f32);
                            label.update_hover(x as f32, y as f32);
                            image_view.update_hover(x as f32, y as f32);
                            // --------------------------------
                            context_menu.handle_mouse_move(x as f32, y as f32);
                            // --------------------------------
                            rect.update_hover(x as f32, y as f32);
                            circle.update_hover(x as f32, y as f32);
                            triangle.update_hover(x as f32, y as f32);
                        }
                    }
                    WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                        if let Some((x, y)) = window.get_cursor_position() {
                            match state {
                                ElementState::Pressed => {
                                    button.on_mouse_press(x as f32, y as f32);
                                    label.on_mouse_press(x as f32, y as f32);
                                    image_view.on_mouse_press(x as f32, y as f32);
                                    // --------------------------------
                                    context_menu.handle_click(x as f32, y as f32);
                                    // --------------------------------
                                    rect.on_mouse_press(x as f32, y as f32);
                                    circle.on_mouse_press(x as f32, y as f32);
                                    triangle.on_mouse_press(x as f32, y as f32);
                                }
                                ElementState::Released => {
                                    button.on_mouse_release(x as f32, y as f32);
                                    label.on_mouse_release(x as f32, y as f32);
                                    image_view.on_mouse_release(x as f32, y as f32);
                                    // --------------------------------
                                    context_menu.handle_click(x as f32, y as f32);
                                    // --------------------------------
                                    rect.on_mouse_release(x as f32, y as f32);
                                    circle.on_mouse_release(x as f32, y as f32);
                                    triangle.on_mouse_release(x as f32, y as f32);
                                }
                            }
                        }
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        button: MouseButton::Right,
                        ..
                    } => {
                        if let Some((x, y)) = window.get_cursor_position() {
                            context_menu.show(x as f32, y as f32);
                        }
                    }
                    _ => (),
                }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = now.duration_since(last_frame).as_secs_f32();
                last_frame = now;

                // 애니메이션 업데이트
                label.update_animations(delta_time);
                button.update_animations(delta_time);
                image_view.update_animations(delta_time);
                // --------------------------------
                rect.update_animations(delta_time);
                circle.update_animations(delta_time);
                triangle.update_animations(delta_time);

                // 페이드 아웃 후 페이드 인 애니메이션
                if !button.has_fade_animation() {
                    static mut FADE_OUT: bool = true;
                    unsafe {
                        if FADE_OUT {
                            button.animate_fade(1.0, 0.0, 1.0); // 1.0(불투명) -> 0.0(투명)
                            FADE_OUT = false;
                        } else {
                            button.animate_fade(0.0, 1.0, 1.0); // 0.0(투명) -> 1.0(불투명)
                            FADE_OUT = true;
                        }
                    }
                }

                if !label.has_fade_animation() {
                    static mut FADE_OUT: bool = true;
                    unsafe {
                        if FADE_OUT {
                            label.animate_fade(1.0, 0.0, 1.0); // 1.0(불투명) -> 0.0(투명)
                            FADE_OUT = false;
                        } else {
                            label.animate_fade(0.0, 1.0, 1.0); // 0.0(투명) -> 1.0(불투명)
                            FADE_OUT = true;
                        }
                    }
                }

                if !rect.has_fade_animation() {
                    static mut FADE_OUT: bool = true;
                    unsafe {
                        if FADE_OUT {
                            rect.animate_fade(1.0, 0.0, 1.0); // 1.0(불투명) -> 0.0(투명)
                            FADE_OUT = false;
                        } else {
                            rect.animate_fade(0.0, 1.0, 1.0); // 0.0(투명) -> 1.0(불투명)
                            FADE_OUT = true;
                        }
                    }
                }

                // 사이즈 랜덤
                // if loop_count % 100 == 0 {
                //     let random_size = rand::thread_rng().gen_range(100..300);
                //     label.set_size(random_size as f32, 50.0);
                //     button.set_size(random_size as f32, 50.0);
                //     image_view.set_size(random_size as f32, 200.0);
                // }

                // // 1000 번의 1번꼴로 예외 발생
                // if loop_count % 1000 == 0 {
                //     panic!("Test panic!");
                // }

                gl_context.clear(0.2, 0.3, 0.3, 1.0);

                // 기본 렌더링
                renderer.render(screen_size.0, screen_size.1);

                // 위젯 렌더링 (텍스트 포함)
                label.draw(&mut renderer, screen_size.0, screen_size.1);
                button.draw(&mut renderer, screen_size.0, screen_size.1);
                image_view.draw(&mut renderer, screen_size.0, screen_size.1);

                // 메뉴 렌더링
                context_menu.draw(&mut renderer, screen_size.0, screen_size.1);

                // 사각형 렌더링
                rect.draw(&mut renderer, screen_size.0, screen_size.1);

                // 원 렌더링
                circle.draw(&mut renderer, screen_size.0, screen_size.1);

                // 삼각형 렌더링
                triangle.draw(&mut renderer, screen_size.0, screen_size.1);

                window.swap_buffers();
                loop_count += 1;
            }
            _ => (),
        }
    });
}
