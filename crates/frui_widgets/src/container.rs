use druid_shell::kurbo::Rect;

use frui::prelude::*;

#[derive(SingleChildWidget)]
pub struct Container<W: Widget> {
    child: W,
    width: Option<f64>,
    height: Option<f64>,
    color: Option<Color>,
}

impl Container<()> {
    pub fn builder() -> Container<()> {
        Container {
            child: (),
            width: None,
            height: None,
            color: None,
        }
    }
}

impl<W: Widget> Container<W> {
    pub fn child<C: Widget>(self, child: C) -> Container<C> {
        Container {
            child,
            width: self.width,
            height: self.height,
            color: self.color,
        }
    }

    #[track_caller]
    pub fn width(mut self, width: f64) -> Self {
        assert!(width >= 0.0, "width must be >= 0.0");
        self.width = Some(width);
        self
    }

    #[track_caller]
    pub fn height(mut self, height: f64) -> Self {
        assert!(height >= 0.0, "height must be >= 0.0");
        self.height = Some(height);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<W: Widget> SingleChildWidget for Container<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let size = ctx.child().layout(Constraints {
            max_width: self.width.unwrap_or(constraints.max_width),
            max_height: self.height.unwrap_or(constraints.max_height),
            ..constraints
        });

        Size {
            width: self.width.unwrap_or(size.width),
            height: self.height.unwrap_or(size.height),
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        if let Some(color) = &self.color {
            let brush = &canvas.solid_brush(color.clone());

            PietRenderContext::fill(canvas, Rect::from_origin_size(offset, ctx.size()), brush);
        }

        ctx.child().paint(canvas, offset)
    }
}
