use std::{path::Path, time::Duration};

use lib::Priority;
use sdl2::{
    self,
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{BlendMode, Canvas, RenderTarget, Texture, TextureCreator},
    ttf::Font,
};

fn text_box<'a, T>(
    t: &str,
    font: &Font,
    color: Color,
    texture_creator: &'a TextureCreator<T>,
) -> Texture<'a> {
    let surf = font.render(t).blended::<'a, _>(color).unwrap();
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
    let texture_creator = canvas.texture_creator();

    // program state enumeration, containing buffers for the text the user
    // is currently typing
    enum TextState {
        Login {
            userbuf: String,
            passbuf: String,
            pass_selected: bool,
        },
        Scratchpad {
            textbuf: String,
        },
    }
    impl TextState {
        fn login() -> Self {
            Self::Login {
                userbuf: String::new(),
                passbuf: String::new(),
                pass_selected: false,
            }
        }
        fn scratchpad() -> Self {
            Self::Scratchpad {
                textbuf: String::new(),
            }
        }
    }
    // the current program state
    let mut state = TextState::login();
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
    let rect_tex = texture_creator
        .load_texture(assets_path.join("rect.svg"))
        .unwrap();

    let font_context = sdl2::ttf::init().unwrap();
    let font = font_context
        .load_font(assets_path.join("Orbit-Regular.ttf"), 13)
        .unwrap();
    let big_font = font_context
        .load_font(assets_path.join("Orbit-Regular.ttf"), 32)
        .unwrap();

    let client = reqwest::blocking::Client::new();

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
        const INPUT_FIELD_HEIGHT: u32 = 30;
        const INPUT_FIELD_WIDTH: u32 = 200;
        let username_input_rect = Rect::from_center(
            Point::new(
                (SCREEN_WIDTH / 2) as i32,
                (SCREEN_HEIGHT / 2 - BETWEEN_PADDING) as i32,
            ),
            INPUT_FIELD_WIDTH,
            INPUT_FIELD_HEIGHT,
        );
        let password_input_rect = Rect::from_center(
            Point::new(
                (SCREEN_WIDTH / 2) as i32,
                (SCREEN_HEIGHT / 2 + INPUT_FIELD_HEIGHT) as i32,
            ),
            INPUT_FIELD_WIDTH,
            INPUT_FIELD_HEIGHT,
        );

        // process input events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::TextEditing {
                    timestamp,
                    window_id,
                    text,
                    start,
                    length,
                } => match &mut state {
                    TextState::Login {
                        userbuf,
                        passbuf,
                        pass_selected,
                    } => {
                        dbg!("text editing!");
                        if *pass_selected {
                            *passbuf = text;
                        } else {
                            *userbuf = text;
                        }
                    }
                    TextState::Scratchpad { textbuf } => *textbuf = text,
                },
                Event::KeyDown {
                    keycode: Some(Keycode::TAB),
                    ..
                } => {
                    if let TextState::Login { pass_selected, .. } = &mut state {
                        *pass_selected = !*pass_selected
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    let p = Point::new(x, y);
                    match &mut state {
                        TextState::Login {
                            userbuf,
                            passbuf,
                            pass_selected,
                        } => {
                            // TODO
                        }
                        TextState::Scratchpad { textbuf } => {
                            if priority_button_rect.contains_point(p) {
                                // the priority button was pressed
                                priority = ((priority as u32 + 1) % 3).try_into().unwrap();
                            } else if jot_button_rect.contains_point(p) {
                                // the jot button was pressed
                                let req = lib::NoteRequest {
                                    user_id: 2, // FIXME
                                    text: textbuf.clone(),
                                    priority,
                                };
                                let res = client
                                    .post(&format!("{}/post", INBOX_URL))
                                    .json(&req)
                                    .send()
                                    .unwrap();
                                res.error_for_status()
                                    .expect_err("could not access server!");
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

        match &state {
            TextState::Scratchpad { textbuf } => {
                if !textbuf.is_empty() {
                    let tb_tex = text_box(&textbuf, &font, Color::BLACK, &texture_creator);
                    copy_unscaled(
                        &mut canvas,
                        Point::new(EDGE_PADDING as i32, EDGE_PADDING as i32),
                        &tb_tex,
                    );
                }
            }
            TextState::Login {
                userbuf,
                passbuf,
                pass_selected,
            } => {
                // darken the background
                canvas.set_draw_color(Color::RGBA(0, 0, 0, 100));
                canvas.set_blend_mode(BlendMode::Blend);
                canvas
                    .fill_rect(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT))
                    .unwrap();

                // draw the box for login inputs
                canvas.set_blend_mode(BlendMode::None);
                let q = rect_tex.query();
                copy_unscaled(
                    &mut canvas,
                    Point::new(
                        (SCREEN_WIDTH - q.width) as i32 / 2,
                        (SCREEN_HEIGHT - q.height) as i32 / 2,
                    ),
                    &rect_tex,
                );

                // draw welcome text
                copy_unscaled(
                    &mut canvas,
                    Point::new(60, SCREEN_HEIGHT as i32 / 2 - 90),
                    &text_box("Welcome!", &big_font, Color::BLACK, &texture_creator),
                );

                input_field(
                    userbuf,
                    "username",
                    &font,
                    &username_input_rect,
                    &mut canvas,
                    &texture_creator,
                );

                input_field(
                    passbuf,
                    "password",
                    &font,
                    &password_input_rect,
                    &mut canvas,
                    &texture_creator,
                );
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn input_field<T: RenderTarget, S>(
    input: &str,
    default: &str,
    font: &Font,
    rect: &Rect,
    canvas: &mut Canvas<T>,
    texture_creator: &TextureCreator<S>,
) {
    canvas.set_draw_color(Color::WHITE);
    canvas.set_blend_mode(BlendMode::None);
    canvas.fill_rect(rect.clone()).unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.draw_rect(rect.clone()).unwrap();

    let (text, text_color): (&str, _) = if input.is_empty() {
        (default, Color::GRAY)
    } else {
        (input, Color::BLACK)
    };
    copy_unscaled(
        canvas,
        rect.top_left().offset(6, 3),
        &text_box(text, &font, text_color, &texture_creator),
    );
}
