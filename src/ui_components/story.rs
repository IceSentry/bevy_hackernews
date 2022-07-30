use bevy::prelude::*;

use super::primitives::{
    button_with_component, div_with_style, text, text_section, text_sections,
    text_sections_with_style, text_with_style,
};
use crate::{
    api::{get_story_comments, HackerNewsStory},
    utils::num_as_f32,
    SelectedStory,
};

pub struct StoryComponentPlugin;
impl Plugin for StoryComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_interaction_story);
    }
}

#[derive(Component)]
struct StoryButton {
    id: f64,
}

pub fn story(
    c: &mut ChildBuilder,
    title_style: &TextStyle,
    dark_style: &TextStyle,
    index: usize,
    story: &HackerNewsStory,
) {
    let btn_style = Style {
        flex_shrink: 0.,
        flex_direction: FlexDirection::Row,
        padding: UiRect {
            top: Val::Px(5.),
            bottom: Val::Px(5.),
            ..default()
        },
        ..Default::default()
    };

    let tag = StoryButton {
        id: story.id.as_f64().unwrap(),
    };
    button_with_component(c, &btn_style, tag, |c| {
        let style = Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        };
        //index
        div_with_style(c, &style, |c| {
            let style = Style {
                margin: UiRect {
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    ..Default::default()
                },
                ..default()
            };
            text_with_style(c, &style, dark_style, &format!("{}.", index + 1));
        });

        // title and meta
        let style = Style {
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        };
        div_with_style(c, &style, |c| {
            // title
            let style = Style {
                align_items: AlignItems::Center,
                ..Default::default()
            };

            div_with_style(c, &style, |c| {
                text_sections(
                    c,
                    [
                        (title_style.clone(), story.title.to_string()),
                        (
                            dark_style.clone(),
                            format!(" ({})", story.domain.as_ref().unwrap_or(&String::from(""))),
                        ),
                    ],
                );
            });

            // meta
            let style = Style {
                align_items: AlignItems::Center,
                ..Default::default()
            };
            let meta = format!(
                "{} points by {} | {} comments",
                num_as_f32(&story.points),
                story.user.as_ref().unwrap_or(&String::from("undefined")),
                story.comments_count.as_f64().unwrap_or(0.0),
            );
            text_with_style(c, &style, dark_style, &meta);
        });
    });
}

fn on_interaction_story(
    mut query: Query<
        (&Interaction, &StoryButton, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_story: ResMut<SelectedStory>,
) {
    for (interaction, StoryButton { id }, mut color) in &mut query {
        match interaction {
            Interaction::Clicked => {
                info!("story {id} clicked");
                let comments = get_story_comments(&id.to_string()).expect("failed to get comments");
                selected_story.0 = Some(comments);
            }
            Interaction::Hovered => {
                *color = Color::rgba(0.0, 0.0, 0.0, 0.25).into();
            }
            Interaction::None => {
                *color = Color::NONE.into();
            }
        }
    }
}
