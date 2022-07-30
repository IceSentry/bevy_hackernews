use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use super::div_with_style;

pub struct ScrollingListPlugin;

impl Plugin for ScrollingListPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_mouse_scroll);
    }
}

pub fn scrolling_list(c: &mut ChildBuilder, tag: impl Component) {
    let style = Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        flex_direction: FlexDirection::ColumnReverse,
        overflow: Overflow::Hidden,
        ..Default::default()
    };
    div_with_style(c, &style, |c| {
        c.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(ScrollingList::default())
        .insert(tag);
    });
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

fn on_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
