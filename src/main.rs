use iced::widget::{button, column, container, row, text, text_editor, Space};
use iced::{Element, Length, Subscription, Task, Theme, Color, Background, Border, Shadow};
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application("AeroTxt", AeroTxt::update, AeroTxt::view)
        .subscription(AeroTxt::subscription)
        .theme(AeroTxt::theme)
        .window_size((900.0, 650.0))
        .run_with(AeroTxt::new)
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    OpenFile,
    SaveFile,
    FileOpened(Option<(PathBuf, String)>),
    FileSaved(Option<PathBuf>),
    ToggleTheme,
    NewTab,
}

struct AeroTxt {
    content: text_editor::Content,
    file_path: Option<PathBuf>,
    status: String,
    dark_mode: bool,
}

impl AeroTxt {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::new(),
                file_path: None,
                status: "Ready".to_string(),
                dark_mode: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Task::none()
            }
            Message::OpenFile => Task::perform(
                async {
                    let file = FileDialog::new()
                        .add_filter("Text", &["txt", "rs", "py", "js", "md", "toml", "json", "html", "css", "c", "cpp", "h", "go", "java"])
                        .add_filter("All", &["*"])
                        .pick_file();
                    match file {
                        Some(path) => {
                            let content = std::fs::read_to_string(&path).ok()?;
                            Some((path, content))
                        }
                        None => None,
                    }
                },
                Message::FileOpened,
            ),
            Message::FileOpened(result) => {
                if let Some((path, content)) = result {
                    self.content = text_editor::Content::with_text(&content);
                    self.status = format!("Opened: {}", path.display());
                    self.file_path = Some(path);
                }
                Task::none()
            }
            Message::SaveFile => {
                if let Some(path) = self.file_path.clone() {
                    let content = self.content.text();
                    Task::perform(
                        async move {
                            std::fs::write(&path, content).ok()?;
                            Some(path)
                        },
                        Message::FileSaved,
                    )
                } else {
                    Task::perform(
                        async {
                            let file = FileDialog::new()
                                .add_filter("Text", &["txt", "rs", "py", "js", "md", "toml", "json", "html", "css"])
                                .save_file();
                            match file {
                                Some(path) => {
                                    let content = String::new();
                                    std::fs::write(&path, content).ok()?;
                                    Some(path)
                                }
                                None => None,
                            }
                        },
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                if let Some(path) = result {
                    self.status = format!("Saved: {}", path.display());
                    self.file_path = Some(path);
                }
                Task::none()
            }
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                Task::none()
            }
            Message::NewTab => {
                self.content = text_editor::Content::new();
                self.file_path = None;
                self.status = "New file".to_string();
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let (bg, toolbar_bg, border_color, text_color, accent, btn_hover, btn_pressed) = if self.dark_mode {
            (
                Color::from_rgb(0.12, 0.12, 0.16),
                Color::from_rgb(0.16, 0.16, 0.20),
                Color::from_rgb(0.25, 0.25, 0.30),
                Color::from_rgb(0.90, 0.90, 0.95),
                Color::from_rgb(0.40, 0.60, 0.90),
                Color::from_rgb(0.22, 0.22, 0.28),
                Color::from_rgb(0.18, 0.18, 0.24),
            )
        } else {
            (
                Color::from_rgb(0.92, 0.94, 0.97),
                Color::from_rgb(0.85, 0.88, 0.93),
                Color::from_rgb(0.70, 0.75, 0.82),
                Color::from_rgb(0.10, 0.10, 0.15),
                Color::from_rgb(0.20, 0.45, 0.80),
                Color::from_rgb(0.78, 0.82, 0.88),
                Color::from_rgb(0.72, 0.76, 0.82),
            )
        };

        let aero_button = |label: &'static str, msg: Message, tc: Color, bc: Color, ac: Color, bh: Color, bp: Color| {
            button(text(label).size(13).color(tc))
                .on_press(msg)
                .padding([6, 14])
                .style(move |_theme, status| {
                    let (bg_color, border_color) = match status {
                        iced::widget::button::Status::Hovered => (bh, ac),
                        iced::widget::button::Status::Pressed => (bp, ac),
                        _ => (bc, border_color),
                    };
                    iced::widget::button::Style {
                        background: Some(Background::Color(bg_color)),
                        text_color: tc,
                        border: Border {
                            color: border_color,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        shadow: Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                            offset: iced::Vector::new(0.0, 1.0),
                            blur_radius: 2.0,
                        },
                        ..Default::default()
                    }
                })
        };

        let title_bar = container(
            row![
                text("AeroTxt").size(14).color(text_color),
                Space::with_width(Length::Fill),
                aero_button(
                    if self.dark_mode { "☀" } else { "🌙" },
                    Message::ToggleTheme,
                    text_color,
                    toolbar_bg,
                    accent,
                    btn_hover,
                    btn_pressed,
                ),
            ]
            .align_y(iced::Alignment::Center)
            .padding([4, 12]),
        )
        .width(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(toolbar_bg)),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        let toolbar = container(
            row![
                aero_button("New", Message::NewTab, text_color, toolbar_bg, accent, btn_hover, btn_pressed),
                aero_button("Open", Message::OpenFile, text_color, toolbar_bg, accent, btn_hover, btn_pressed),
                aero_button("Save", Message::SaveFile, text_color, toolbar_bg, accent, btn_hover, btn_pressed),
            ]
            .spacing(6)
            .padding([4, 8]),
        )
        .width(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(
                toolbar_bg.r * 0.96,
                toolbar_bg.g * 0.96,
                toolbar_bg.b * 0.96,
            ))),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        let editor = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill)
            .font(iced::Font::MONOSPACE)
            .size(14)
            .padding(10)
            .style(move |_theme, _status| text_editor::Style {
                background: Background::Color(bg),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                icon: text_color,
                placeholder: Color::from_rgb(0.5, 0.5, 0.55),
                value: text_color,
                selection: Color::from_rgb(accent.r, accent.g, accent.b),
            });

        let status_bar = container(
            row![
                text(&self.status).size(12).color(text_color),
                Space::with_width(Length::Fill),
                text(if self.file_path.is_some() {
                    self.file_path.as_ref().unwrap().display().to_string()
                } else {
                    "Untitled".to_string()
                })
                .size(12)
                .color(Color::from_rgb(0.5, 0.5, 0.55)),
            ]
            .padding([2, 12]),
        )
        .width(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(toolbar_bg)),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        column![title_bar, toolbar, editor, status_bar].into()
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
