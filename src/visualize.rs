use crate::{Dir2, Pos2};
use pixels::{Pixels, SurfaceTexture};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct Channel {
  channel: Option<Sender<UserEvent>>,
}

pub type Color = (u8, u8, u8);

impl Channel {
  pub fn empty() -> Channel {
    Channel { channel: None }
  }

  pub fn draw_init(&self, top_left: Pos2, bottom_right: Pos2, color_fn: impl Fn(Pos2) -> Color) {
    if self.channel.is_none() {
      return;
    }
    let dims = bottom_right - top_left + Dir2::new(1, 1);
    let mut framebuf: Vec<u8> = vec![0u8; (dims.x * dims.y * 4) as usize];
    for pos in Pos2::iter_rect(top_left, bottom_right) {
      let (r, g, b) = color_fn(pos);
      let idx = (((pos.y - top_left.y) * dims.x + (pos.x - top_left.x)) as usize) * 4;
      framebuf[idx..idx + 4].copy_from_slice(&[r, g, b, 0xff]);
    }
    self
      .channel
      .as_ref()
      .unwrap()
      .send(UserEvent::ResizeAndDraw {
        top_left,
        bottom_right,
        framebuf,
      })
      .map_err(|_| ())
      .expect("message failed");
  }

  pub fn draw_map_pixel(&self, pos: Pos2, color: Color) {
    if let Some(ref channel) = self.channel {
      channel
        .send(UserEvent::Pixel { pos, color })
        .map_err(|_| ())
        .expect("message failed");
    }
  }

  pub fn sleep(&self, dur: Duration) {
    if self.channel.is_some() {
      std::thread::sleep(dur);
    }
  }
}

enum UserEvent {
  ResizeAndDraw {
    top_left: Pos2,
    bottom_right: Pos2,
    framebuf: Vec<u8>,
  },
  Pixel {
    pos: Pos2,
    color: Color,
  },
}

pub fn visualize(title: &str, worker_fn: impl FnOnce(Channel) + Send + 'static) {
  let event_loop = EventLoop::new();
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

  let (sender, receiver) = channel();
  let channel = Channel { channel: Some(sender) };
  std::thread::spawn(|| worker_fn(channel));

  let mut view_state = ViewState {
    top_left: Pos2::new(0, 0),
    bottom_right: Pos2::new(0, 0),
  };
  event_loop.run(move |event, _, control_flow| {
    if let Event::MainEventsCleared = event {
      while let Ok(user) = receiver.try_recv() {
        view_state.process_user_event(&mut pixels, user);
      }
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
  });
}

struct ViewState {
  top_left: Pos2,
  bottom_right: Pos2,
}

impl ViewState {
  fn process_user_event(&mut self, pixels: &mut Pixels, event: UserEvent) {
    match event {
      UserEvent::ResizeAndDraw {
        top_left,
        bottom_right,
        framebuf,
      } => {
        let dims = bottom_right - top_left + Dir2::new(1, 1);
        pixels.resize_buffer(dims.x as u32, dims.y as u32);
        pixels.get_frame_mut().copy_from_slice(&framebuf);
        self.top_left = top_left;
        self.bottom_right = bottom_right;
      }
      UserEvent::Pixel { pos, color: (r, g, b) } => {
        if pos.inside_rect(self.top_left, self.bottom_right) {
          let relative = pos - self.top_left;
          let w = self.bottom_right.x - self.top_left.x + 1;
          let idx = ((relative.y * w + relative.x) as usize) * 4;
          pixels.get_frame_mut()[idx..idx + 4].copy_from_slice(&[r, g, b, 0xff]);
        }
      }
    }
  }
}
