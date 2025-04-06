use std::{path::Path, time::Duration};

use lib::Priority;
use sdl2::{
    self,
    event::Event,
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    ttf::Font,
};

fn text_box<'a, T>(t: &str, font: &Font, texture_creator: &'a TextureCreator<T>) -> Texture<'a> {
    let surf = font.render(t).blended::<'a, _>(Color::BLACK).unwrap();
    texture_creator.create_texture_from_surface(surf).unwrap()
}

fn copy_unscaled<T: sdl2::render::RenderTarget>(
    dst: &mut Canvas<T>,
    top_left: Point,
    texture: &Texture<'_>,
) {
    let q = texture.query();
    dst.copy(
        texture,
        None,
        Rect::new(top_left.x, top_left.y, q.width, q.height),
    )
    .unwrap();
}

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 568;
const INBOX_URL: &'static str = "https://jot.mndco11age.xyz";
const USER_ID: u32 = 1; // FIXME

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.text_input();

    let window = video_subsystem
        .window("jot | scratchpad", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut texture_creator = canvas.texture_creator();

    // program state enumeration, containing buffers for the text the user
    // is currently typing
    enum TextState {
        Login { userbuf: String, passbuf: String },
        Scratchpad { textbuf: String },
    }
    impl TextState {
        fn login() -> Self {
            Self::Login {
                userbuf: String::new(),
                passbuf: String::new(),
            }
        }
        fn scratchpad() -> Self {
            Self::Scratchpad {
                textbuf: String::new(),
            }
        }
    }
    // the current program state
    let mut state = TextState::scratchpad();
    let mut priority: Priority = Priority::LOW;

    // texture assets
    let assets_path = Path::new("scratchpad/assets");
    let jot_tex = texture_creator
        .load_texture(assets_path.join("jot.svg"))
        .unwrap();
    let low_tex = texture_creator
        .load_texture(assets_path.join("low.svg"))
        .unwrap();
    let medium_tex = texture_creator
        .load_texture(assets_path.join("medium.svg"))
        .unwrap();
    let high_tex = texture_creator
        .load_texture(assets_path.join("high.svg"))
        .unwrap();

    let font_context = sdl2::ttf::init().unwrap();
    let font = font_context
        .load_font(assets_path.join("Orbit-Regular.ttf"), 13)
        .unwrap();
    let big_font = font_context
        .load_font(assets_path.join("Orbit-Regular.ttf"), 32)
        .unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // rects used for input
        let button_width = jot_tex.query().width;
        let button_height = jot_tex.query().height;
        const BETWEEN_PADDING: u32 = 10;
        const EDGE_PADDING: u32 = 20;
        let priority_button_rect = Rect::new(
            EDGE_PADDING as i32,
            (SCREEN_HEIGHT - button_height - EDGE_PADDING) as i32,
            button_width,
            button_height,
        );
        let jot_button_rect = Rect::new(
            (EDGE_PADDING + BETWEEN_PADDING + button_width) as i32,
            (SCREEN_HEIGHT - button_height - EDGE_PADDING) as i32,
            button_width,
            button_height,
        );

        let client = reqwest::blocking::Client::new();

        // process input events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::TextInput {
                    timestamp,
                    window_id,
                    text,
                } => match &mut state {
                    TextState::Login { userbuf, passbuf } => {
                        todo!()
                    }
                    TextState::Scratchpad { textbuf } => textbuf.push_str(&text),
                },
                // Event::TextEditing {
                //     timestamp,
                //     window_id,
                //     text,
                //     start,
                //     length,
                // } => match &mut state {
                //     TextState::Login { userbuf, passbuf } => {
                //         todo!()
                //     }
                //     TextState::Scratchpad { textbuf } => {
                //         textbuf.clear();
                //         textbuf.push_str(&text)
                //     }
                // },
                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    let p = Point::new(x, y);
                    match &mut state {
                        TextState::Login { userbuf, passbuf } => {
                            todo!()
                        }
                        TextState::Scratchpad { textbuf } => {
                            if priority_button_rect.contains_point(p) {
                                // the priority button was pressed
                                priority = ((priority as u32 + 1) % 3).try_into().unwrap();
                            } else if jot_button_rect.contains_point(p) {
                                // the jot button was pressed
                                let req = lib::NotePost {
                                    user_id: 2, // FIXME
                                    text: textbuf.clone(),
                                    priority,
                                };
                                let res = client
                                    .post(&format!("{}/post", INBOX_URL))
                                    .json(&req)
                                    .send()
                                    .unwrap();
                                textbuf.clear();
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // draw the jot button
        canvas.copy(&jot_tex, None, jot_button_rect).unwrap();
        // draw the priority button
        let priority_tex = match priority {
            Priority::LOW => &low_tex,
            Priority::MED => &medium_tex,
            Priority::HIGH => &high_tex,
        };
        canvas
            .copy(&priority_tex, None, priority_button_rect)
            .unwrap();

        if let TextState::Scratchpad { textbuf } = &state {
            if !textbuf.is_empty() {
                let tb_tex = text_box(&textbuf, &font, &texture_creator);
                copy_unscaled(
                    &mut canvas,
                    Point::new(EDGE_PADDING as i32, EDGE_PADDING as i32),
                    &tb_tex,
                );
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
