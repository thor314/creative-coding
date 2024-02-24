//! x,y are at 0,0. left to right is 600, top to bottom is 400.
use nannou::prelude::*;

pub fn draw() {
  nannou::app(model).update(update).run()
}

struct Model {
  _window: window::Id,
}

fn model (app: &App) -> Model {
  let _window = app.new_window().view(view).build().unwrap();
  Model {_window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}