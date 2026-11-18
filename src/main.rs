use iced::widget::{button, column, container, row, text, text_editor, Space};
use iced::{Element, Length, Subscription, Task, Theme, Color, Background, Border, Shadow, Font};
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application("AeroTxt", AeroTxt::update, AeroTxt::view)
        .subscription(AeroTxt::subscription)
        .theme(AeroTxt::theme)
        .window_size((1000.0, 700.0))
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
                dark_mode: true,
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
                        .add_filter("Text", &["txt", "rs", "py", "js", "md", "toml", "json", "html", "css", "c", "cpp", "h", "go", "java", "lua"])
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
        let (bg, toolbar_bg, border_color, text_color, accent, btn_hover, btn_pressed, muted, selection) = if self.dark_mode {
            (
                Color::from_rgb(0.12, 0.12, 0.14),
                Color::from_rgb(0.16, 0.16, 0.18),
                Color::from_rgb(0.22, 0.22, 0.26),
                Color::from_rgb(0.92, 0.92, 0.95),
                Color::from_rgb(0.30, 0.60, 0.95),
                Color::from_rgb(0.22, 0.22, 0.26),
                Color::from_rgb(0.14, 0.14, 0.17),
                Color::from_rgb(0.50, 0.50, 0.55),
                Color::from_rgb(0.25, 0.40, 0.65),
            )
        } else {
            (
                Color::from_rgb(0.98, 0.98, 0.99),
                Color::from_rgb(0.90, 0.90, 0.93),
                Color::from_rgb(0.80, 0.80, 0.85),
                Color::from_rgb(0.15, 0.15, 0.20),
                Color::from_rgb(0.20, 0.45, 0.85),
                Color::from_rgb(0.82, 0.82, 0.85),
                Color::from_rgb(0.76, 0.76, 0.79),
                Color::from_rgb(0.45, 0.45, 0.50),
                Color::from_rgb(0.75, 0.85, 0.95),
            )
        };

        let make_btn = |label: &'static str, msg: Message| {
            let tc = text_color;
            let bc = toolbar_bg;
            let ac = accent;
            let bh = btn_hover;
            let bp = btn_pressed;
            button(text(label).size(13).color(tc))
                .on_press(msg)
                .padding([6, 12])
                .style(move |_theme, status| {
                    let bg_color = match status {
                        iced::widget::button::Status::Hovered => bh,
                        iced::widget::button::Status::Pressed => bp,
                        _ => bc,
                    };
                    iced::widget::button::Style {
                        background: Some(Background::Color(bg_color)),
                        text_color: tc,
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 4.0.into(),
                        },
                        shadow: Shadow::default(),
                        ..Default::default()
                    }
                })
        };

        let title_bar = container(
            row![
                text("AeroTxt").size(15).color(accent).font(Font::MONOSPACE),
                Space::with_width(Length::Fill),
                make_btn(if self.dark_mode { "☀" } else { "🌙" }, Message::ToggleTheme),
            ]
            .align_y(iced::Alignment::Center)
            .padding([4, 12]),
        )
        .width(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(toolbar_bg)),
            border: Border {
                color: border_color,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        let toolbar = container(
            row![
                make_btn("New", Message::NewTab),
                make_btn("Open", Message::OpenFile),
                make_btn("Save", Message::SaveFile),
            ]
            .spacing(4)
            .padding([6, 8]),
        )
        .width(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(
                toolbar_bg.r * 0.95,
                toolbar_bg.g * 0.95,
                toolbar_bg.b * 0.95,
            ))),
            border: Border {
                color: border_color,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        let editor = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill)
            .font(iced::Font::MONOSPACE)
            .size(14)
            .padding(12)
            .style(move |_theme, _status| text_editor::Style {
                background: Background::Color(bg),
                border: Border {
                    color: border_color,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                icon: muted,
                placeholder: Color::from_rgb(0.4, 0.4, 0.45),
                value: text_color,
                selection: selection,
            });

        let status_bar = container(
            row![
                text(&self.status).size(12).color(muted),
                Space::with_width(Length::Fill),
                text(if self.file_path.is_some() {
                    self.file_path.as_ref().unwrap().display().to_string()
                } else {
                    "Untitled".to_string()
                })
                .size(12)
                .color(muted),
            ]
            .padding([4, 12]),
        )
        .width(Length::Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(toolbar_bg)),
            border: Border {
                color: border_color,
                width: 0.0,
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
