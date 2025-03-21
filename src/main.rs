// src/main.rs
//
// BASE-UI 데모 애플리케이션
// 이 파일은 BASE-UI 라이브러리의 주요 기능을 보여주는 예제입니다.
// 각 섹션은 라이브러리의 다른 기능을 시연합니다.

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
    // =========================================
    // 1. 초기화 및 기본 설정
    // =========================================

    // 오류 처리 초기화 - 패닉 발생 시 스택 트레이스 출력
    initialize_error_handler();

    // 윈도우 생성 (1200x1000 크기)
    let (mut window, event_loop) = Window::new("BASE-UI Demo Application", 1200, 1000);
    let gl_context = base_ui::initialize(&window);
    info!("Window created: {}x{}", 1200, 1000);
    debug!("OpenGL context initialized");

    // FiraCode 폰트 로드 및 렌더러 초기화
    let font_data = include_bytes!("assets/FiraCode-VariableFont_wght.ttf").to_vec();
    let mut renderer = Renderer::new(font_data);

    // 흰색 배경 설정
    renderer.set_background_color(1.0, 1.0, 1.0, 1.0);

    let mut screen_size = window.size();

    // =========================================
    // 2. UI 컴포넌트 생성 및 설정
    // =========================================

    // TextView 생성 및 스타일링
    let mut label = TextView::new("Hello, Widget!", &renderer);
    label.set_position(200.0, 200.0);
    label.set_font_size(50.0, &renderer);
    label.set_background_color(Color::new(1.0, 0.3, 0.8, 0.3)); // 반투명 핑크
    label.set_text_color(Color::new(0.5, 0.4, 1.0, 1.0)); // 보라색
    label.set_hover_background_color(Color::new(0.5, 0.4, 1.0, 0.3)); // 반투명 보라
    label.set_hover_text_color(Color::new(1.0, 1.0, 1.0, 1.0)); // 흰색

    // Button 생성 및 스타일링
    let mut button = Button::new("Click Me!", &renderer);
    button.set_position(300.0, 300.0);
    button.set_size(200.0, 50.0);
    button.set_font_size(24.0, &renderer);
    button.set_background_color(Color::new(0.2, 0.6, 1.0, 1.0)); // 파란색
    button.set_text_color(Color::new(1.0, 1.0, 1.0, 1.0)); // 흰색
    button.set_hover_text_color(Color::new(0.9, 0.9, 0.9, 0.0)); // 연한 회색
    button.set_pressed_text_color(Color::new(0.7, 0.7, 0.7, 1.0)); // 진한 회색
    button.set_border_color(Color::new(0.1, 0.3, 0.8, 1.0)); // 진한 파란색
    button.set_hover_border_color(Color::new(0.2, 0.4, 0.9, 1.0)); // 밝은 파란색
    button.set_pressed_border_color(Color::new(0.05, 0.2, 0.6, 1.0)); // 어두운 파란색
    button.set_border_width(2.0);

    // ImageView 생성 및 설정
    let mut image_view = ImageView::new();
    image_view.set_position(500.0, 200.0);
    image_view.set_size(500.0, 400.0);

    // 이미지 URL에서 로드
    match
        image_view.load_from_url(
            "https://blog.kakaocdn.net/dn/bPZNUl/btqNH1ERpNt/ytnoU8PkkkFi1Kw81jx1Y0/img.png"
        )
    {
        Ok(_) => info!("Image loaded successfully"),
        Err(e) => info!("Failed to load image: {}", e),
    }

    // =========================================
    // 3. 컨텍스트 메뉴 설정
    // =========================================

    let mut context_menu = ContextMenu::new();

    // 메뉴 아이템 추가 및 이벤트 핸들러 설정
    let mut item1 = MenuItem::new("Open");
    item1.set_on_click(|| println!("Open clicked!"));
    context_menu.add_item(item1);

    let mut item2 = MenuItem::new("Save");
    item2.set_on_click(|| println!("Save clicked!"));
    context_menu.add_item(item2);

    // =========================================
    // 4. 기본 도형 생성
    // =========================================

    // 빨간색 사각형
    let mut rect = Shape::new(ShapeType::Rectangle);
    rect.set_position(100.0, 100.0);
    rect.set_size(200.0, 100.0);
    rect.set_fill_color(Color::new(1.0, 0.0, 0.0, 1.0));
    rect.set_border_color(Color::new(0.0, 0.0, 0.0, 1.0));
    rect.set_border_width(2.0);

    // 초록색 원
    let mut circle = Shape::new(ShapeType::Circle);
    circle.set_position(400.0, 100.0);
    circle.set_size(100.0, 100.0);
    circle.set_fill_color(Color::new(0.0, 1.0, 0.0, 1.0));

    // 파란색 삼각형
    let mut triangle = Shape::new(ShapeType::Triangle);
    triangle.set_position(600.0, 100.0);
    triangle.set_size(100.0, 100.0);
    triangle.set_fill_color(Color::new(0.0, 0.0, 1.0, 1.0));

    // =========================================
    // 5. 이벤트 핸들러 설정
    // =========================================

    // 각 위젯에 개별적으로 이벤트 핸들러 설정
    label.set_on_click(|| {
        println!("Label clicked!");
    });
    label.set_on_hover(|is_hovered| {
        println!("Label {}!", if is_hovered { "hovered" } else { "unhovered" });
    });

    button.set_on_click(|| {
        println!("Button clicked!");
    });
    button.set_on_hover(|is_hovered| {
        println!("Button {}!", if is_hovered { "hovered" } else { "unhovered" });
    });

    image_view.set_on_click(|| {
        println!("ImageView clicked!");
    });
    image_view.set_on_hover(|is_hovered| {
        println!("ImageView {}!", if is_hovered { "hovered" } else { "unhovered" });
    });

    rect.set_on_click(|| {
        println!("Rectangle clicked!");
    });
    rect.set_on_hover(|is_hovered| {
        println!("Rectangle {}!", if is_hovered { "hovered" } else { "unhovered" });
    });

    circle.set_on_click(|| {
        println!("Circle clicked!");
    });
    circle.set_on_hover(|is_hovered| {
        println!("Circle {}!", if is_hovered { "hovered" } else { "unhovered" });
    });

    triangle.set_on_click(|| {
        println!("Triangle clicked!");
    });
    triangle.set_on_hover(|is_hovered| {
        println!("Triangle {}!", if is_hovered { "hovered" } else { "unhovered" });
    });

    // =========================================
    // 6. 애니메이션 및 이벤트 루프
    // =========================================

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
            }
            _ => (),
        }
    });
}
