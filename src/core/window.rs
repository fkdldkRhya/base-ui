use glutin::dpi::PhysicalPosition;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::PhysicalSize;
use glutin::PossiblyCurrent;
use glutin::WindowedContext;

pub struct Window {
    context: WindowedContext<PossiblyCurrent>,
    size: PhysicalSize<u32>,
    previous_size: PhysicalSize<u32>, // 이전 크기 저장
    cursor_position: Option<PhysicalPosition<f64>>, // 커서 위치 저장 필드
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let size = PhysicalSize::new(width, height);
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new().with_title(title).with_inner_size(size);

        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &event_loop)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        (
            Self {
                context: windowed_context,
                size,
                previous_size: size, // 초기값은 현재 크기와 동일
                cursor_position: None,
            },
            event_loop,
        )
    }

    pub fn size(&self) -> (f32, f32) {
        (self.size.width as f32, self.size.height as f32)
    }

    pub fn swap_buffers(&self) {
        self.context.swap_buffers().unwrap();
    }

    pub fn get_proc_address(&self, s: &str) -> *const std::ffi::c_void {
        self.context.get_proc_address(s) as *const _
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) -> (f32, f32) {
        let scale_x = (new_size.width as f32) / (self.size.width as f32);
        let scale_y = (new_size.height as f32) / (self.size.height as f32);

        self.previous_size = self.size;
        self.size = new_size;
        self.context.resize(new_size);

        unsafe {
            gl::Viewport(0, 0, new_size.width as i32, new_size.height as i32);
        }

        (scale_x, scale_y)
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.resize(PhysicalSize::new(width, height));
    }

    pub fn set_cursor_position(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = Some(position);
    }

    // 커서 위치를 반환하는 함수
    pub fn get_cursor_position(&self) -> Option<(f64, f64)> {
        self.cursor_position.map(|pos| (pos.x, pos.y))
    }
}
