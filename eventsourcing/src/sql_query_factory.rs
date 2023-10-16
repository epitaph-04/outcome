pub(crate) struct SqlQueryFactory {
    event_table: String,
    select_events: String,
    insert_event: String,
    all_events: String,
}

impl SqlQueryFactory {
    pub fn new(event_table: &str) -> Self {
        Self {
            event_table: event_table.to_string(),
            select_events: format!("
SELECT aggregate_type, aggregate_id, sequence, event_type, event_version, payload, metadata
  FROM {}
  WHERE aggregate_type = $1 AND aggregate_id = $2
  ORDER BY sequence", event_table),
            insert_event: format!("
INSERT INTO {} (aggregate_type, aggregate_id, sequence, event_type, event_version, payload, metadata)
VALUES ($1, $2, $3, $4, $5, $6, $7)", event_table),
            all_events: format!("
SELECT aggregate_type, aggregate_id, sequence, event_type, event_version, payload, metadata
  FROM {}
  WHERE aggregate_type = $1
  ORDER BY sequence", event_table)
        }
    }
    pub fn select_events(&self) -> &str {
        &self.select_events
    }
    pub fn insert_event(&self) -> &str {
        &self.insert_event
    }
    pub fn all_events(&self) -> &str {
        &self.all_events
    }
    pub fn get_last_events(&self, last_sequence: usize) -> String {
        format!(
            "
SELECT aggregate_type, aggregate_id, sequence, event_type, event_version, payload, metadata
  FROM {}
  WHERE aggregate_type = $1 AND aggregate_id = $2 AND sequence > {}
  ORDER BY sequence",
            &self.event_table, last_sequence
        )
    }
}
