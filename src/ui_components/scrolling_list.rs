//! A scrolling list component. This is take verbatim from bevy's ui example.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::WindowResized,
};

use super::primitives::div_with_style;

pub struct ScrollingListPlugin;
impl Plugin for ScrollingListPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(on_mouse_scroll)
                .with_system(on_children_update)
                .with_system(on_resize),
        )
        .add_system_to_stage(CoreStage::PostUpdate, sync_positions);
    }
}

#[derive(Component)]
struct Indicator;

pub fn scrolling_list(c: &mut ChildBuilder, tag: impl Component) {
    let style = Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        flex_direction: FlexDirection::ColumnReverse,
        overflow: Overflow::Hidden,
        ..default()
    };
    let indicator_style = Style {
        position_type: PositionType::Absolute,
        size: Size::new(Val::Px(10.), Val::Px(50.)),
        position: UiRect {
            top: Val::Px(0.),
            right: Val::Px(0.),
            ..default()
        },
        ..default()
    };
    div_with_style(c, &style, |c| {
        let indicator_entity = c
            .spawn_bundle(NodeBundle {
                style: indicator_style,
                ..default()
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
            indicator_height: 0.0,
        })
        .insert(tag);
    });
}

#[derive(Component, Debug)]
pub struct ScrollingList {
    position: f32,
    indicator_position: f32,
    indicator_height: f32,
    indicator_entity: Entity,
}

fn on_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        let dy = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
            MouseScrollUnit::Pixel => mouse_wheel_event.y,
        };

        for (mut list, children, uinode) in &mut query_list {
            let items_height = compute_children_height(children, &query_item);
            let (indicator_height, remaining_space, max_scroll, jump) =
                compute_values(items_height, uinode.size.y);

            list.position += dy * jump;
            list.position = list.position.clamp(-max_scroll, 0.);

            list.indicator_position -= dy;
            list.indicator_position = list.indicator_position.clamp(0., remaining_space);
            list.indicator_height = indicator_height;
        }
    }
}

fn on_children_update(
    mut query_list: Query<
        (&mut ScrollingList, &Children, &Node),
        (Without<Indicator>, Or<(Changed<Children>, Changed<Node>)>),
    >,
    query_item: Query<&Node>,
) {
    for (mut list, children, uinode) in &mut query_list {
        info!("children changed");
        let items_height = compute_children_height(children, &query_item);
        let (indicator_height, _, _, _) = compute_values(items_height, uinode.size.y);
        if items_height < uinode.size.y {
            list.position = 0.;
            list.indicator_position = 0.;
        }

        list.indicator_height = indicator_height;
    }
}

fn on_resize(
    mut query_list: Query<(&mut ScrollingList, &Children, &Node)>,
    query_item: Query<&Node>,
    resize_events: EventReader<WindowResized>,
) {
    if !resize_events.is_empty() {
        for (mut list, children, uinode) in &mut query_list {
            let items_height = compute_children_height(children, &query_item);
            let (indicator_height, _, _, _) = compute_values(items_height, uinode.size.y);

            // It seems like the children only update their size 1 frrame aftre the resize
            // So this doesn't do anything
            list.indicator_height = indicator_height;
        }
    }
}

fn sync_positions(
    mut query_list: Query<
        (&ScrollingList, &mut Style),
        (Without<Indicator>, Changed<ScrollingList>),
    >,
    mut query_indicator: Query<&mut Style, With<Indicator>>,
) {
    for (list, mut style) in &mut query_list {
        style.position.top = Val::Px(list.position);
        if let Ok(mut indicator_style) = query_indicator.get_mut(list.indicator_entity) {
            indicator_style.position.top = Val::Px(list.indicator_position);
            indicator_style.size.height = Val::Px(list.indicator_height);
        }
    }
}

fn compute_children_height(children: &Children, query_item: &Query<&Node>) -> f32 {
    children
        .iter()
        .map(|entity| query_item.get(*entity).map(|x| x.size.y).unwrap_or(0.0))
        .sum()
}

fn compute_values(items_height: f32, panel_height: f32) -> (f32, f32, f32, f32) {
    let ratio = panel_height / items_height;
    let indicator_height = panel_height * ratio;
    let remaining_space = panel_height - indicator_height;
    let max_scroll = (items_height - panel_height).max(0.);
    let mut jump = max_scroll / remaining_space;
    if jump.is_nan() {
        jump = 0.;
    }
    (indicator_height, remaining_space, max_scroll, jump)
}
