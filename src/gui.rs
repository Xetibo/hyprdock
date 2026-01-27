use iced::keyboard::key::{Code, Named};
use iced::theme::Style;
use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length, Subscription, Task, Theme, event};
use oxiced::theme::theme_impl::get_derived_iced_theme;
use oxiced::utils::spacing::{_LARGE_SPACING, MEDIUM_SPACING};
use oxiced::widgets::oxi_button;
use oxiced::widgets::oxi_layer::{layer_theme, rounded_layer};

use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings};

use crate::monitors::hypr_monitors::{get_all_hypr_monitors, save_hypr_monitor_data};
use crate::{HyprDock, create_config_dir, parse_config};

// TODO make this configurable
const SCALE_FACTOR: f32 = 1.0;
const WINDOW_SIZE: (u32, u32) = (1200, 300);
const WINDOW_MARGINS: (i32, i32, i32, i32) = (100, 100, 100, 100);
const WINDOW_LAYER: Layer = Layer::Overlay;
const WINDOW_KEYBAORD_MODE: KeyboardInteractivity = KeyboardInteractivity::Exclusive;

pub fn run_gui() -> Result<(), iced_layershell::Error> {
    let default_anchor = Anchor::empty();
    let settings = Settings {
        layer_settings: LayerShellSettings {
            size: Some(WINDOW_SIZE),
            exclusive_zone: 0,
            anchor: default_anchor,
            layer: WINDOW_LAYER,
            margin: WINDOW_MARGINS,
            keyboard_interactivity: WINDOW_KEYBAORD_MODE,
            ..Default::default()
        },
        ..Default::default()
    };
    iced_layershell::application(
        HyprDockGui::new,
        HyprDockGui::namespace,
        HyprDockGui::update,
        HyprDockGui::view,
    )
    .subscription(HyprDockGui::subscription)
    .settings(settings)
    .theme(HyprDockGui::theme)
    .style(HyprDockGui::style)
    .scale_factor(HyprDockGui::scale_factor)
    .run()
}

struct HyprDockGui {
    theme: Theme,
    dock: HyprDock,
}

impl Default for HyprDockGui {
    fn default() -> Self {
        let monitors = get_all_hypr_monitors();
        let dock = parse_config(
            create_config_dir()
                .map(|path| path.join("hyprdock.toml"))
                .unwrap_or_default()
                .to_str()
                .expect("Could not convert path to string"),
            monitors,
        );
        Self {
            theme: get_derived_iced_theme(),
            dock,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    InternalMonitor,
    ExternalMonitor,
    ExtendMonitor,
    MirrorMonitor,
    SaveMonitors,
}

impl TryInto<iced_layershell::actions::LayershellCustomActionWithId> for Message {
    type Error = Self;
    fn try_into(
        self,
    ) -> Result<iced_layershell::actions::LayershellCustomActionWithId, Self::Error> {
        Err(self)
    }
}

impl HyprDockGui {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn namespace() -> String {
        String::from("HyprDock")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => std::process::exit(0),
            Message::InternalMonitor => {
                self.dock.internal_monitor();
                std::process::exit(0);
            }
            Message::ExternalMonitor => {
                self.dock.external_monitor();
                std::process::exit(0);
            }
            Message::ExtendMonitor => {
                self.dock.extend_monitor();
                std::process::exit(0);
            }
            Message::MirrorMonitor => {
                self.dock.mirror_monitor();
                std::process::exit(0);
            }
            Message::SaveMonitors => {
                save_hypr_monitor_data(self.dock.monitor_config_path.clone());
                std::process::exit(0);
            }
        }
    }

    fn view(&'_ self) -> Element<'_, Message> {
        let internal = oxi_button::button(
            column!(text("Internal Monitor only"))
                .align_x(Alignment::Center)
                .width(Length::Fill),
            oxi_button::ButtonVariant::PrimaryBg,
        )
        .on_press(Message::InternalMonitor)
        .width(Length::FillPortion(1));
        let external = oxi_button::button(
            column!(text("External Monitors only"))
                .align_x(Alignment::Center)
                .width(Length::Fill),
            oxi_button::ButtonVariant::PrimaryBg,
        )
        .on_press(Message::ExternalMonitor)
        .width(Length::FillPortion(1));
        let extend = oxi_button::button(
            column!(text("Extend Monitors"))
                .align_x(Alignment::Center)
                .width(Length::Fill),
            oxi_button::ButtonVariant::PrimaryBg,
        )
        .on_press(Message::ExtendMonitor)
        .width(Length::FillPortion(1));
        let mirror = oxi_button::button(
            column!(text("Mirror Monitors"))
                .align_x(Alignment::Center)
                .width(Length::Fill),
            oxi_button::ButtonVariant::PrimaryBg,
        )
        .on_press(Message::MirrorMonitor)
        .width(Length::FillPortion(1));
        let save = oxi_button::button(
            column!(text("Save Configuration"))
                .align_x(Alignment::Center)
                .width(Length::Fill),
            oxi_button::ButtonVariant::PrimaryBg,
        )
        .on_press(Message::SaveMonitors)
        .width(Length::FillPortion(1));
        let row = row!(internal, external, extend, mirror, save)
            .width(Length::Fill)
            .spacing(_LARGE_SPACING)
            .padding(MEDIUM_SPACING);
        rounded_layer(row, WINDOW_SIZE)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(move |event, _status, _id| match event {
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers: _modifier,
                key,
                modified_key: _,
                physical_key,
                location: _,
                text: _,
                repeat: _repeat,
            }) => match (key, physical_key) {
                (iced::keyboard::key::Key::Named(Named::Escape), _) => Some(Message::Exit),
                (_, iced::keyboard::key::Physical::Code(Code::Digit1)) => {
                    Some(Message::InternalMonitor)
                }
                (_, iced::keyboard::key::Physical::Code(Code::Digit2)) => {
                    Some(Message::ExternalMonitor)
                }
                (_, iced::keyboard::key::Physical::Code(Code::Digit3)) => {
                    Some(Message::ExtendMonitor)
                }
                (_, iced::keyboard::key::Physical::Code(Code::Digit4)) => {
                    Some(Message::MirrorMonitor)
                }
                (_, iced::keyboard::key::Physical::Code(Code::Digit5)) => {
                    Some(Message::SaveMonitors)
                }
                _ => None,
            },
            _ => None,
        })
    }

    // remove the annoying background color
    fn style(&self, _: &Theme) -> Style {
        layer_theme()
    }

    fn scale_factor(&self) -> f32 {
        SCALE_FACTOR
    }
}
