use super::font::FontRenderer;
use crate::graphics::Shader;
use rusttype::PositionedGlyph;

pub struct TextRenderer {
    font_renderer: FontRenderer,
    shader: Shader,
}

impl TextRenderer {
    pub fn new(font_data: Vec<u8>) -> Self {
        Self {
            font_renderer: FontRenderer::new(font_data),
            shader: Shader::new(
                include_str!("shaders/text.vert"),
                include_str!("shaders/text.frag")
            ),
        }
    }

    pub fn render_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        scale: f32,
        screen_width: f32,
        screen_height: f32,
        color: [f32; 4]
    ) {
        self.shader.use_program();
        self.shader.set_vec4("textColor", &color);

        let glyphs = self.font_renderer.render_text(text, scale);
        for glyph in glyphs.iter() {
            if let Some(bb) = glyph.pixel_bounding_box() {
                // 글리프의 위치에 x, y 오프셋을 더합니다
                let adjusted_bb = rusttype::Rect {
                    min: rusttype::Point {
                        x: bb.min.x + (x as i32),
                        y: bb.min.y + (y as i32),
                    },
                    max: rusttype::Point {
                        x: bb.max.x + (x as i32),
                        y: bb.max.y + (y as i32),
                    },
                };
                self.draw_glyph_with_bb(glyph, &adjusted_bb, screen_width, screen_height);
            }
        }
    }

    /// 주어진 glyph를 래스터화하여 OpenGL 텍스처를 생성하고, 해당 glyph의 사각형(quad)을 그리는 함수.
    ///
    /// - `font_renderer`: 폰트 데이터를 관리하는 객체 (래스터화 전에 glyph의 배치 정보가 필요함)
    /// - `glyph`: 렌더링할 개별 글리프 (rusttype::PositionedGlyph)
    /// - `screen_width`, `screen_height`: 현재 화면의 픽셀 크기
    fn draw_glyph_with_bb(
        &self,
        glyph: &PositionedGlyph<'_>,
        bb: &rusttype::Rect<i32>,
        screen_width: f32,
        screen_height: f32
    ) {
        let width = bb.width() as usize;
        let height = bb.height() as usize;

        // 픽셀 데이터를 저장할 버퍼 (단일 채널, 8비트)
        let mut pixel_data = vec![0u8; width * height];

        // glyph.draw() 콜백을 통해 각 픽셀의 커버리지 값을 0.0~1.0 범위의 f32로 받아 0~255로 변환합니다.
        glyph.draw(|x, y, v| {
            let idx = (y as usize) * width + (x as usize);
            pixel_data[idx] = (v * 255.0) as u8;
        });

        // OpenGL 텍스처 생성 및 업로드
        let mut tex_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);

            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // 1바이트 정렬 보장을 위해 설정 (픽셀 데이터가 1바이트 단위이므로)
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width as i32,
                height as i32,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                pixel_data.as_ptr() as *const _
            );
        }

        let x0 = bb.min.x as f32;
        let y0 = bb.min.y as f32;
        let x1 = bb.max.x as f32;
        let y1 = bb.max.y as f32;

        // OpenGL의 좌표계는 정규화 장치 좌표(NDC, -1 ~ 1)입니다.
        let ndc_x0 = (x0 / screen_width) * 2.0 - 1.0;
        let ndc_y0 = 1.0 - (y0 / screen_height) * 2.0;
        let ndc_x1 = (x1 / screen_width) * 2.0 - 1.0;
        let ndc_y1 = 1.0 - (y1 / screen_height) * 2.0;

        // 사각형의 정점 데이터: 각 정점에 3차원 위치와 2차원 텍스처 좌표 (총 5개 요소)
        let vertices: [f32; 20] = [
            // 위치(x, y, z)         // 텍스처 좌표 (u, v)
            ndc_x0,
            ndc_y1,
            0.0,
            0.0,
            1.0, // 좌측 하단
            ndc_x1,
            ndc_y1,
            0.0,
            1.0,
            1.0, // 우측 하단
            ndc_x1,
            ndc_y0,
            0.0,
            1.0,
            0.0, // 우측 상단
            ndc_x0,
            ndc_y0,
            0.0,
            0.0,
            0.0, // 좌측 상단
        ];
        // 사각형을 그리기 위한 인덱스
        let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

        // VAO, VBO, EBO를 생성하여 사각형 드로잉 준비
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // VBO 설정
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            // EBO 설정
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            // 정점 속성: location 0 = position (vec3), location 1 = tex coord (vec2)
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as i32,
                (3 * std::mem::size_of::<f32>()) as *const _
            );
            gl::EnableVertexAttribArray(1);

            // 드로우 호출: 텍스처가 활성화된 상태에서 사각형을 그립니다.
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            gl::DeleteBuffers(1, &vbo);
            gl::DeleteBuffers(1, &ebo);
            gl::DeleteVertexArrays(1, &vao);
            gl::DeleteTextures(1, &tex_id);
        }
    }

    pub fn font_renderer(&self) -> &FontRenderer {
        &self.font_renderer
    }
}
