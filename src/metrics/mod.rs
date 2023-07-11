use {
    crate::error::{Error, Result},
    opentelemetry::{
        metrics::Counter,
        sdk::{
            self,
            export::metrics::aggregation,
            metrics::{processors, selectors},
            Resource,
        },
    },
    opentelemetry_prometheus::PrometheusExporter,
    prometheus_core::TextEncoder,
};

#[derive(Clone)]
pub struct Metrics {
    pub prometheus_exporter: PrometheusExporter,

    pub received_items: Counter<u64>,
    pub stored_items: Counter<u64>,

    pub get_queries: Counter<u64>,
    pub served_items: Counter<u64>,

    pub register: Counter<u64>,
    pub registration_overwrite: Counter<u64>,
    pub registration_update: Counter<u64>,
    pub cached_registrations: Counter<u64>,
    pub fetched_registrations: Counter<u64>,
    pub registration_cache_invalidation: Counter<u64>,
}

impl Metrics {
    pub fn new(resource: Resource) -> Result<Self> {
        let controller = sdk::metrics::controllers::basic(
            processors::factory(
                selectors::simple::histogram(vec![]),
                aggregation::cumulative_temporality_selector(),
            )
            .with_memory(true),
        )
        .with_resource(resource)
        .build();

        let prometheus_exporter = opentelemetry_prometheus::exporter(controller).init();

        let meter = prometheus_exporter.meter_provider().unwrap();

        opentelemetry::global::set_meter_provider(meter);

        let meter = opentelemetry::global::meter("history-server");

        let received_items = meter
            .u64_counter("received_items")
            .with_description("The number of items received from relay")
            .init();

        let stored_items = meter
            .u64_counter("stored_items")
            .with_description("The number of items actually stored")
            .init();

        let get_queries = meter
            .u64_counter("get_queries")
            .with_description("The number of items retrieval queries")
            .init();

        let served_items = meter
            .u64_counter("served_items")
            .with_description("The number of messages served to clients")
            .init();

        let register = meter
            .u64_counter("register")
            .with_description("The total number of calls to the register method")
            .init();

        let registration_overwrite = meter
            .u64_counter("register")
            .with_description("The number of calls to the register method in overwrite mode")
            .init();

        let registration_update = meter
            .u64_counter("register")
            .with_description("The number of calls to the register method in update mode")
            .init();

        let cached_registrations = meter
            .u64_counter("cached_registrations")
            .with_description("The number of registrations retrieved from the in-memory cache")
            .init();

        let fetched_registrations = meter
            .u64_counter("fetched_registrations")
            .with_description("The number of registrations retrieved from the database")
            .init();

        let registration_cache_invalidation = meter
            .u64_counter("registration_cache_invalidation")
            .with_description("The number of registrations cache invalidations")
            .init();

        Ok(Metrics {
            prometheus_exporter,
            received_items,
            stored_items,
            get_queries,
            served_items,
            register,
            registration_overwrite,
            registration_update,
            cached_registrations,
            fetched_registrations,
            registration_cache_invalidation,
        })
    }

    pub fn export(&self) -> Result<String> {
        let data = self.prometheus_exporter.registry().gather();
        TextEncoder::new()
            .encode_to_string(&data)
            .map_err(Error::Prometheus)
    }
}
