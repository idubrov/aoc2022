use crate::{CharMap, Pos2};
use pixels::{Pixels, SurfaceTexture};
use std::time::Duration;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct Channel {
  proxy: Option<EventLoopProxy<UserEvent>>,
}

type Color = (u8, u8, u8);

impl Channel {
  pub fn empty() -> Channel {
    Channel { proxy: None }
  }

  pub fn draw_map(&self, map: &CharMap, color_fn: impl Fn(u8) -> Color) {
    if self.proxy.is_none() {
      return;
    }
    let dims = map.dims();
    let mut framebuf: Vec<u8> = vec![0u8; (dims.x * dims.y * 4) as usize];
    for pos in map.every_pos() {
      let (r, g, b) = color_fn(map[pos]);
      let idx = ((pos.y * dims.x + pos.x) as usize) * 4;
      framebuf[idx..idx + 4].copy_from_slice(&[r, g, b, 0xff]);
    }
    self
      .proxy
      .as_ref()
      .unwrap()
      .send_event(UserEvent::ResizeAndDraw {
        width: dims.x,
        height: dims.y,
        framebuf,
      })
      .map_err(|_| ())
      .expect("message failed");
  }

  pub fn draw_pixel(&self, pos: Pos2, color: Color) {
    if let Some(ref proxy) = self.proxy {
      proxy
        .send_event(UserEvent::Pixel { pos, color })
        .map_err(|_| ())
        .expect("message failed");
    }
  }

  pub fn sleep(&self, dur: Duration) {
    if self.proxy.is_some() {
      std::thread::sleep(dur);
    }
  }
}

enum UserEvent {
  ResizeAndDraw {
    width: isize,
    height: isize,
    framebuf: Vec<u8>,
  },
  Pixel {
    pos: Pos2,
    color: Color,
  },
}

pub fn visualize(title: &str, worker_fn: impl FnOnce(Channel) + Send + 'static) {
  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
  let mut input = WinitInputHelper::new();
  let window = {
    let size = LogicalSize::new(1024.0, 768.0);
    WindowBuilder::new()
      .with_title(title)
      .with_inner_size(size)
      .with_min_inner_size(size)
      .build(&event_loop)
      .unwrap()
  };

  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(1 as u32, 1 as u32, surface_texture).unwrap()
  };

  let channel = Channel {
    proxy: Some(event_loop.create_proxy()),
  };
  std::thread::spawn(|| worker_fn(channel));

  event_loop.run(move |event, _, control_flow| {
    if let Event::RedrawRequested(_) = event {
      if pixels.render().is_err() {
        *control_flow = ControlFlow::Exit;
        eprintln!("Render has failed.");
        return;
      }
    }

    // Handle input events
    if input.update(&event) {
      // Close events
      if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
        *control_flow = ControlFlow::Exit;
        return;
      }

      // Resize the window
      if let Some(size) = input.window_resized() {
        pixels.resize_surface(size.width, size.height);
      }
    }

    if let Event::UserEvent(user) = event {
      match user {
        UserEvent::ResizeAndDraw {
          width,
          height,
          framebuf,
        } => {
          pixels.resize_buffer(width as u32, height as u32);
          pixels.get_frame_mut().copy_from_slice(&framebuf);
          window.request_redraw();
        }
        UserEvent::Pixel { pos, color: (r, g, b) } => {
          let w = pixels.clamp_pixel_pos((isize::MAX, isize::MAX)).0 as isize + 1;
          let idx = ((pos.y * w + pos.x) as usize) * 4;
          pixels.get_frame_mut()[idx..idx + 4].copy_from_slice(&[r, g, b, 0xff]);
          window.request_redraw();
        }
      }
    }
  });
}
