use bevy::prelude::*;

use super::primitives::{
    button_with_component, div, div_with_style, text_section, text_section_with_style,
};
use crate::{api::HackerNewsComment, spawn_get_stories_async, theme::*, SelectedStory};

pub fn comment(
    c: &mut ChildBuilder,
    text_style: &TextStyle,
    meta_style: &TextStyle,
    hn_comment: &HackerNewsComment,
) {
    let style = Style {
        flex_direction: FlexDirection::ColumnReverse,
        flex_shrink: 0.,
        margin: UiRect {
            left: Val::Px(15.),
            top: Val::Px(5.),
            bottom: Val::Px(5.),
            ..default()
        },
        ..default()
    };
    let content_style = Style {
        max_size: Size {
            width: Val::Px(800.),
            height: Val::Auto,
        },
        ..default()
    };
    div_with_style(c, &style, |c| {
        let style = Style {
            // size: Size::new(Val::Undefined, Val::Px(24.)),
            // flex_shrink: 0.,
            align_items: AlignItems::FlexEnd,
            margin: UiRect {
                top: Val::Px(5.),
                bottom: Val::Px(5.),
                ..default()
            },
            ..default()
        };
        div_with_style(c, &style, |c| {
            text_section(
                c,
                meta_style,
                format!("{} {}", hn_comment.user, hn_comment.time_ago),
            );
        });

        div(c, |c| {
            let parsed_content = hn_comment
                .content
                .replace("<p>", "\n\n")
                .replace("</p>", "")
                .replace("<i>", "")
                .replace("</i>", "")
                .replace("&quot;", "\"");
            text_section_with_style(c, &content_style, text_style, parsed_content);
        });

        div_with_style(
            c,
            &Style {
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            |c| {
                //
                for sub_comment in &hn_comment.comments {
                    comment(c, text_style, meta_style, sub_comment);
                }
            },
        );
    });
}
