use std::{fs, time::Duration};

use sdl::game_window::GameWindow;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{FullscreenType, Window},
};

use crate::{
    level::{config::LevelConfig, Level},
    save::SaveFile,
    textures::{draw_string, load_textures},
    UPDATE_AVAILABLE,
};

pub struct Win {
    running: bool,
    pause: bool,
    save: SaveFile,

    levels_count: u8,
    level: Option<Level>,
}

impl Win {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        load_textures(canvas, Box::leak(Box::new(canvas.texture_creator())))?;
        let levels_count = fs::read_dir("levels").map_err(|e| e.to_string())?.count();
        if levels_count == 0 || fs::read_dir("levels").map_err(|e| e.to_string())?.count() > 99 {
            return Err("Too much or no levels".to_owned());
        }

        Ok(Self {
            running: true,
            pause: false,
            save: SaveFile::load()?,
            levels_count: levels_count as u8,
            level: None,
        })
    }
}

impl GameWindow for Win {
    fn running(&mut self) -> bool {
        self.running
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        if self.pause {
            return Ok(());
        }
        if let Some(level) = self.level.as_mut() {
            level.update(canvas, elapsed)?;
        }
        Ok(())
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let scale_x = |x: i32| x as f32 * 1280. / width as f32;
        let scale_y = |y: i32| y as f32 * 720. / height as f32;

        match event {
            Event::Quit { .. } => self.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.pause = !self.pause,
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                change_full_screen(canvas)?;
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let x = scale_x(x) as i32;
                let y = scale_y(y) as i32;
                if self.level.is_some() {
                    if self.pause && (565..=715).contains(&x) {
                        if (200..=240).contains(&y) {
                            self.save.next_lang()?;
                        } else if (260..=300).contains(&y) {
                            change_full_screen(canvas)?;
                        } else if (320..=360).contains(&y) {
                            self.level = None;
                            self.pause = false;
                        } else if (380..=420).contains(&y) {
                            self.running = false;
                        }
                    }
                } else if (485..=635).contains(&x) {
                    if (200..=240).contains(&y) {
                        self.save.next_lang()?;
                    } else if (260..=300).contains(&y) {
                        change_full_screen(canvas)?;
                    } else if (320..=360).contains(&y) {
                        self.running = false;
                    }
                } else if (645..=796).contains(&x) && y >= 200 {
                    let y = (y as f32 - 200.) / 60.;
                    if (y - y.floor()) <= 40. / 60. {
                        let y = y.floor() as u8;
                        if (0..self.levels_count).contains(&y) {
                            self.level =
                                Some(LevelConfig::load_config(y).map_err(|e| e.to_string())?);
                        }
                    }
                }
            }
            _ => {}
        }
        if let Some(level) = self.level.as_mut() {
            level.event(canvas, event.clone(), &mut self.pause)?;
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        set_scale(canvas, 1., 1.)?;
        if let Some(level) = self.level.as_ref() {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            return self.draw_with_level(canvas, level);
        }
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        self.draw_main_menu(canvas)
    }
}

impl Win {
    fn draw_with_level(&self, canvas: &mut Canvas<Window>, level: &Level) -> Result<(), String> {
        level.draw(canvas, &self.save)?;
        if self.pause {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(Rect::new(565, 200, 150, 40))?;
            canvas.fill_rect(Rect::new(565, 260, 150, 40))?;
            canvas.fill_rect(Rect::new(565, 320, 150, 40))?;
            canvas.fill_rect(Rect::new(565, 380, 150, 40))?;

            draw_string(canvas, Rect::new(575, 206, 130, 28), self.save.texts().lang)?;
            draw_string(
                canvas,
                Rect::new(575, 266, 130, 28),
                self.save.texts().full_screen,
            )?;
            draw_string(
                canvas,
                Rect::new(575, 326, 130, 28),
                self.save.texts()._return,
            )?;
            draw_string(canvas, Rect::new(575, 386, 130, 28), self.save.texts().quit)?;
        }
        Ok(())
    }

    fn draw_main_menu(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(485, 200, 150, 40))?;
        canvas.fill_rect(Rect::new(485, 260, 150, 40))?;
        canvas.fill_rect(Rect::new(485, 320, 150, 40))?;
        for i in 0..self.levels_count {
            canvas.fill_rect(Rect::new(645, 200 + i as i32 * 60, 150, 40))?;
        }
        canvas.fill_rect(Rect::new(570, 670, 700, 40))?;

        draw_string(canvas, Rect::new(495, 206, 130, 28), self.save.texts().lang)?;
        draw_string(
            canvas,
            Rect::new(495, 266, 130, 28),
            self.save.texts().full_screen,
        )?;
        draw_string(
            canvas,
            Rect::new(495, 326, 130, 28),
            self.save.texts()._return,
        )?;

        for i in 0..self.levels_count {
            draw_string(
                canvas,
                Rect::new(654, 206 + i as i32 * 60, 130, 28),
                format!("{:0>3}", i + 1).as_str(),
            )?;
        }

        draw_string(
            canvas,
            Rect::new(570, 670, 700, 40),
            match unsafe { UPDATE_AVAILABLE.as_ref() } {
                Some(Ok(true)) => self.save.texts().update_available,
                Some(Ok(false)) => self.save.texts().up_to_date,
                Some(Err(e)) => e,
                None => self.save.texts().loading,
            },
        )
    }
}

fn change_full_screen(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let window = canvas.window_mut();
    window.set_fullscreen(if window.fullscreen_state() == FullscreenType::Off {
        FullscreenType::Desktop
    } else {
        FullscreenType::Off
    })?;
    Ok(())
}

pub fn set_scale(canvas: &mut Canvas<Window>, scale_x: f32, scale_y: f32) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    canvas.set_scale(
        scale_x * width as f32 / 1280.,
        scale_y * height as f32 / 720.,
    )
}
