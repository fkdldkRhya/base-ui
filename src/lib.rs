pub mod core;
pub mod graphics;
pub mod text;
pub mod animation;
pub mod widget;
pub mod figlet;
pub mod style;
use core::{ GLContext, Window };

// 외부에서 직접 사용하기 쉽도록 re-export합니다.
pub use graphics::renderer::Renderer;
pub use text::font::FontRenderer;
pub use text::text_renderer::TextRenderer;
pub use animation::Animation;
pub use widget::Widget;
pub use widget::widgets::text_view::TextView;
pub use figlet::figlet_3d::FIGLET_3D_FONT;
pub use core::error_handler::initialize_error_handler;

use log::info;
use figlet_rs::FIGfont;
use gl;

pub const ENGINE_NAME: &str = "BASE-UI";
pub const ENGINE_VERSION: &str = "0.0.1";
pub const ENGINE_AUTHOR: &str = "CHOI SIHUN";
pub const ENGINE_COPYRIGHT: &str =
    "Copyright © 2025 BASE-UI GUI Engine by CHOI SIHUN, All rights reserved.";

pub fn initialize(window: &Window) -> GLContext {
    core::logger::initialize();

    // FIGlet 폰트로 아스키 아트 생성
    let standard_font = FIGfont::from_content(FIGLET_3D_FONT).unwrap();
    let figure = standard_font.convert(ENGINE_NAME).unwrap();

    info!("\n{}", figure);
    info!("----------------------------------------");
    info!("{} v{}", ENGINE_NAME, ENGINE_VERSION);
    info!("{}", ENGINE_COPYRIGHT);
    info!("Author: {}", ENGINE_AUTHOR);
    info!("----------------------------------------");

    // OpenGL 컨텍스트 초기화
    let gl_context = GLContext::new(window);

    // OpenGL 정보 출력
    unsafe {
        let version = gl::GetString(gl::VERSION);
        let renderer = gl::GetString(gl::RENDERER);
        let vendor = gl::GetString(gl::VENDOR);
        let glsl = gl::GetString(gl::SHADING_LANGUAGE_VERSION);

        info!("OpenGL Information:");
        info!("  Vendor: {}", std::ffi::CStr::from_ptr(vendor as *const i8).to_string_lossy());
        info!("  Renderer: {}", std::ffi::CStr::from_ptr(renderer as *const i8).to_string_lossy());
        info!("  Version: {}", std::ffi::CStr::from_ptr(version as *const i8).to_string_lossy());
        info!("  GLSL Version: {}", std::ffi::CStr::from_ptr(glsl as *const i8).to_string_lossy());
    }
    info!("----------------------------------------");

    gl_context
}

#[no_mangle]
pub extern "C" fn engine_version() -> *const u8 {
    b"Base UI v0.0.1\0".as_ptr()
}
