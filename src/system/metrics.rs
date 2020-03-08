/// This struct reports statistics about clickhouse 
/// internals and are pulled from system.metrics table.
/// 
/// These metrics are instant values.
/// In prometheus term, they are called Gauge (https://prometheus.io/docs/concepts/metric_types/#gauge)

#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct Metrics {
    /// Number of executing queries
    pub query: Count,

    /// Number of executing background merges
    pub merge: Count,

    /// Number of mutations (ALTER DELETE/UPDATE)
    pub part_mutation: Count,

    /// Number of data parts being fetched from replica
    pub replicated_fetch: Count,

    /// Number of data parts being sent to replicas
    pub replicated_send: Count,

    /// Number of data parts checking for consistency
    pub replicated_checks: Count,

    /// Number of active tasks in BackgroundProcessingPool (merges, mutations, fetches, or replication queue bookkeeping
    pub background_pool_task: Count,

    /// Number of active tasks in BackgroundProcessingPool for moves
    pub background_move_pool_task: Count,

    /// Number of active tasks in BackgroundSchedulePool
    /// This pool is used for periodic ReplicatedMergeTree tasks, like cleaning old data parts, altering data parts, replica re-initialization, etc
    pub background_schedule_pool_task: Count,

    /// Number of 'batches' (a set of keys) in update queue in CacheDictionaries
    pub cache_dictionary_update_queue_batches: Count,

    /// Exact number of keys in update queue in CacheDictionaries
    pub cache_dictionary_update_queue_keys: Count,

    /// Disk space reserved for currently running background merges
    /// It is slightly more than the total size of currently merging parts
    pub disk_space_reserved_for_merge: Count,

    /// Number of connections to remote servers sending data that was INSERTed into Distributed tables
    /// Both synchronous and asynchronous mode
    pub distributed_send: Count,

    /// Number of queries that are stopped and waiting due to 'priority' setting
    pub query_preempted: Count,

    /// Number of connections to TCP server (clients with native interface), also included server-server distributed query connections")
    pub tcp_connection: Count,
}