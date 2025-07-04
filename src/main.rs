use gpui::{
  App, Application, Bounds, Context, Layer, LayerShellSettings, SharedString, Window, WindowBounds,
  WindowOptions, div, prelude::*, px, rgb, size,
};

struct HelloWorld {
  text: SharedString,
}

impl Render for HelloWorld {
  fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    div()
      .flex()
      .flex_col()
      .gap_3()
      .bg(rgb(0x505050))
      .size(px(500.0))
      .justify_center()
      .items_center()
      .shadow_lg()
      .border_1()
      .border_color(rgb(0x0000ff))
      .text_xl()
      .text_color(rgb(0xffffff))
      .child(format!("Hello, {}!", &self.text))
  }
}

fn main() {
  Application::new().run(|cx: &mut App| {
    let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
    cx.open_window(
      WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        kind: gpui::WindowKind::LayerShell(LayerShellSettings {
          layer: Layer::Top,
          keyboard_interactivity: gpui::KeyboardInteractivity::OnDemand,
          namespace: "gpui-test".into(),
          ..Default::default()
        }),
        ..Default::default()
      },
      |_, cx| {
        cx.new(|_| HelloWorld {
          text: "Dude".into(),
        })
      },
    )
    .unwrap();

    cx.activate(true);
  });
}
