use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Default)]
pub struct OrderMetrics {
    orders_received: AtomicU64,
    orders_matched: AtomicU64,
    orders_cancelled: AtomicU64,
    orders_expired: AtomicU64,
    trades_executed: AtomicU64,
    total_volume: AtomicU64,
    total_value: AtomicU64,
    last_update: AtomicU64,
}

impl OrderMetrics {
    pub fn new() -> Self {
        Self {
            last_update: AtomicU64::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
            ),
            ..Default::default()
        }
    }

    pub fn record_order_received(&self) {
        self.orders_received.fetch_add(1, Ordering::Relaxed);
        self.update_timestamp();
    }

    pub fn record_order_matched(&self) {
        self.orders_matched.fetch_add(1, Ordering::Relaxed);
        self.update_timestamp();
    }

    pub fn record_order_cancelled(&self) {
        self.orders_cancelled.fetch_add(1, Ordering::Relaxed);
        self.update_timestamp();
    }

    pub fn record_order_expired(&self) {
        self.orders_expired.fetch_add(1, Ordering::Relaxed);
        self.update_timestamp();
    }

    pub fn record_trade(&self, quantity: u64, price: u64) {
        self.trades_executed.fetch_add(1, Ordering::Relaxed);
        self.total_volume.fetch_add(quantity, Ordering::Relaxed);
        self.total_value
            .fetch_add(quantity * price, Ordering::Relaxed);
        self.update_timestamp();
    }

    fn update_timestamp(&self) {
        self.last_update.store(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            Ordering::Relaxed,
        );
    }

    pub fn get_metrics(&self) -> OrderMetricsSnapshot {
        OrderMetricsSnapshot {
            orders_received: self.orders_received.load(Ordering::Relaxed),
            orders_matched: self.orders_matched.load(Ordering::Relaxed),
            orders_cancelled: self.orders_cancelled.load(Ordering::Relaxed),
            orders_expired: self.orders_expired.load(Ordering::Relaxed),
            trades_executed: self.trades_executed.load(Ordering::Relaxed),
            total_volume: self.total_volume.load(Ordering::Relaxed),
            total_value: self.total_value.load(Ordering::Relaxed),
            last_update: self.last_update.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderMetricsSnapshot {
    pub orders_received: u64,
    pub orders_matched: u64,
    pub orders_cancelled: u64,
    pub orders_expired: u64,
    pub trades_executed: u64,
    pub total_volume: u64,
    pub total_value: u64,
    pub last_update: u64,
}

#[derive(Default)]
pub struct LatencyMetrics {
    order_processing_time: AtomicU64, 
    order_processing_count: AtomicU64,
    matching_time: AtomicU64, 
    matching_count: AtomicU64,
}

impl LatencyMetrics {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn record_order_processing_time(&self, duration: Duration) {
        self.order_processing_time
            .fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
        self.order_processing_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_matching_time(&self, duration: Duration) {
        self.matching_time
            .fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
        self.matching_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> LatencyMetricsSnapshot {
        let order_processing_count = self.order_processing_count.load(Ordering::Relaxed);
        let matching_count = self.matching_count.load(Ordering::Relaxed);

        LatencyMetricsSnapshot {
            avg_order_processing_time: if order_processing_count > 0 {
                self.order_processing_time.load(Ordering::Relaxed) / order_processing_count
            } else {
                0
            },
            avg_matching_time: if matching_count > 0 {
                self.matching_time.load(Ordering::Relaxed) / matching_count
            } else {
                0
            },
            order_processing_count,
            matching_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LatencyMetricsSnapshot {
    pub avg_order_processing_time: u64, 
    pub avg_matching_time: u64,         
    pub order_processing_count: u64,
    pub matching_count: u64,
}
