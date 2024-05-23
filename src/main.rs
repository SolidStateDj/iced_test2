mod quad;
use std::default;

use iced::border::{self, Radius};
use iced::theme::{self, Theme};
use iced::widget::container::Appearance;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, horizontal_space, row, slider, text,
};
use iced::{
    application, event, executor, Application, Border, Command, Event, Font, Shadow, Subscription,
    Vector,
};
use iced::{gradient, window};
use iced::{Alignment, Background, Color, Element, Length, Radians, Settings};
use tracing_subscriber::layer::SubscriberExt;

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

#[derive(Debug, Clone, Default)]
struct EventsWidget {
    last: Vec<Event>,
}

#[derive(Debug, Default, Clone)]
struct Minimal {
    start: Color,
    end: Color,
    angle: Radians,
    shadow: Shadow,
    radius: [f32; 4],
    quad_color: Color,
    events: EventsWidget,
}

#[derive(Debug, Clone)]
enum Message {
    StartChanged(Color),
    EndChanged(Color),
    AngleChanged(Radians),
    ShadowColorChanged(Color),
    ShadowOffsetXChanged(f32),
    ShadowOffsetYChanged(f32),
    QuadColorChanged(Color),
    EventHappened(Event),
}

impl Application for Minimal {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Minimal, Command<Message>) {
        (
            Self {
                start: Color::new(1.0, 0.5, 1.0, 1.0),
                end: Color::new(0.0, 0.0, 1.0, 1.0),
                angle: Radians(0.0),
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.8),
                    offset: Vector::new(0.0, 8.0),
                    blur_radius: 16.0,
                },
                radius: [50.0; 4],
                quad_color: Color::from_rgba(1.0, 1.0, 1.0, 0.5),
                events: EventsWidget {
                    ..Default::default()
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced Widget Showcase")
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventHappened)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
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
            Message::EventHappened(event) => {
                self.events.last.push(event);

                if self.events.last.len() > 2 {
                    let _ = self.events.last.remove(0);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let angle = self.angle;
        let ang = format!("{angle:.4}");
        // if !self.events.last.is_empty() {
        //     let event = self.events.last[0];
        //     let event_widget = column![text(format!("{event:?}"))];
        // }

        let start = self.start;
        let end = self.end;

        let angle_picker = row![
            text("Angle").width(64),
            slider(Radians::RANGE, self.angle, Message::AngleChanged).step(0.01),
            text(format!("{ang:.4}")),
            // text("0.00"),
        ]
        .spacing(8)
        .padding(8)
        .align_items(Alignment::Center);

        let mut event_widget = text("Nothing Yet...");
        if !self.events.last.is_empty() {
            event_widget = text(format!("{:?}", self.events.last[0]))
        }
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
                container(horizontal_rule(0)).padding(8),
                column![
                    row![text("Cursor Interactions").font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })]
                    .padding(8)
                    .spacing(8),
                    // row![text(format!("{:?}", self.events.last[0]))],
                    row![text("Most Recent Event:"), event_widget,]
                        .padding(8)
                        .spacing(8),
                ],
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
    .padding(8)
    .spacing(8)
    .align_items(Alignment::Center)
    .into()
}
