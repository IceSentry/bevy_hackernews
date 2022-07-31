//! A scrolling list component. This is take verbatim from bevy's ui example.

use std::ops::Add;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use super::primitives::div_with_style;

pub struct ScrollingListPlugin;
impl Plugin for ScrollingListPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_mouse_scroll);
    }
}

#[derive(Component)]
struct Indicator;

pub fn scrolling_list(c: &mut ChildBuilder, tag: impl Component) {
    let style = Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        flex_direction: FlexDirection::ColumnReverse,
        overflow: Overflow::Hidden,
        ..Default::default()
    };
    div_with_style(c, &style, |c| {
        let indicator_entity = c
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(10.), Val::Px(50.)),
                    position: UiRect {
                        top: Val::Px(0.),
                        right: Val::Px(0.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Indicator)
            .id();

        c.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(ScrollingList {
            indicator_entity,
            position: 0.0,
            indicator_position: 0.0,
        })
        .insert(tag);
    });
}

#[derive(Component)]
pub struct ScrollingList {
    position: f32,
    indicator_position: f32,
    indicator_entity: Entity,
}

fn on_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node), Without<Indicator>>,
    query_item: Query<&Node>,
    mut query_indicator: Query<(&mut Style), With<Indicator>>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();

            let panel_height = uinode.size.y;
            let ratio = (panel_height / items_height);
            let indicator_height = panel_height * ratio;
            let remaining_space = panel_height - indicator_height;
            let max_scroll = (items_height - panel_height).max(0.);

            let jump = max_scroll / remaining_space;

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy * jump;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);

            let (mut indicator_style) = query_indicator
                .get_mut(scrolling_list.indicator_entity)
                .expect("Scroll indicator should exist");

            scrolling_list.indicator_position -= dy;
            scrolling_list.indicator_position =
                scrolling_list.indicator_position.clamp(0., remaining_space);

            indicator_style.position.top = Val::Px(scrolling_list.indicator_position);
            indicator_style.size.height = Val::Px(indicator_height);
            info!(
                "pos: {:?} {remaining_space}",
                scrolling_list.indicator_position
            );
        }
    }
}
