use iced::mouse;
use iced::widget::canvas::{self, Canvas, Geometry, Image as CanvasImage, Program};
use iced::widget::image;
use iced::{exit, Color, Element, Length, Rectangle, Renderer, Task, Theme};
use iced_layershell::application;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::to_layer_message;
use rand::Rng;
use std::time::Duration;

pub fn main() -> Result<(), iced_layershell::Error> {
    application(|| LucyWindow::new(), namespace, update, view)
        .style(style)
        .subscription(subscription)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((2880, 1800)),
                anchor: Anchor::Top | Anchor::Left | Anchor::Right | Anchor::Bottom,
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone)]
struct FallingImage {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    rotation_angle: f32, // Added for current rotation
    rotation_speed: f32, // Added for rotation speed
}

struct LucyWindow {
    falling_images: Vec<FallingImage>,
    spawn_timer: f32,
    image_handle: image::Handle,
}

impl LucyWindow {
    fn new() -> Self {
        static LUCY_BYTES: &[u8] = include_bytes!("../lucy.png");
        Self {
            falling_images: Vec::new(),
            spawn_timer: 0.0,
            image_handle: image::Handle::from_bytes(LUCY_BYTES),
        }
    }
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    Tick(std::time::Instant),
    Event(iced::Event),
}

fn namespace() -> String {
    String::from("Lucy Snow")
}

fn subscription(_state: &LucyWindow) -> iced::Subscription<Message> {
    iced::Subscription::batch([
        iced::time::every(Duration::from_millis(16)).map(Message::Tick),
        iced::event::listen().map(Message::Event),
    ])
}

fn update(state: &mut LucyWindow, message: Message) -> Task<Message> {
    match message {
        Message::Tick(_instant) => {
            let mut rng = rand::rng();

            // Update spawn timer
            state.spawn_timer += 0.016;

            // Spawn new images periodically
            if state.spawn_timer >= 0.2 {
                state.spawn_timer = 0.0;

                let spawn_count = rng.random_range(1..=2);
                for _ in 0..spawn_count {
                    state.falling_images.push(FallingImage {
                        x: rng.random_range(0.0..2880.0),
                        y: -150.0,
                        size: rng.random_range(60.0..120.0),
                        speed: rng.random_range(150.0..400.0),
                        rotation_angle: rng.random_range(0.0..std::f32::consts::TAU), // Initialize with a random angle
                        rotation_speed: rng.random_range(-0.5..0.5), // Random slow rotation speed
                    });
                }
            }

            // Update positions and rotations
            for img in &mut state.falling_images {
                img.y += img.speed * 0.016;
                img.rotation_angle += img.rotation_speed * 0.016;
            }

            // Remove off-screen
            state.falling_images.retain(|img| img.y < 1800.0 + 150.0);

            Task::none()
        }
        Message::Event(iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
            ..
        })) => exit(),
        _ => Task::none(),
    }
}

impl Program<Message> for LucyWindow {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for falling in &self.falling_images {
            // Save the current frame state
            frame.with_save(|frame| {
                // Translate to the center of the image for rotation
                frame.translate(iced::Vector::new(falling.x, falling.y));
                // Rotate by the current angle
                frame.rotate(falling.rotation_angle);
                // Translate back to draw the image centered
                frame.translate(iced::Vector::new(-falling.x, -falling.y));

                frame.draw_image(
                    Rectangle {
                        x: falling.x - falling.size / 2.0,
                        y: falling.y - falling.size / 2.0,
                        width: falling.size,
                        height: falling.size,
                    },
                    CanvasImage::new(self.image_handle.clone()),
                );
            });
        }

        vec![frame.into_geometry()]
    }
}

fn view(state: &LucyWindow) -> Element<'_, Message> {
    Canvas::new(state)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn style(_state: &LucyWindow, _theme: &iced::Theme) -> iced::theme::Style {
    iced::theme::Style {
        background_color: Color::TRANSPARENT,
        text_color: Color::BLACK,
    }
}
