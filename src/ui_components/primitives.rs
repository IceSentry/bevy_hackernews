use bevy::{prelude::*, ui::FocusPolicy};

pub fn div(c: &mut ChildBuilder, f: impl FnOnce(&mut ChildBuilder)) -> Entity {
    div_with_style(c, &Style::default(), f)
}

pub fn div_with_style(
    c: &mut ChildBuilder,
    style: &Style,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    div_color_with_style(c, Color::NONE, style, f)
}

pub fn div_color_with_style(
    c: &mut ChildBuilder,
    color: Color,
    style: &Style,
    f: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    c.spawn_bundle(NodeBundle {
        color: color.into(),
        style: style.clone(),
        focus_policy: FocusPolicy::Pass,
        ..Default::default()
    })
    .with_children(f)
    .id()
}

pub fn button(c: &mut ChildBuilder, style: &Style, f: impl FnOnce(&mut ChildBuilder)) {
    c.spawn_bundle(ButtonBundle {
        color: Color::NONE.into(),
        style: style.clone(),
        ..Default::default()
    })
    .with_children(f);
}

pub fn button_with_component(
    c: &mut ChildBuilder,
    style: &Style,
    component: impl Component,
    f: impl FnOnce(&mut ChildBuilder),
) {
    c.spawn_bundle(ButtonBundle {
        color: Color::NONE.into(),
        style: style.clone(),
        ..Default::default()
    })
    .insert(component)
    .with_children(f);
}

pub fn button_color(
    c: &mut ChildBuilder,
    style: &Style,
    color: Color,
    f: impl FnOnce(&mut ChildBuilder),
) {
    c.spawn_bundle(ButtonBundle {
        color: color.into(),
        style: style.clone(),
        ..Default::default()
    })
    .with_children(f);
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
