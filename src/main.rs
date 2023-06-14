use std::time::Duration;

use opentelemetry::{
    global,
    trace::{TraceContextExt, Tracer},
    Key,
};
use opentelemetry_otlp::WithExportConfig;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    init_tracer();

    loop {
        do_work();
        sleep(Duration::from_secs(10)).await;
    }
}

fn init_tracer() {
    // First, create a OTLP exporter builder. Configure it as you need.
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://0.0.0.0:4317");
    // Then pass it into pipeline

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();
}

const LEMONS_KEY: Key = Key::from_static_str("ex.com/lemons");
const ANOTHER_KEY: Key = Key::from_static_str("ex.com/another");

// Example function to be instrumented
fn do_work() {
    let tracer = global::tracer("ex.com/basic");

    tracer.in_span("operation", |cx| {
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![Key::new("bogons").i64(100)],
        );
        span.set_attribute(ANOTHER_KEY.string("yes"));

        tracer.in_span("Sub operation...", |cx| {
            let span = cx.span();
            span.set_attribute(LEMONS_KEY.string("five"));

            span.add_event("Sub span event", vec![]);
        });
    });
}
