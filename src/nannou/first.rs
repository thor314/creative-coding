//! To make a Nannou,
//! - create the app model
//! - provide the update model
//! - draw the window
//! and go!
use nannou::prelude::*;

pub fn first() {
  nannou::app(model)
    // .event() // the event function runs every time some app event occurs, like a mouse movement.
    .update(update) // the update event runs at 60 Hz
    .simple_window(view) 
    .run();
}

pub fn sketch() {
  // sketches are pared down static creations, simpler than the full MVC model, only requiring a
  // view. nannou::sketch(sketch_view).run()
  nannou::sketch(ellipse_view).run()
}

/// The Model in MVC describes the internal state.
/// Where application state is defined, including user input and timed updates.
struct Model;

fn model(_app: &App) -> Model { Model {} }

/// the Controller in MVC describes how to update the model
fn update(_app: &App, _model: &mut Model, _update: Update) {}

/// the View in MVC describes how to present the model
fn view(_app: &App, _model: &Model, frame: Frame) {
  frame.clear(PURPLE);
}

fn sketch_view(app: &App, frame: Frame) {
  // get canvas to draw on
  let canvas = app.draw();
  let background = canvas.background().color(LIGHTSLATEGRAY);
  canvas.to_frame(app, &frame).unwrap();
}

fn ellipse_view(app: &App, frame: Frame) {
  let draw = app.draw();
  draw.background().color(PLUM);
  draw.ellipse().color(STEELBLUE);
  draw.to_frame(app, &frame).unwrap();
}
