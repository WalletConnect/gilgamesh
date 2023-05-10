{
  get_queries:    (import 'get_queries.libsonnet').new,
  received_items: (import 'received_items.libsonnet').new,
  registrations:  (import 'registrations.libsonnet').new,
  served_items:   (import 'served_items.libsonnet').new,
  stored_items:   (import 'stored_items.libsonnet').new,
}
