# bevy_hackernews

This is an implementation of the hacker news website made with bevy_ui.

The main goal is to experiment with bevy_ui apis and find areas that could be improved. It will also be nice to see this evolve over time.

## Concepts

The main idea I used is to extract the ui_components in their own modules that exposes a function to create those ui_components. To do that I also needed to create primitive functions like `div` and `text`. I then used those primitives to build more complex components. Each primitives have a few variants like `with_style` or `with_component`.

When extracting those ui_components I also created a `Plugin` for each of those components. This makes it easy to encapsulate behaviour like changing thhe color of a button on hover.

One other thing I did to keep things concise is to use `c` for `ChildBuilder` params. This is used everywhere and makes the code much easier to read.

### UI components variants

- `with_style`: The primitives using this vairant will let you pass a `Style` component to configure the style.
- `with_component`: The primitives using this variant will let you pass a `Component` to the constructor. This is useful when you need to query that node in a separate system.

## TODO

- `style!` macro:
  - The biggest source of verbosity right now is declaring a `Style` because it forces every parameter on a new line *and* it pretty much always has a `..Default::default()` since you almost never change all the values.
- Create macros for the primitives:
  - Since rust doesn't have variadic arguments, I can't have multiple `div()` so I need a bunch of verbose overload. having a `div!` macro would fix the majority of the verbosity.
- Consider a `h!(Component, Option<Props>, Option<Children>)` macro
