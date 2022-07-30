#![allow(unused)]

use bevy::{prelude::*, ui::FocusPolicy};

use self::{
    header::HeaderPlugin, scrolling_list::ScrollingListPlugin, story::StoryComponentPlugin,
};

pub mod comment;
pub mod header;
pub mod primitives;
pub mod scrolling_list;
pub mod story;

pub struct UiComponentsPlugin;
impl Plugin for UiComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HeaderPlugin)
            .add_plugin(ScrollingListPlugin)
            .add_plugin(StoryComponentPlugin);
    }
}
