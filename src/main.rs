/// This example shows how to create a widget that animates a beating heart shape.
use druid::{
    kurbo::{BezPath, Point},
    piet::{Color, RenderContext},
    AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WindowDesc,
};

#[derive(Clone, Data)]
struct AppState {
    time: f64,
}

struct HeartWidget;

impl Widget<AppState> for HeartWidget {
    /// Handles events for the HeartWidget.
    ///
    /// In particular, it processes animation frame events to update the
    /// animation time and request the next animation frame and repaint.
    ///
    /// # Arguments
    /// 
    /// * `ctx` - The event context used to request animation frames and painting.
    /// * `event` - The event being handled. Only `AnimFrame` events are processed.
    /// * `data` - The application state, which holds the current animation time.
    /// * `_env` - The environment, which is currently unused.
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        // Check if the event is an animation frame event
        if let Event::AnimFrame(_) = event {
            // Increment the animation time
            data.time += 0.016;

            // Request the next animation frame
            ctx.request_anim_frame();

            // Request a repaint to update the display
            ctx.request_paint();
        }
    }

    /// Handles life cycle events for the HeartWidget.
    ///
    /// In particular, it handles the `WidgetAdded` event by requesting
    /// an animation frame to start the animation loop.
    ///
    /// # Arguments
    /// 
    /// * `ctx` - The lifecycle context used to request animation frames.
    /// * `event` - The lifecycle event being handled.
    /// * `data` - The application state, which is currently unused.
    /// * `_env` - The environment, which is currently unused.
    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            // Start the animation loop
            ctx.request_anim_frame();
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {}

    /// Computes the preferred size of the HeartWidget.
    ///
    /// The preferred size is the maximum size allowed by the `BoxConstraints`
    /// passed in, which should be the size of the window. This makes the widget
    /// fill the window.
    ///
    /// # Arguments
    ///
    /// * `_ctx` - The layout context, which is currently unused.
    /// * `bc` - The box constraints that specify the maximum size allowed.
    /// * `_data` - The application state, which is currently unused.
    /// * `_env` - The environment, which is currently unused.
    ///
    /// # Returns
    ///
    /// The preferred size of the widget.
    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.max() // Make the widget fill the window
    }

/// Paints a heart shape on the widget with a beating animation effect.
/// 
/// The heart shape is drawn centered within the widget, and its size
/// oscillates over time to simulate a beating effect. The heart is outlined
/// in black and filled with red.
/// 
/// # Arguments
/// 
/// * `ctx` - The painting context used to draw the heart.
/// * `data` - The application state, which provides the current time for the
///            beating animation.
/// * `_env` - The environment, which is currently unused.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let size = ctx.size();
        let center = Point::new(size.width / 2.0, size.height / 2.0);
        let scale = 1.0 + 0.1 * f64::sin(data.time * 3.0); // Heart beating effect

        // Define the heart shape
        let mut path = BezPath::new();
        let width = size.width.min(size.height) * 0.25 * scale;
        let height = size.width.min(size.height) * 0.48 * scale;

        // Start at the bottom tip of the heart
        path.move_to(Point::new(center.x, center.y + height / 2.0));

        // Left half of the heart
        path.curve_to(
            Point::new(center.x - width, center.y + height / 4.0),
            Point::new(center.x - width, center.y - height / 2.0),
            Point::new(center.x, center.y - height / 4.0),
        );

        // Right half of the heart
        path.curve_to(
            Point::new(center.x + width, center.y - height / 2.0),
            Point::new(center.x + width, center.y + height / 4.0),
            Point::new(center.x, center.y + height / 2.0),
        );

        path.close_path();

        ctx.stroke(&path, &Color::rgb8(0, 0, 0), 4.0);

        // Fill the heart with red color
        ctx.fill(&path, &Color::rgb8(255, 0, 0));
    }
}

fn main() {
    let main_window = WindowDesc::new(HeartWidget)
        .window_size((400.0, 400.0))
        .title("Beating Heart");

    let initial_state = AppState { time: 0.0 };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
