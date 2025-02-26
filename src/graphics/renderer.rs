use super::shader::Shader;
use crate::text::TextRenderer;
use nalgebra_glm as glm;

pub struct Renderer {
    shader: Shader,
    texture_shader: Shader,
    text_renderer: TextRenderer,
    screen_width: f32,
    screen_height: f32,
    background_color: [f32; 4],
    vao: u32,
    vbo: u32,
    circle_vertices: Vec<f32>, // 미리 계산된 원의 버텍스들
}

const CIRCLE_SEGMENTS: usize = 32;

impl Renderer {
    pub fn new(font_data: Vec<u8>) -> Self {
        let vertex_source = include_str!("shaders/basic.vert");
        let fragment_source = include_str!("shaders/basic.frag");
        let shader = Shader::new(vertex_source, fragment_source);
        let texture_shader = Shader::new(
            include_str!("shaders/texture.vert"),
            include_str!("shaders/texture.frag")
        );
        let text_renderer = TextRenderer::new(font_data);

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        }

        // 원의 버텍스들을 미리 계산
        let mut circle_vertices = Vec::with_capacity(CIRCLE_SEGMENTS * 3 * 3);
        for i in 0..CIRCLE_SEGMENTS {
            let angle = (2.0 * std::f32::consts::PI * (i as f32)) / (CIRCLE_SEGMENTS as f32);
            let x = angle.cos();
            let y = angle.sin();
            circle_vertices.extend_from_slice(&[x, y, 0.0]);
        }

        Self {
            shader,
            texture_shader,
            text_renderer,
            screen_width: 0.0,
            screen_height: 0.0,
            background_color: [0.1, 0.35, 0.33, 1.0],
            vao,
            vbo,
            circle_vertices,
        }
    }

    pub fn set_background_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.background_color = [r, g, b, a];
    }

    /// 매 프레임마다 호출하여 전체 화면을 다시 그리는 메서드
    /// (배경 지우기, 카메라/투영 매트릭스 설정 등)
    pub fn render(&mut self, screen_width: f32, screen_height: f32) {
        // 화면 크기 업데이트
        self.screen_width = screen_width;
        self.screen_height = screen_height;

        unsafe {
            // 설정된 배경색으로 화면 지우기
            gl::ClearColor(
                self.background_color[0],
                self.background_color[1],
                self.background_color[2],
                self.background_color[3]
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 셰이더 사용
        self.shader.use_program();

        // 예: Orthographic 투영 행렬을 만들고 셰이더 uniform에 설정
        // (glm 사용 시 예시)
        let projection = glm::ortho(0.0, screen_width, screen_height, 0.0, -1.0, 1.0);
        self.shader.set_mat4("projection", &projection);
    }

    /// 텍스트 렌더링 객체에 접근 (폰트 그리기 등)
    pub fn text_renderer(&self) -> &TextRenderer {
        &self.text_renderer
    }

    /// Draws a filled rectangle at (x, y) with given width, height and color.
    /// Assumes that self.screen_width and self.screen_height are updated.
    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        // 현재 바인딩된 shader를 저장
        let current_shader = unsafe {
            let mut current_program = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            current_program
        };

        // 기본 셰이더로 전환하여 사각형 그리기
        self.shader.use_program();
        self.shader.set_vec4("uColor", &color);

        // Convert screen coordinates to normalized device coordinates (NDC)
        let x_ndc = (x / self.screen_width) * 2.0 - 1.0;
        let y_ndc = 1.0 - (y / self.screen_height) * 2.0;
        let x2_ndc = ((x + width) / self.screen_width) * 2.0 - 1.0;
        let y2_ndc = 1.0 - ((y + height) / self.screen_height) * 2.0;

        let vertices: [f32; 12] = [
            x_ndc,
            y_ndc,
            0.0, // top-left
            x2_ndc,
            y_ndc,
            0.0, // top-right
            x2_ndc,
            y2_ndc,
            0.0, // bottom-right
            x_ndc,
            y2_ndc,
            0.0, // bottom-left
        ];
        let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

        unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            let mut ebo = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            // Cleanup
            gl::DeleteVertexArrays(1, &vao);
            gl::DeleteBuffers(1, &vbo);
            gl::DeleteBuffers(1, &ebo);

            // 이전 shader로 복원
            if current_shader != 0 {
                gl::UseProgram(current_shader as u32);
            }
        }
    }

    /// Add a new method for mutable access to the text renderer.
    pub fn text_renderer_mut(&mut self) -> &mut TextRenderer {
        &mut self.text_renderer
    }

    pub fn draw_textured_rect(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_id: u32,
        opacity: f32
    ) {
        self.texture_shader.use_program();

        // Set projection matrix
        let projection = glm::ortho(0.0, self.screen_width, self.screen_height, 0.0, -1.0, 1.0);
        self.texture_shader.set_mat4("projection", &projection);
        self.texture_shader.set_float("uOpacity", opacity);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            let vertices: [f32; 20] = [
                x,
                y,
                0.0,
                0.0,
                1.0,
                x + width,
                y,
                0.0,
                1.0,
                1.0,
                x + width,
                y + height,
                0.0,
                1.0,
                0.0,
                x,
                y + height,
                0.0,
                0.0,
                0.0,
            ];

            let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

            let mut vao = 0;
            let mut vbo = 0;
            let mut ebo = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            // Position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            // Texture coord attribute
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as i32,
                (3 * std::mem::size_of::<f32>()) as *const _
            );
            gl::EnableVertexAttribArray(1);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            gl::DeleteVertexArrays(1, &vao);
            gl::DeleteBuffers(1, &vbo);
            gl::DeleteBuffers(1, &ebo);
        }
    }

    pub fn draw_triangle(&mut self, vertices: [(f32, f32); 3], color: [f32; 4]) {
        let current_shader = unsafe {
            let mut current_program = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            current_program
        };

        self.shader.use_program();
        self.shader.set_vec4("uColor", &color);

        // Convert to NDC
        let vertices_ndc: Vec<f32> = vertices
            .iter()
            .flat_map(|(x, y)| {
                let x_ndc = (x / self.screen_width) * 2.0 - 1.0;
                let y_ndc = 1.0 - (y / self.screen_height) * 2.0;
                vec![x_ndc, y_ndc, 0.0]
            })
            .collect();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices_ndc.len() * std::mem::size_of::<f32>()) as isize,
                vertices_ndc.as_ptr() as *const _,
                gl::STATIC_DRAW
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // 이전 셰이더로 복원
        unsafe {
            gl::UseProgram(current_shader as u32);
        }
    }

    pub fn draw_circle(&mut self, center_x: f32, center_y: f32, radius: f32, color: [f32; 4]) {
        let current_shader = unsafe {
            let mut current_program = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            current_program
        };

        self.shader.use_program();
        self.shader.set_vec4("uColor", &color);

        // 스케일과 위치 변환을 적용한 버텍스들 생성
        let vertices_transformed: Vec<f32> = self.circle_vertices
            .chunks(3)
            .flat_map(|v| {
                let x = center_x + v[0] * radius;
                let y = center_y + v[1] * radius;
                vec![(x / self.screen_width) * 2.0 - 1.0, 1.0 - (y / self.screen_height) * 2.0, 0.0]
            })
            .collect();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices_transformed.len() * std::mem::size_of::<f32>()) as isize,
                vertices_transformed.as_ptr() as *const _,
                gl::STATIC_DRAW
            );
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, CIRCLE_SEGMENTS as i32);
        }

        unsafe {
            gl::UseProgram(current_shader as u32);
        }
    }
}
