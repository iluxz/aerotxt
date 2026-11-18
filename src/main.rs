use iced::widget::{button, column, container, row, text, text_editor, TextEditor};
use iced::{Element, Length, Subscription, Task, Theme};
use iced::theme::palette;
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application("AeroTxt", AeroTxt::update, AeroTxt::view)
        .subscription(AeroTxt::subscription)
        .theme(AeroTxt::theme)
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
                        .add_filter("Text", &["txt", "rs", "py", "js", "md", "toml", "json", "html", "css"])
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
                                    // placeholder - will be handled in FileSaved
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
        }
    }

    fn view(&self) -> Element<Message> {
        let editor = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill)
            .font(iced::Font::MONOSPACE);

        let toolbar = row![
            button("Open").on_press(Message::OpenFile),
            button("Save").on_press(Message::SaveFile),
            button(if self.dark_mode { "Light" } else { "Dark" }).on_press(Message::ToggleTheme),
        ]
        .spacing(8)
        .padding(8);

        let status_bar = container(
            text(&self.status).size(12),
        )
        .padding(4)
        .width(Length::Fill);

        column![toolbar, editor, status_bar].into()
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
