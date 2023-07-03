use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLUE);
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(100.0, 100.0)
        .z_degrees(45.0)
        .color(PLUM);

    draw.to_frame(_app, &frame).unwrap();
}
