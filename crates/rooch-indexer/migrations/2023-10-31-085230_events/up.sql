CREATE TABLE events
(
    event_handle_id    VARCHAR      NOT NULL,
    event_seq          BIGINT      NOT NULL,
    type_tag           VARCHAR      NOT NULL,
    event_data         BLOB         NOT NULL,
    event_index        BIGINT      NOT NULL,

    tx_hash            VARCHAR      NOT NULL,
    tx_order           BIGINT      NOT NULL,
    sender             VARCHAR      NOT NULL,
    created_at         BIGINT      NOT NULL,
    updated_at         BIGINT      NOT NULL,
    -- Constraints
    PRIMARY KEY (event_handle_id, event_seq)
);

CREATE INDEX idx_events_tx_hash ON events (tx_hash);
CREATE INDEX idx_events_tx_order ON events (tx_order);
CREATE INDEX idx_events_sender ON events (sender);
CREATE INDEX idx_events_created_at ON events (created_at);