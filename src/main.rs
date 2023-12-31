use std::{collections::HashMap, env, time::Duration};

use opentelemetry::{
    global,
    sdk::trace::{self, IdGenerator},
    trace::{Span, SpanId, Status, TraceId, Tracer},
    Key,
};
use opentelemetry_otlp::WithExportConfig;
use rand::{distributions::Alphanumeric, prelude::Distribution, thread_rng};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    init_tracer();

    loop {
        do_work();
        sleep(Duration::from_secs(5)).await;
    }
}

#[derive(Debug)]
struct Idgen;

impl IdGenerator for Idgen {
    fn new_trace_id(&self) -> opentelemetry::trace::TraceId {
        let mut rng = thread_rng();
        let mut id = [0u8; 16];
        id.fill_with(|| Alphanumeric.sample(&mut rng));
        TraceId::from_bytes(id)
    }

    fn new_span_id(&self) -> opentelemetry::trace::SpanId {
        let mut rng = thread_rng();
        let mut id = [0u8; 8];
        id.fill_with(|| Alphanumeric.sample(&mut rng));
        SpanId::from_bytes(id)
    }
}

fn init_tracer() {
    let endpoint =
        env::var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT").expect("Requires exporter endpoint");

    // with tonic you can use http://0.0.0.0:4318/
    // with http you can use http://0.0.0.0:4318/v1/traces

    // First, create a OTLP exporter builder. Configure it as you need.
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(endpoint)
        .with_headers(HashMap::from([
            (
                "Authorization".to_owned(),
                "Basic YWRtaW46YWRtaW4=".to_owned(),
            ),
            ("X-P-Stream".to_owned(), "oteltrace".to_owned()),
        ]));

    // Then pass it into pipeline
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(trace::Config::default().with_id_generator(Idgen))
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();
}

const LEMONS_KEY: Key = Key::from_static_str("ex.com/lemons");
const ANOTHER_KEY: Key = Key::from_static_str("ex.com/another");

// Example function to be instrumented
fn do_work() {
    let tracer = global::tracer("ex.com/basic");
    let mut span = tracer.start("operation");
    span.add_event("Event!".to_string(), vec![Key::new("value").i64(100)]);
    span.set_attribute(ANOTHER_KEY.string("yes"));
    span.set_attribute(LEMONS_KEY.string("five"));
    span.set_status(Status::Ok);

    // perform some operation

    span.end()
}
