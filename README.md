# BASE-UI

<div align="center">
  <img src="https://img.shields.io/badge/Rust-1.70%2B-orange" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-MIT-blue" alt="License">
  <img src="https://img.shields.io/badge/OpenGL-3.3%2B-green" alt="OpenGL Version">
</div>

## 개요

BASE-UI는 Rust로 작성된 OpenGL 기반의 GUI 엔진입니다. 이 프로젝트는 가볍고 사용하기 쉬운 UI 컴포넌트를 제공하여 데스크톱 애플리케이션 개발을 간소화합니다.

## 특징

- **OpenGL 기반 렌더링**: 하드웨어 가속을 통한 빠른 그래픽 렌더링
- **다양한 UI 위젯**: 버튼, 텍스트 뷰, 이미지 뷰, 컨텍스트 메뉴 등 기본 UI 컴포넌트 제공
- **사용자 정의 가능한 스타일**: 색상, 폰트, 크기 등을 쉽게 커스터마이징
- **이벤트 처리 시스템**: 클릭, 호버 등의 사용자 상호작용 이벤트 처리
- **애니메이션 지원**: UI 요소에 애니메이션 효과 적용 가능
- **텍스트 렌더링**: TrueType 폰트 지원 및 고품질 텍스트 렌더링
- **FIGlet 아스키 아트 지원**: 콘솔 출력용 아스키 아트 생성

## 시작하기

### 요구사항

- Rust 1.70 이상
- OpenGL 3.3 이상을 지원하는 그래픽 카드 및 드라이버

### 설치

Cargo.toml에 다음 의존성을 추가하세요:

```toml
[dependencies]
base-ui = "0.1.0"
```

또는 이 저장소를 직접 클론하여 사용할 수 있습니다:

```bash
git clone https://github.com/yourusername/base-ui.git
cd base-ui
cargo build --release
```

### 간단한 예제

```rust
use base_ui::widget::widgets::Button;
use base_ui::style::color::Color;
use base_ui::core::Window;

fn main() {
    // 오류 핸들러 초기화
    base_ui::core::initialize_error_handler();

    // 윈도우 생성
    let (mut window, event_loop) = Window::new("My First App", 800, 600);

    // OpenGL 컨텍스트 초기화
    let gl_context = base_ui::initialize(&window);

    // 렌더러 생성
    let font_data = include_bytes!("assets/FiraCode-VariableFont_wght.ttf").to_vec();
    let mut renderer = base_ui::graphics::Renderer::new(font_data);

    // 버튼 생성
    let mut button = Button::new("Click Me!", &renderer);
    button.set_position(300.0, 300.0);
    button.set_size(200.0, 50.0);
    button.set_background_color(Color::new(0.2, 0.6, 1.0, 1.0));
    button.set_on_click(|| {
        println!("Button clicked!");
    });

    // 이벤트 루프 실행
    // ...
}
```

## 컴포넌트

BASE-UI는 다음과 같은 UI 컴포넌트를 제공합니다:

- **Button**: 클릭 가능한 버튼
- **TextView**: 텍스트 표시 위젯
- **ImageView**: 이미지 표시 위젯
- **ContextMenu**: 컨텍스트 메뉴 및 메뉴 아이템
- **Shape**: 사각형, 원 등의 기본 도형

각 컴포넌트는 위치, 크기, 색상, 이벤트 핸들러 등을 설정할 수 있습니다.

## 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 LICENSE 파일을 참조하세요.

## 기여하기

기여는 언제나 환영합니다! 버그 리포트, 기능 요청, 풀 리퀘스트 등을 통해 프로젝트 개선에 참여해주세요.

## 저자

- **최시훈** - 초기 개발 및 유지보수

## 감사의 말

이 프로젝트는 다음 라이브러리를 사용합니다:

- gl
- glutin
- rusttype
- image
- figlet-rs
- 그 외 Cargo.toml에 명시된 의존성들

---

Copyright © 2025 BASE-UI GUI Engine by CHOI SIHUN, All rights reserved.
