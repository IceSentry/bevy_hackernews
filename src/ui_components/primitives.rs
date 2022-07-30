//! Primitive components are used to build more complex components.
//!
//! The basic primitives are:
//! - `div` -> NodeBundle
//! - `text` -> TextBundle
//! - `button` -> ButtonBundle

use bevy::{prelude::*, ui::FocusPolicy};

pub fn div(c: &mut ChildBuilder, f: impl FnOnce(&mut ChildBuilder)) -> Entity {
    div_impl(c, None, None, f)
}

pub fn div_with_style(
    c: &mut ChildBuilder,
    style: &Style,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    div_impl(c, None, Some(style.clone()), f)
}

pub fn div_color_with_style(
    c: &mut ChildBuilder,
    color: Color,
    style: &Style,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    div_impl(c, Some(color), Some(style.clone()), f)
}

pub fn div_impl(
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

pub fn button(c: &mut ChildBuilder, style: &Style, f: impl FnOnce(&mut ChildBuilder)) -> Entity {
    button_impl(c, Some(style.clone()), None, EmptyComponent, f)
}

pub fn button_with_component(
    c: &mut ChildBuilder,
    style: &Style,
    component: impl Component,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    button_impl(c, Some(style.clone()), None, component, f)
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
    component: impl Component,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    c.spawn_bundle(ButtonBundle {
        color: color.unwrap_or(Color::NONE).into(),
        style: style.unwrap_or_default(),
        ..Default::default()
    })
    .insert(component)
    .with_children(f)
    .id()
}

pub fn text(c: &mut ChildBuilder, style: &Style, text_style: &TextStyle, text: impl Into<String>) {
    div_with_style(c, style, |c| {
        text_section(c, text_style, text);
    });
}

pub fn text_section(c: &mut ChildBuilder, text_style: &TextStyle, text: impl Into<String>) {
    c.spawn_bundle(TextBundle::from_section(text, text_style.clone()));
}

pub fn text_sections(c: &mut ChildBuilder, sections: impl IntoIterator<Item = TextSection>) {
    c.spawn_bundle(TextBundle::from_sections(sections));
}
