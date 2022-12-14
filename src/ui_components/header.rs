use bevy::prelude::*;

use super::primitives::{button_with_tag, text};
use crate::{spawn_get_stories_async, theme::*, SelectedStory};

pub struct HeaderPlugin;
impl Plugin for HeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_interaction_header);
    }
}

#[derive(Component)]
struct HeaderButton {
    value: String,
}

pub fn header(
    c: &mut ChildBuilder,
    text_style: &TextStyle,
    value: &str,
    text_content: &str,
) -> Entity {
    button_with_tag(
        c,
        &Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect {
                left: Val::Px(10.),
                right: Val::Px(10.),
                ..Default::default()
            },
            size: Size::new(Val::Auto, Val::Px(50.)),
            ..Default::default()
        },
        HeaderButton {
            value: value.to_string(),
        },
        |c| {
            text(c, None, text_style, text_content);
        },
    )
}

fn on_interaction_header(
    mut commands: Commands,
    mut query: Query<
        (&Interaction, &HeaderButton, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_story: ResMut<SelectedStory>,
) {
    for (interaction, HeaderButton { value }, mut color) in &mut query {
        match interaction {
            Interaction::Clicked => {
                info!("Header clicked {value}");
                *color = Color::BLACK.into();
                spawn_get_stories_async(&mut commands, value.clone());
                selected_story.0 = None;
            }
            Interaction::Hovered => {
                *color = BG_ORANGE_700.into();
            }
            Interaction::None => {
                *color = BG_ORANGE_600.into();
            }
        }
    }
}
