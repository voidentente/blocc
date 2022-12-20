use bevy::prelude::*;

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(fonts);
    }
}

fn fonts(mut ctx: ResMut<bevy_egui::EguiContext>) {
    let mut font_data = std::collections::BTreeMap::new();
    let mut families = std::collections::BTreeMap::new();

    {
        {
            let data = egui::FontData {
                font: std::borrow::Cow::Borrowed(assets::FONT_HEAD),
                index: 0,
                tweak: egui::FontTweak::default(),
            };
            font_data.insert("blocc_head".to_string(), data);
        }

        {
            let data = egui::FontData {
                font: std::borrow::Cow::Borrowed(assets::FONT_BODY),
                index: 0,
                tweak: egui::FontTweak::default(),
            };
            font_data.insert("blocc_body".to_string(), data);
        }

        let font_family = egui::FontFamily::Proportional;

        families.insert(
            font_family,
            vec!["blocc_head".to_string(), "blocc_body".to_string()],
        );
    }

    {
        {
            let data = egui::FontData {
                font: std::borrow::Cow::Borrowed(assets::FONT_MONO_KONGT),
                index: 0,
                tweak: egui::FontTweak::default(),
            };
            font_data.insert("blocc_mono_kongt".to_string(), data);
        }

        {
            let data = egui::FontData {
                font: std::borrow::Cow::Borrowed(assets::FONT_MONO_NOVEM),
                index: 0,
                tweak: egui::FontTweak::default(),
            };
            font_data.insert("blocc_mono_novem".to_string(), data);
        }

        let font_family = egui::FontFamily::Monospace;

        families.insert(
            font_family,
            vec![
                "blocc_mono_kongt".to_string(),
                "blocc_mono_novem".to_string(),
            ],
        );
    }

    let font_definitions = egui::FontDefinitions {
        font_data,
        families,
    };

    ctx.ctx_mut().set_fonts(font_definitions);
}
