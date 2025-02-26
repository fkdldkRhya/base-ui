use gl::types::*;

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn upload_data(&self, width: i32, height: i32, data: &[u8]) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            // 텍스처 파라미터 설정
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // 1바이트 정렬 보장
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            // 텍스처 데이터 업로드
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width,
                height,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
