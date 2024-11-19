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
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        if let Event::AnimFrame(_) = event {
            data.time += 0.016; // Increment time for animation
            ctx.request_anim_frame();
            ctx.request_paint();
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            ctx.request_anim_frame(); // Start the animation loop
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {}

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.max() // Make the widget fill the window
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let size = ctx.size();
        let center = Point::new(size.width / 2.0, size.height / 2.0);
        let scale = 1.0 + 0.1 * f64::sin(data.time * 3.0); // Heart beating effect

        // Define the heart shape
        let mut path = BezPath::new();
        let width = size.width.min(size.height) * 0.25 * scale;
        let height = size.width.min(size.height) * 0.45 * scale;

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

        ctx.stroke(&path, &Color::rgb8(255, 0, 0), 4.0);
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
