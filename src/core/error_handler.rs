use backtrace::Backtrace;
use log::error;
use native_dialog::{ MessageDialog, MessageType };
use std::panic;

pub fn initialize_error_handler() {
    panic::set_hook(
        Box::new(move |panic_info| {
            let message = if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown panic occurred".to_string()
            };

            let location = panic_info.location().map_or_else(
                || "Unknown location".to_string(),
                |loc| format!("{}:{}", loc.file(), loc.line())
            );

            let backtrace = Backtrace::new();

            // 전체 오류 정보는 로그에만 기록
            error!(
                "Application crashed!\n\nError: {}\nLocation: {}\n\nStack trace:\n{:?}",
                message,
                location,
                backtrace
            );

            // 메시지 박스에는 간단한 정보만 표시
            let dialog_msg = format!(
                "Application crashed!\n\nError: {}\nLocation: {}\n\nCheck the log for more details.",
                message,
                location
            );

            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Fatal Error")
                .set_text(&dialog_msg)
                .show_alert()
                .unwrap_or(());

            std::process::exit(1);
        })
    );
}
