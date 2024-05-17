//! This example showcases a drawing a quad.
mod quad {
    use iced::advanced::graphics::color;
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::mouse;
    use iced::{Border, Color, Element, Length, Rectangle, Shadow, Size};

    pub struct CustomQuad {
        color: Color,
        size: f32,
        radius: [f32; 4],
        shadow: Shadow,
    }

    impl CustomQuad {
        pub fn new(color: Color, size: f32, radius: [f32; 4], shadow: Shadow) -> Self {
            Self {
                color,
                size,
                radius,
                shadow,
            }
        }
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for CustomQuad
    where
        Renderer: renderer::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Shrink,
                height: Length::Shrink,
            }
        }

        fn layout(
            &self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.size, self.size))
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        radius: self.radius.into(),
                        width: 0.0,
                        color: Color::from_rgb(1.0, 0.0, 0.0),
                    },
                    shadow: self.shadow,
                },
                self.color,
            );
        }
    }

    impl<'a, Message> From<CustomQuad> for Element<'a, Message> {
        fn from(circle: CustomQuad) -> Self {
            Self::new(circle)
        }
    }
}

use std::default;

use iced::border::{self, Radius};
use iced::theme::{self, Theme};
use iced::widget::container::Appearance;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, horizontal_space, row, slider, text,
};
use iced::{application, Border, Font, Shadow, Vector};
use iced::{gradient, window};
use iced::{Alignment, Background, Color, Element, Length, Radians, Sandbox, Settings};

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    Minimal::run(Settings {
        window: window::Settings {
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy)]
struct Minimal {
    start: Color,
    end: Color,
    angle: Radians,
    shadowed: bool,
    shadow: Shadow,
    radius: [f32; 4],
    quad_color: Color,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StartChanged(Color),
    EndChanged(Color),
    AngleChanged(Radians),
    ShadowColorChanged(Color),
    ShadowOffsetXChanged(f32),
    ShadowOffsetYChanged(f32),
    QuadColorChanged(Color),
}

impl Sandbox for Minimal {
    type Message = Message;

    fn new() -> Self {
        Self {
            start: Color::new(1.0, 0.5, 1.0, 1.0),
            end: Color::new(0.0, 0.0, 1.0, 1.0),
            angle: Radians(0.0),
            shadowed: false,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.8),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 16.0,
            },
            radius: [50.0; 4],
            quad_color: Color::from_rgba(1.0, 1.0, 1.0, 0.5),
        }
    }

    fn title(&self) -> String {
        String::from("Iced Widget Showcase")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartChanged(color) => self.start = color,
            Message::EndChanged(color) => self.end = color,
            Message::AngleChanged(angle) => self.angle = angle,
            Message::ShadowColorChanged(color) => self.shadow.color = color,
            Message::QuadColorChanged(color) => self.quad_color = color,
            Message::ShadowOffsetXChanged(x) => {
                self.shadow.offset.x = x;
            }
            Message::ShadowOffsetYChanged(y) => {
                self.shadow.offset.y = y;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Self {
            start,
            end,
            angle,
            shadowed,
            shadow,
            radius,
            quad_color,
        } = *self;

        // let gradient_box = container(horizontal_space())
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .style(move |_: &_| {
        //         let gradient = gradient::Linear::new(angle)
        //             .add_stop(0.0, start)
        //             .add_stop(1.0, end)
        //             .into();
        //
        //         container::Appearance {
        //             background: Some(Background::Gradient(gradient)),
        //             ..Default::default()
        //         }
        //     });
        //
        let ang = format!("{angle:.4}");

        let angle_picker = row![
            text("Angle").width(64),
            slider(Radians::RANGE, self.angle, Message::AngleChanged).step(0.01),
            text(format!("{ang:.4}")),
        ]
        .spacing(8)
        .padding(8)
        .align_items(Alignment::Center);

        column![
            column![
                row![text("Gradient").font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })]
                .padding(8)
                .spacing(8),
                container(horizontal_rule(0)).padding(8),
                row![
                    column![
                        color_picker("Start", self.start).map(Message::StartChanged),
                        color_picker("End", self.end).map(Message::EndChanged),
                        angle_picker,
                    ],
                    // container(gradient_box),
                ],
                container(horizontal_rule(0)).padding(8),
                row![text("Boxes").font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })]
                .padding(8)
                .spacing(8),
                column![
                    color_picker("Color:", self.quad_color).map(Message::QuadColorChanged),
                    color_picker("Shadow:", self.shadow.color).map(Message::ShadowColorChanged),
                    row![
                        text("Offset: ").width(64),
                        row![
                            text("X: "),
                            slider(
                                -100.0..=100.0,
                                self.shadow.offset.x,
                                Message::ShadowOffsetXChanged
                            )
                            .step(0.01),
                            text(format!(" {:.2}", self.shadow.offset.x)),
                        ],
                        row![
                            text("Y: "),
                            slider(
                                -50.0..=100.0,
                                self.shadow.offset.y,
                                Message::ShadowOffsetYChanged
                            )
                            .step(0.01),
                            text(format!(" {:.2}", self.shadow.offset.y)),
                        ],
                    ]
                    .padding(8)
                    .spacing(8),
                ],
                // container(horizontal_rule(0)).padding(8),
                // row![text("Cursor Interactions").font(Font {
                //     weight: iced::font::Weight::Bold,
                //     ..Default::default()
                // })]
                // .padding(8)
                // .spacing(8),
            ],
            container(
                row![
                    container(horizontal_space()),
                    container(quad::CustomQuad::new(
                        self.quad_color,
                        200.0,
                        self.radius,
                        self.shadow
                    ))
                    .padding([50, 0]),
                    container(quad::CustomQuad::new(
                        self.quad_color,
                        200.0,
                        self.radius,
                        self.shadow
                    ))
                    .padding([100, 50]),
                    container(quad::CustomQuad::new(
                        self.quad_color,
                        200.0,
                        self.radius,
                        self.shadow
                    ))
                    .padding([150, 25]),
                    container(horizontal_space()),
                ]
                .spacing(-200.0)
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .style(move |_: &_| {
                let gradient = gradient::Linear::new(angle)
                    .add_stop(0.0, start)
                    .add_stop(1.0, end)
                    .into();

                container::Appearance {
                    background: Some(Background::Gradient(gradient)),
                    ..Default::default()
                }
            }),
        ]
        .into()
    }
}

fn color_picker(label: &str, color: Color) -> Element<'_, Color> {
    row![
        text(label).width(64),
        row![
            text("R: "),
            slider(0.0..=1.0, color.r, move |r| { Color { r, ..color } }).step(0.01),
            text(format!(" {:.2}", color.r)),
        ],
        row![
            text("G: "),
            slider(0.0..=1.0, color.g, move |g| { Color { g, ..color } }).step(0.01),
            text(format!(" {:.2}", color.g)),
        ],
        row![
            text("B: "),
            slider(0.0..=1.0, color.b, move |b| { Color { b, ..color } }).step(0.01),
            text(format!(" {:.2}", color.b)),
        ],
        row![
            text("A: "),
            slider(0.0..=1.0, color.a, move |a| { Color { a, ..color } }).step(0.01),
            text(format!(" {:.2}", color.a)),
        ],
        quad::CustomQuad::new(
            Color::from_rgba(color.r, color.g, color.b, color.a),
            20.0,
            [4.0, 4.0, 4.0, 4.0],
            Shadow::default(),
        ),
    ]
    .spacing(8)
    .padding(8)
    .align_items(Alignment::Center)
    .into()
}
