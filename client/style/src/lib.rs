use bevy::prelude::*;
use egui::{FontData, FontFamily, FontId, Style, TextStyle};

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(fonts);
        app.add_startup_system(style);
    }
}

fn style(mut ctx: ResMut<bevy_egui::EguiContext>) {
    let text_styles = [
        (
            TextStyle::Heading,
            FontId::new(24.00000, FontFamily::Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(12.00000, FontFamily::Monospace),
        ),
        (
            TextStyle::Button,
            FontId::new(12.00000, FontFamily::Monospace),
        ),
        (
            TextStyle::Monospace,
            FontId::new(12.00000, FontFamily::Monospace),
        ),
        (
            TextStyle::Small,
            FontId::new(06.00000, FontFamily::Monospace),
        ),
    ]
    .into();

    let visuals = egui::style::Visuals {
        hyperlink_color: egui::Color32::GOLD,
        ..Default::default()
    };

    let style = Style {
        text_styles,
        visuals,
        ..Default::default()
    };

    ctx.ctx_mut().set_style(style);
}

fn fonts(mut ctx: ResMut<bevy_egui::EguiContext>) {
    let font_data = [
        (
            "dreams".to_string(),
            FontData::from_static(embedded_assets::FONT_DREAMS),
        ),
        (
            "square".to_string(),
            FontData::from_static(embedded_assets::FONT_SQUARE),
        ),
        (
            "kongtext".to_string(),
            FontData::from_static(embedded_assets::FONT_KONGTEXT),
        ),
        (
            "november".to_string(),
            FontData::from_static(embedded_assets::FONT_NOVEMBER),
        ),
    ]
    .into();

    let families = [
        (
            FontFamily::Proportional,
            vec!["dreams".to_string(), "square".to_string()],
        ),
        (
            FontFamily::Monospace,
            vec!["kongtext".to_string(), "november".to_string()],
        ),
        (
            FontFamily::Name("dreams".into()),
            vec!["dreams".to_string()],
        ),
        (
            FontFamily::Name("square".into()),
            vec!["square".to_string()],
        ),
        (
            FontFamily::Name("kongtext".into()),
            vec!["kongtext".to_string()],
        ),
        (
            FontFamily::Name("november".into()),
            vec!["november".to_string()],
        ),
    ]
    .into();

    let font_definitions = egui::FontDefinitions {
        font_data,
        families,
    };

    ctx.ctx_mut().set_fonts(font_definitions);
}
