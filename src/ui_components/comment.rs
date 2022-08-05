use bevy::prelude::*;

use super::primitives::{container, text};
use crate::api::HackerNewsComment;

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
    container(c, None, Some(style), |c| {
        meta(c, meta_style, hn_comment);

        content(c, text_style, hn_comment);

        // sub comments
        container(
            c,
            None,
            Some(Style {
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            }),
            |c| {
                for sub_comment in &hn_comment.comments {
                    comment(c, text_style, meta_style, sub_comment);
                }
            },
        );
    });
}

fn meta(c: &mut ChildBuilder, meta_style: &TextStyle, hn_comment: &HackerNewsComment) {
    container(
        c,
        None,
        Some(Style {
            // size: Size::new(Val::Undefined, Val::Px(24.)),
            // flex_shrink: 0.,
            align_items: AlignItems::FlexEnd,
            margin: UiRect {
                top: Val::Px(5.),
                bottom: Val::Px(5.),
                ..default()
            },
            ..default()
        }),
        |c| {
            text(
                c,
                None,
                meta_style,
                format!(
                    "{} {}",
                    hn_comment.user.as_ref().unwrap_or(&String::from("")),
                    hn_comment.time_ago
                ),
            );
        },
    );
}

fn content(c: &mut ChildBuilder, text_style: &TextStyle, hn_comment: &HackerNewsComment) {
    container(c, None, None, |c| {
        let escaped_content = hn_comment
            .content
            .as_ref()
            .map(|c| {
                c.replace("<p>", "\n\n")
                    .replace("</p>", "")
                    .replace("<i>", "")
                    .replace("</i>", "")
                    .replace("&quot;", "\"")
            })
            .unwrap_or_else(|| String::from(""));
        text(
            c,
            Some(Style {
                max_size: Size {
                    width: Val::Px(800.),
                    height: Val::Auto,
                },
                ..default()
            }),
            text_style,
            escaped_content,
        );
    });
}
