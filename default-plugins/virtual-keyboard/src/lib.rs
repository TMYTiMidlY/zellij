use zellij_tile::prelude::*;
use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    mode_info: ModeInfo,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        set_selectable(true);
        subscribe(&[EventType::ModeUpdate, EventType::Mouse]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => {
                self.mode_info = mode_info;
                true
            }
            Event::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event);
                false
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        println!("{}", "┌─────────────────────────────────────────────────────────┐");
        println!("{}", "│ [ESC] [TAB] [↑] [↓] [←] [→] [Ctrl+G] [P] [T] [Q]       │");
        println!("{}", "└─────────────────────────────────────────────────────────┘");
    }

    fn pipe(&mut self, _pipe_message: PipeMessage) -> bool {
        false
    }
}

impl State {
    fn handle_mouse_event(&mut self, mouse_event: Mouse) {
        if let Mouse::LeftClick(row, col) = mouse_event {
            // 简单的点击区域检测
            if row == 1 {
                let button_width = 7;
                let button_index = col / button_width;
                
                match button_index {
                    0 => write_chars("\u{001b}"), // ESC
                    1 => write_chars("\t"),        // TAB
                    2 => write_chars("\u{001b}[A"), // UP
                    3 => write_chars("\u{001b}[B"), // DOWN
                    4 => write_chars("\u{001b}[D"), // LEFT
                    5 => write_chars("\u{001b}[C"), // RIGHT
                    6 => write_chars("\u{0007}"),   // Ctrl+G
                    _ => {}
                }
            }
        }
    }
}
