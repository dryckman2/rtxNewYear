use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossbeam_channel::Receiver;
use minifb::{Key, Window, WindowOptions};

#[derive(Clone, Copy)]
pub struct Cord {
    pub x: usize,
    pub y: usize,
}

pub struct Pixel {
    pub cord: Cord,
    pub color: (u8, u8, u8),
}

pub fn draw(height: usize, width: usize, receiver: Receiver<Pixel>) {
    // Create a window using the minifb crate.
    let mut window = Window::new(
        "Pixel Renderer - Press ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_target_fps(60);

    let frame_buffer = Arc::new(Mutex::new(vec![0u32; width * height]));

    let jh_fb = frame_buffer.clone();
    // This thread will now block when no messages are available (CPU efficient)
    // and will terminate automatically when the channel is closed.
    std::thread::spawn(move || {
        while let Ok(change) = receiver.recv() {
            jh_fb.lock().unwrap()[change.cord.x + change.cord.y * width] =
                ((change.color.0 as u32) << 16)
                    | ((change.color.1 as u32) << 8)
                    | (change.color.2 as u32);
        }
    });

    // Main loop to keep the window open and responsive. This must be on the main thread.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        {
            let frame = frame_buffer.try_lock();
            match frame {
                Ok(frame) => {
                    window
                        .update_with_buffer(frame.as_slice(), width, height)
                        .unwrap();
                }
                Err(_) => {}
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
