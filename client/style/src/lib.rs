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

    let style = Style {
        text_styles,
        ..Default::default()
    };

    ctx.ctx_mut().set_style(style);
}

fn fonts(mut ctx: ResMut<bevy_egui::EguiContext>) {
    let font_data = [
        (
            "blocc_dada".to_string(),
            FontData::from_static(assets::FONT_DADA),
        ),
        (
            "blocc_sqre".to_string(),
            FontData::from_static(assets::FONT_SQRE),
        ),
        (
            "blocc_kong".to_string(),
            FontData::from_static(assets::FONT_KONG),
        ),
        (
            "blocc_novm".to_string(),
            FontData::from_static(assets::FONT_NOVM),
        ),
    ]
    .into();

    let families = [
        (
            FontFamily::Proportional,
            vec!["blocc_dada".to_string(), "blocc_sqre".to_string()],
        ),
        (
            FontFamily::Monospace,
            vec!["blocc_kong".to_string(), "blocc_novm".to_string()],
        ),
    ]
    .into();

    let font_definitions = egui::FontDefinitions {
        font_data,
        families,
    };

    ctx.ctx_mut().set_fonts(font_definitions);
}
