use iced::widget::container;
use iced::Color;
use iced_native::color;

pub struct CustomContainer;

impl container::StyleSheet for CustomContainer {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let appearance = style.appearance(&iced::theme::Container::default());

        container::Appearance {
            background: Some(iced::Background::Color(color!(25, 25, 35))),
            border_radius: 10.0,
            ..appearance
        }
    }
}

impl From<CustomContainer> for iced::theme::Container {
    fn from(style: CustomContainer) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}
