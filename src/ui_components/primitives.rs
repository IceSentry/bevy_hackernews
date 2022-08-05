//! Primitive components are used to build more complex components.
//!
//! The basic primitives are:
//! - `div` -> NodeBundle
//! - `text` -> TextBundle
//! - `button` -> ButtonBundle

use bevy::{prelude::*, ui::FocusPolicy};

pub fn container(
    c: &mut ChildBuilder,
    color: Option<Color>,
    style: Option<Style>,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    c.spawn_bundle(NodeBundle {
        color: color.unwrap_or(Color::NONE).into(),
        style: style.unwrap_or_default(),
        focus_policy: FocusPolicy::Pass,
        ..Default::default()
    })
    .with_children(f)
    .id()
}

pub fn container_with_tag(
    c: &mut ChildBuilder,
    color: Option<Color>,
    style: Option<Style>,
    component: impl Component,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    c.spawn_bundle(NodeBundle {
        color: color.unwrap_or(Color::NONE).into(),
        style: style.unwrap_or_default(),
        focus_policy: FocusPolicy::Pass,
        ..Default::default()
    })
    .insert(component)
    .with_children(f)
    .id()
}

pub fn button(c: &mut ChildBuilder, style: &Style, f: impl FnOnce(&mut ChildBuilder)) -> Entity {
    button_impl(c, Some(style.clone()), None, EmptyComponent, f)
}

pub fn button_with_tag(
    c: &mut ChildBuilder,
    style: &Style,
    tag: impl Component,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    button_impl(c, Some(style.clone()), None, tag, f)
}

pub fn button_color(
    c: &mut ChildBuilder,
    style: &Style,
    color: Color,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    button_impl(c, Some(style.clone()), Some(color), EmptyComponent, f)
}

#[derive(Component)]
struct EmptyComponent;

pub fn button_impl(
    c: &mut ChildBuilder,
    style: Option<Style>,
    color: Option<Color>,
    tag: impl Component,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    c.spawn_bundle(ButtonBundle {
        color: color.unwrap_or(Color::NONE).into(),
        style: style.unwrap_or_default(),
        ..Default::default()
    })
    .insert(tag)
    .with_children(f)
    .id()
}

pub fn text(
    c: &mut ChildBuilder,
    style: Option<Style>,
    text_style: &TextStyle,
    text: impl Into<String>,
) -> Entity {
    container(c, None, style, |c| {
        c.spawn_bundle(TextBundle::from_section(text, text_style.clone()));
    })
}

pub fn text_sections(
    c: &mut ChildBuilder,
    style: Option<Style>,
    sections: impl IntoIterator<Item = (TextStyle, String)>,
) {
    c.spawn_bundle(TextBundle {
        text: Text {
            sections: sections
                .into_iter()
                .map(|(style, value)| TextSection::new(value, style))
                .collect(),
            ..default()
        },
        style: style.unwrap_or_default(),
        ..default()
    });
}

pub fn text_sections_with_style(
    c: &mut ChildBuilder,
    style: &Style,
    sections: impl IntoIterator<Item = (TextStyle, String)>,
) {
    c.spawn_bundle(
        TextBundle::from_sections(
            sections
                .into_iter()
                .map(|(style, value)| TextSection::new(value, style)),
        )
        .with_style(style.clone()),
    );
}
