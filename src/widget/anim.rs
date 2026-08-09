use std::time::Instant;

use iced_core::event::{self, Event};
use iced_core::touch;
use iced_core::widget::{Operation, Tree, tree};
use iced_core::window;
use iced_core::{
    Clipboard, Element, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget, layout,
    mouse, overlay,
};

use crate::anim::spring::{Spring, SpringConfig};

/// An animation preset for micro-interactions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimPreset {
    /// Scale up slightly on hover, press in on click.
    Lift {
        hover_scale: f32,
        press_scale: f32,
        hover_lift: f32,
    },
    /// Glow effect on hover.
    Glow,
    /// Fade in/out.
    Fade,
    /// Custom spring-driven animation.
    Spring {
        config: SpringConfig,
    },
}

impl Default for AnimPreset {
    fn default() -> Self {
        Self::Lift {
            hover_scale: 1.02,
            press_scale: 0.98,
            hover_lift: -2.0,
        }
    }
}

/// Wraps any widget with spring-based micro-interaction animations.
///
/// Apply hover/press animations (scale, lift) using physics springs
/// from `crate::anim::spring`. No changes to the existing widget API.
#[allow(missing_debug_implementations)]
pub struct Animated<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Renderer: iced_core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    preset: AnimPreset,
    on_press: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Animated<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            preset: AnimPreset::default(),
            on_press: None,
        }
    }

    pub fn preset(mut self, preset: AnimPreset) -> Self {
        self.preset = preset;
        self
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }
}

struct AnimState {
    scale: Spring<f32>,
    lift: Spring<f32>,
    is_hovered: bool,
    is_pressed: bool,
}

impl AnimState {
    fn new() -> Self {
        Self {
            scale: Spring::<f32>::new(1.0),
            lift: Spring::<f32>::new(0.0),
            is_hovered: false,
            is_pressed: false,
        }
    }

    fn init_springs(&mut self, preset: &AnimPreset) {
        let cfg = spring_config(preset);
        let now = Instant::now();
        self.scale = Spring::<f32>::new_with_config(1.0, cfg);
        self.scale.last_update = Some(now);
        self.scale.target = 1.0;
        self.lift = Spring::<f32>::new_with_config(0.0, cfg);
        self.lift.last_update = Some(now);
        self.lift.target = 0.0;
        self.is_hovered = false;
        self.is_pressed = false;
    }

    fn update_targets(&mut self, preset: &AnimPreset, hovered: bool, pressed: bool) {
        match preset {
            AnimPreset::Lift { hover_scale, press_scale, hover_lift } => {
                if pressed {
                    self.scale.set_target(*press_scale);
                    self.lift.set_target(0.0);
                } else if hovered {
                    self.scale.set_target(*hover_scale);
                    self.lift.set_target(*hover_lift);
                } else {
                    self.scale.set_target(1.0);
                    self.lift.set_target(0.0);
                }
            }
            AnimPreset::Spring { .. } => {
                if pressed {
                    self.scale.set_target(0.98);
                } else if hovered {
                    self.scale.set_target(1.02);
                } else {
                    self.scale.set_target(1.0);
                }
                if hovered && !pressed {
                    self.lift.set_target(-2.0);
                } else {
                    self.lift.set_target(0.0);
                }
            }
            AnimPreset::Glow | AnimPreset::Fade => {
                self.scale.set_target(1.0);
                self.lift.set_target(0.0);
            }
        }
    }

    fn tick(&mut self, now: Instant) -> bool {
        let scale_moving = self.scale.update(now);
        let lift_moving = self.lift.update(now);
        scale_moving || lift_moving
    }
}

fn spring_config(preset: &AnimPreset) -> SpringConfig {
    match preset {
        AnimPreset::Lift { .. } => SpringConfig::GENTLE,
        AnimPreset::Spring { config } => *config,
        AnimPreset::Glow => SpringConfig::SNAPPY,
        AnimPreset::Fade => SpringConfig::GENTLE,
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Animated<'_, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: 'static,
    Message: Clone + 'static,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<AnimState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(AnimState::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<AnimState>();
        if !state.scale.is_at_rest() || !state.lift.is_at_rest() {
            // still animating, tick in layout to keep redraws going
        }

        let node = self.content.as_widget_mut().layout(
            &mut tree.children[0],
            renderer,
            limits,
        );
        let size = node.size();

        layout::Node::with_children(size, vec![node])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<AnimState>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if !state.is_pressed {
                    state.is_pressed = true;
                    let now = Instant::now();
                    state.scale.last_update = Some(now);
                    state.lift.last_update = Some(now);
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if state.is_pressed {
                    state.is_pressed = false;
                    let now = Instant::now();
                    state.scale.last_update = Some(now);
                    state.lift.last_update = Some(now);

                    if cursor_position.is_over(layout.bounds()) {
                        if let Some(msg) = self.on_press.clone() {
                            shell.publish(msg);
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) => {
                if state.is_hovered {
                    state.is_hovered = false;
                    let now = Instant::now();
                    state.scale.last_update = Some(now);
                    state.lift.last_update = Some(now);
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let was_hovered = state.is_hovered;
                state.is_hovered = cursor_position.is_over(layout.bounds());

                if was_hovered != state.is_hovered {
                    state.scale.last_update = Some(*now);
                    state.lift.last_update = Some(*now);
                }

                state.update_targets(&self.preset, state.is_hovered, state.is_pressed);

                let moving = state.tick(*now);
                if moving {
                    shell.request_redraw();
                }
            }
            _ => {}
        }

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<AnimState>();
        let scale = state.scale.value;
        let lift = state.lift.value;

        if (scale - 1.0).abs() < 0.001 && lift.abs() < 0.001 {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor_position,
                viewport,
            );
            return;
        }

        let bounds = layout.bounds();
        let cx = bounds.x + bounds.width / 2.0;
        let cy = bounds.y + bounds.height / 2.0;

        let transform = iced_core::Transformation::translate(cx, cy)
            * iced_core::Transformation::scale(scale)
            * iced_core::Transformation::translate(-cx, -cy + lift);

        renderer.with_transformation(transform, |renderer| {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor_position,
                viewport,
            );
        });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }

    fn drag_destinations(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        dnd_rectangles: &mut iced_core::clipboard::DndDestinationRectangles,
    ) {
        self.content.as_widget().drag_destinations(
            &state.children[0],
            layout.children().next().unwrap(),
            renderer,
            dnd_rectangles,
        );
    }

    fn id(&self) -> Option<crate::widget::Id> {
        self.content.as_widget().id()
    }

    fn set_id(&mut self, id: crate::widget::Id) {
        self.content.as_widget_mut().set_id(id);
    }

    #[cfg(feature = "a11y")]
    fn a11y_nodes(
        &self,
        layout: Layout<'_>,
        state: &Tree,
        cursor: mouse::Cursor,
    ) -> iced_accessibility::A11yTree {
        self.content.as_widget().a11y_nodes(
            layout.children().next().unwrap(),
            &state.children[0],
            cursor,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Animated<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'static,
    Renderer: iced_core::Renderer + 'a,
    Theme: 'static + 'a,
{
    fn from(animated: Animated<'a, Message, Theme, Renderer>) -> Self {
        Element::new(animated)
    }
}

/// Convenience function to wrap any element with micro-interaction animations.
pub fn animated<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Animated<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    Animated::new(content)
}
