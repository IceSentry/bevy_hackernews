use bevy::prelude::*;

use super::primitives::{button_with_tag, container, text, text_sections};
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
        ..default()
    };

    let tag = StoryButton {
        id: story.id.as_f64().unwrap(),
    };
    button_with_tag(c, &btn_style, tag, |c| {
        text(
            c,
            Some(Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                margin: UiRect {
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    ..default()
                },
                ..default()
            }),
            dark_style,
            &format!("{}.", index + 1),
        );

        container(
            c,
            None,
            Some(Style {
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            }),
            |c| {
                title(c, title_style, dark_style, story);
                meta(c, dark_style, story);
            },
        );
    });
}

fn meta(c: &mut ChildBuilder, dark_style: &TextStyle, story: &HackerNewsStory) {
    text(
        c,
        Some(Style {
            align_items: AlignItems::Center,
            ..default()
        }),
        dark_style,
        &format!(
            "{} points by {} | {} comments",
            num_as_f32(&story.points),
            story.user.as_ref().unwrap_or(&String::from("undefined")),
            story.comments_count.as_f64().unwrap_or(0.0),
        ),
    );
}

fn title(
    c: &mut ChildBuilder,
    title_style: &TextStyle,
    dark_style: &TextStyle,
    story: &HackerNewsStory,
) {
    container(
        c,
        None,
        Some(Style {
            align_items: AlignItems::Center,
            ..default()
        }),
        |c| {
            text_sections(
                c,
                None,
                [
                    (title_style.clone(), story.title.to_string()),
                    (
                        dark_style.clone(),
                        format!(" ({})", story.domain.as_ref().unwrap_or(&String::from(""))),
                    ),
                ],
            );
        },
    );
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
