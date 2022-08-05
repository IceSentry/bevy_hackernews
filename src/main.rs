#![allow(clippy::type_complexity)]

use api::{get_stories, HackerNewsStory, HackerNewsStoryWithComments};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::Instant,
};
use futures_lite::future;

use theme::*;
use ui_components::{
    comment::comment,
    header::header,
    primitives::{container, text},
    scrolling_list::scrolling_list,
    story::story,
    UiComponentsPlugin,
};

mod api;
mod theme;
mod ui_components;
mod utils;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct StoriesRootNode;

struct Stories {
    data: Vec<HackerNewsStory>,
}

struct SelectedStory(Option<HackerNewsStoryWithComments>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiComponentsPlugin)
        .insert_resource(Stories { data: vec![] })
        .insert_resource(SelectedStory(None))
        .add_startup_system(setup)
        .add_startup_system(get_stories_async)
        .add_system(handle_get_stories)
        .add_system(handle_stories_changed)
        .add_system(handle_comments)
        .run();
}

#[allow(clippy::redundant_clone)]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: BG_NEUTRAL_400.into(),
            ..default()
        })
        .insert(UiRoot)
        .with_children(|c| {
            nav(c, asset_server);
            //stories root
            scrolling_list(c, StoriesRootNode);
        });
}

fn nav(c: &mut ChildBuilder, asset_server: Res<AssetServer>) {
    container(
        c,
        Some(BG_ORANGE_600),
        Some(Style {
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }),
        |c| {
            container(c, None, None, |c| {
                let text_style = TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                };

                header(c, &text_style, "news", "Hacker News");
                header(c, &text_style, "newest", "Newest");
                header(c, &text_style, "show", "Show");
                header(c, &text_style, "ask", "Ask");
                header(c, &text_style, "jobs", "Jobs");
            });

            text(
                c,
                Some(Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        ..default()
                    },
                    size: Size::new(Val::Auto, Val::Px(50.)),
                    ..default()
                }),
                &TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                "made with Bevy",
            );
        },
    );
}

#[derive(Component)]
struct GetStoriesTask(Task<anyhow::Result<Vec<HackerNewsStory>>>);

fn get_stories_async(mut commands: Commands) {
    spawn_get_stories_async(&mut commands, "news".to_string());
}

fn spawn_get_stories_async(commands: &mut Commands, r#type: String) {
    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move {
        let start = Instant::now();
        let stories = get_stories(&r#type);
        info!(
            "getting stories took {}ms",
            (Instant::now() - start).as_millis()
        );
        stories
    });
    commands.spawn().insert(GetStoriesTask(task));
}

fn handle_get_stories(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut GetStoriesTask)>,
    mut stories: ResMut<Stories>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(stories_response) = future::block_on(future::poll_once(&mut task.0)) {
            stories.data = stories_response.expect("failed to get stories");
            commands.entity(entity).despawn();
        }
    }
}

fn handle_stories_changed(
    mut commands: Commands,
    stories: Res<Stories>,
    stories_root: Query<Entity, With<StoriesRootNode>>,
    asset_server: Res<AssetServer>,
) {
    if !stories.is_changed() {
        return;
    }

    // we need to despawn all children because we are about to change all of them
    commands.entity(stories_root.single()).despawn_descendants();

    commands.entity(stories_root.single()).with_children(|c| {
        let title_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        };

        let dark_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: TEXT_NEUTRAL_400,
        };

        for (i, hn_story) in stories.data.iter().enumerate() {
            story(c, &title_style, &dark_style, i, hn_story);
        }
    });
}

fn handle_comments(
    mut commands: Commands,
    selected_story: Res<SelectedStory>,
    stories_root: Query<Entity, With<StoriesRootNode>>,
    asset_server: Res<AssetServer>,
) {
    if !selected_story.is_changed() {
        return;
    }

    if let Some(story) = &selected_story.0 {
        commands.entity(stories_root.single()).despawn_descendants();
        commands.entity(stories_root.single()).with_children(|c| {
            let text_style = TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            };
            let meta_style = TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 14.0,
                color: Color::BLACK,
            };

            for hn_comment in &story.comments {
                comment(c, &text_style, &meta_style, hn_comment);
            }
        });
    }
}
