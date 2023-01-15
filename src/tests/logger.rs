use tracing::{field::Visit, span, Subscriber};
use tracing_subscriber::{registry::LookupSpan, Layer};

pub struct CustomLayer {
    indent_width: usize,
}

impl CustomLayer {
    pub fn new(indent_width: usize) -> CustomLayer {
        CustomLayer { indent_width }
    }
}

impl<S> Layer<S> for CustomLayer
where
    S: Subscriber,
    for<'lookup> S: LookupSpan<'lookup>,
{
    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // get span name
        let Some(span ) = ctx.span(id) else { return };
        let span_name = span.metadata().name();

        // get parent indent
        let (parent_indent, default_delta) = match ctx.lookup_current() {
            Some(parent_span) => (*parent_span.extensions().get().unwrap_or(&0), 0),
            None => (0, 0),
        };

        // get delta indent
        let mut visitor = CustomVisitor {
            msg: String::new(),
            indent: default_delta,
        };
        attrs.record(&mut visitor);
        let delta_indent = visitor.indent;
        let msg = visitor.msg;

        // calculate current indent (= span + delta)
        let current_indent = parent_indent + delta_indent;

        // print span name if any
        if !span_name.is_empty() {
            println!(
                "{}{}:",
                " ".repeat(current_indent * self.indent_width),
                span_name
            );
        }

        // print message if any
        if !msg.is_empty() {
            println!(
                "{}{}",
                " ".repeat((current_indent + 1) * self.indent_width),
                msg
            );
        }

        // save base indent
        let base_indent = current_indent + 1;
        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        extensions.insert(base_indent);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // load base indent
        let span = ctx.lookup_current().unwrap();
        let extensions = span.extensions();
        let base_indent: &usize = extensions.get().unwrap_or(&0);

        // get delta indent
        let mut visitor = CustomVisitor {
            msg: String::new(),
            indent: 0, // default delta indent per event is zero
        };
        event.record(&mut visitor);
        let delta_indent = visitor.indent;
        let log = visitor.msg;

        // calculate indent (= span + delta)
        let indent = base_indent + delta_indent;

        // print log
        println!("{}{}", " ".repeat(indent * self.indent_width), log);
    }
}

struct CustomVisitor {
    msg: String,
    indent: usize,
}

impl Visit for CustomVisitor {
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if field.name() == "indent" {
            self.indent = value as usize;
        }
    }
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.msg = value.to_string();
        }
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            let msg = format!("{:?}", value);
            self.msg = if msg.starts_with('"') {
                msg.trim_matches('"').to_string() // remove surrounding quotes
            } else {
                msg
            };
        }
    }
}
