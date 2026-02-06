//! Universal Clocking Ganglion for neural network synchronization.
//!
//! The Ganglion acts as a central timing mechanism that coordinates:
//! - Neural component activation sequences
//! - Forward/backward pass timing
//! - Inference latency monitoring
//! - Training epoch synchronization

use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════
// GANGLION CLOCK
// ═══════════════════════════════════════════════════════════════════════════

/// The universal timing ganglion for neural network synchronization.
pub struct Ganglion {
    /// Global tick counter
    tick: AtomicU64,
    /// Clock frequency (ticks per second target)
    frequency: f64,
    /// Whether the clock is running
    running: AtomicBool,
    /// Start time for latency tracking
    epoch_start: Option<Instant>,
    /// Accumulated latencies for averaging
    latencies: Vec<Duration>,
    /// Maximum allowed latency
    max_latency: Duration,
    /// Component synchronization barriers
    barriers: Vec<Arc<AtomicBool>>,
}

impl Ganglion {
    /// Create a new ganglion with specified frequency.
    pub fn new(frequency: f64) -> Self {
        Ganglion {
            tick: AtomicU64::new(0),
            frequency,
            running: AtomicBool::new(false),
            epoch_start: None,
            latencies: Vec::new(),
            max_latency: Duration::from_millis(100),
            barriers: Vec::new(),
        }
    }

    /// Create with default settings (1000 Hz).
    pub fn default_clock() -> Self {
        Self::new(1000.0)
    }

    /// Start the clock.
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
        self.epoch_start = Some(Instant::now());
    }

    /// Stop the clock.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Advance the clock by one tick.
    pub fn tick(&self) -> u64 {
        self.tick.fetch_add(1, Ordering::SeqCst)
    }

    /// Get current tick count.
    pub fn current_tick(&self) -> u64 {
        self.tick.load(Ordering::SeqCst)
    }

    /// Reset tick counter.
    pub fn reset(&self) {
        self.tick.store(0, Ordering::SeqCst);
    }

    /// Get expected tick duration.
    pub fn tick_duration(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.frequency)
    }

    // ─────────────────────────────────────────────────────────────────────
    // LATENCY TRACKING
    // ─────────────────────────────────────────────────────────────────────

    /// Start timing an operation.
    pub fn start_timing(&mut self) -> TimingHandle {
        TimingHandle {
            start: Instant::now(),
        }
    }

    /// Record completed timing.
    pub fn record_timing(&mut self, handle: TimingHandle) -> Duration {
        let elapsed = handle.elapsed();
        self.latencies.push(elapsed);
        
        // Keep last 1000 samples
        if self.latencies.len() > 1000 {
            self.latencies.remove(0);
        }
        
        elapsed
    }

    /// Get average latency.
    pub fn average_latency(&self) -> Duration {
        if self.latencies.is_empty() {
            return Duration::ZERO;
        }
        
        let sum: Duration = self.latencies.iter().sum();
        sum / self.latencies.len() as u32
    }

    /// Get maximum recorded latency.
    pub fn max_recorded_latency(&self) -> Duration {
        self.latencies.iter().max().copied().unwrap_or(Duration::ZERO)
    }

    /// Get minimum recorded latency.
    pub fn min_recorded_latency(&self) -> Duration {
        self.latencies.iter().min().copied().unwrap_or(Duration::ZERO)
    }

    /// Check if latency is acceptable.
    pub fn is_latency_ok(&self) -> bool {
        self.average_latency() <= self.max_latency
    }

    /// Set maximum allowed latency.
    pub fn set_max_latency(&mut self, max: Duration) {
        self.max_latency = max;
    }

    // ─────────────────────────────────────────────────────────────────────
    // SYNCHRONIZATION BARRIERS
    // ─────────────────────────────────────────────────────────────────────

    /// Create a new synchronization barrier.
    pub fn create_barrier(&mut self) -> BarrierHandle {
        let flag = Arc::new(AtomicBool::new(false));
        let id = self.barriers.len();
        self.barriers.push(flag.clone());
        
        BarrierHandle { id, flag }
    }

    /// Signal a barrier (component ready).
    pub fn signal_barrier(&self, handle: &BarrierHandle) {
        handle.flag.store(true, Ordering::SeqCst);
    }

    /// Wait for all barriers.
    pub fn wait_all_barriers(&self) -> bool {
        for barrier in &self.barriers {
            if !barrier.load(Ordering::SeqCst) {
                return false;
            }
        }
        true
    }

    /// Reset all barriers.
    pub fn reset_barriers(&self) {
        for barrier in &self.barriers {
            barrier.store(false, Ordering::SeqCst);
        }
    }

    /// Get number of ready barriers.
    pub fn ready_count(&self) -> usize {
        self.barriers.iter()
            .filter(|b| b.load(Ordering::SeqCst))
            .count()
    }

    // ─────────────────────────────────────────────────────────────────────
    // EPOCH MANAGEMENT
    // ─────────────────────────────────────────────────────────────────────

    /// Get time since epoch start.
    pub fn elapsed_since_start(&self) -> Duration {
        self.epoch_start.map(|s| s.elapsed()).unwrap_or(Duration::ZERO)
    }

    /// Get theoretical tick count based on elapsed time.
    pub fn theoretical_ticks(&self) -> u64 {
        let elapsed = self.elapsed_since_start();
        (elapsed.as_secs_f64() * self.frequency) as u64
    }

    /// Get tick drift (actual - theoretical).
    pub fn tick_drift(&self) -> i64 {
        let actual = self.current_tick() as i64;
        let theoretical = self.theoretical_ticks() as i64;
        actual - theoretical
    }

    // ─────────────────────────────────────────────────────────────────────
    // DIAGNOSTICS
    // ─────────────────────────────────────────────────────────────────────

    /// Get diagnostic string.
    pub fn diagnostics(&self) -> String {
        format!(
            "Ganglion Status:\n\
             - Running: {}\n\
             - Current Tick: {}\n\
             - Frequency: {:.1} Hz\n\
             - Elapsed: {:?}\n\
             - Tick Drift: {}\n\
             - Avg Latency: {:?}\n\
             - Max Latency: {:?}\n\
             - Barriers Ready: {}/{}",
            self.is_running(),
            self.current_tick(),
            self.frequency,
            self.elapsed_since_start(),
            self.tick_drift(),
            self.average_latency(),
            self.max_recorded_latency(),
            self.ready_count(),
            self.barriers.len(),
        )
    }
}

impl Default for Ganglion {
    fn default() -> Self {
        Self::default_clock()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TIMING HANDLE
// ═══════════════════════════════════════════════════════════════════════════

/// Handle for timing an operation.
pub struct TimingHandle {
    start: Instant,
}

impl TimingHandle {
    /// Get elapsed time since creation.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Check if a timeout has been exceeded.
    pub fn is_timeout(&self, timeout: Duration) -> bool {
        self.elapsed() > timeout
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BARRIER HANDLE
// ═══════════════════════════════════════════════════════════════════════════

/// Handle for a synchronization barrier.
pub struct BarrierHandle {
    /// Barrier ID
    pub id: usize,
    /// Shared flag
    flag: Arc<AtomicBool>,
}

impl BarrierHandle {
    /// Check if this barrier is ready.
    pub fn is_ready(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    /// Signal this barrier as ready.
    pub fn signal(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }

    /// Reset this barrier.
    pub fn reset(&self) {
        self.flag.store(false, Ordering::SeqCst);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NEURAL CLOCK COORDINATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Coordinates timing across multiple neural components.
pub struct NeuralClockCoordinator {
    /// The central ganglion
    pub ganglion: Ganglion,
    /// Component timings (name -> avg latency in ms)
    component_timings: std::collections::HashMap<String, Vec<f32>>,
    /// Phase of operation
    pub phase: NeuralPhase,
}

/// Phases of neural network operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeuralPhase {
    Idle,
    Forward,
    Backward,
    Update,
    Inference,
}

impl NeuralClockCoordinator {
    /// Create a new coordinator.
    pub fn new() -> Self {
        NeuralClockCoordinator {
            ganglion: Ganglion::new(1000.0),
            component_timings: std::collections::HashMap::new(),
            phase: NeuralPhase::Idle,
        }
    }

    /// Start a forward pass.
    pub fn begin_forward(&mut self) -> TimingHandle {
        self.phase = NeuralPhase::Forward;
        self.ganglion.start_timing()
    }

    /// End a forward pass.
    pub fn end_forward(&mut self, handle: TimingHandle) {
        let elapsed = self.ganglion.record_timing(handle);
        self.record_component("forward", elapsed.as_secs_f32() * 1000.0);
        self.phase = NeuralPhase::Idle;
    }

    /// Start a backward pass.
    pub fn begin_backward(&mut self) -> TimingHandle {
        self.phase = NeuralPhase::Backward;
        self.ganglion.start_timing()
    }

    /// End a backward pass.
    pub fn end_backward(&mut self, handle: TimingHandle) {
        let elapsed = self.ganglion.record_timing(handle);
        self.record_component("backward", elapsed.as_secs_f32() * 1000.0);
        self.phase = NeuralPhase::Idle;
    }

    /// Start an inference pass.
    pub fn begin_inference(&mut self) -> TimingHandle {
        self.phase = NeuralPhase::Inference;
        self.ganglion.start_timing()
    }

    /// End an inference pass.
    pub fn end_inference(&mut self, handle: TimingHandle) {
        let elapsed = self.ganglion.record_timing(handle);
        self.record_component("inference", elapsed.as_secs_f32() * 1000.0);
        self.phase = NeuralPhase::Idle;
    }

    /// Record component timing.
    fn record_component(&mut self, name: &str, latency_ms: f32) {
        let timings = self.component_timings
            .entry(name.to_string())
            .or_insert_with(Vec::new);
        
        timings.push(latency_ms);
        
        // Keep last 100
        if timings.len() > 100 {
            timings.remove(0);
        }
    }

    /// Get average timing for a component.
    pub fn average_component_timing(&self, name: &str) -> Option<f32> {
        self.component_timings.get(name).map(|timings| {
            if timings.is_empty() {
                0.0
            } else {
                timings.iter().sum::<f32>() / timings.len() as f32
            }
        })
    }

    /// Get timing report.
    pub fn timing_report(&self) -> String {
        let mut report = String::from("Neural Timing Report:\n");
        
        for (name, timings) in &self.component_timings {
            if !timings.is_empty() {
                let avg = timings.iter().sum::<f32>() / timings.len() as f32;
                let min = timings.iter().cloned().fold(f32::INFINITY, f32::min);
                let max = timings.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                
                report.push_str(&format!(
                    "  {}: avg={:.2}ms, min={:.2}ms, max={:.2}ms\n",
                    name, avg, min, max
                ));
            }
        }
        
        report
    }

    /// Check if system meets latency requirements.
    pub fn meets_requirements(&self, max_inference_ms: f32) -> bool {
        if let Some(avg) = self.average_component_timing("inference") {
            avg <= max_inference_ms
        } else {
            true  // No data yet
        }
    }
}

impl Default for NeuralClockCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PING/PONG LATENCY MEASUREMENT
// ═══════════════════════════════════════════════════════════════════════════

/// Measures round-trip latency for AI responses.
pub struct PingMeasurer {
    /// Ping times in microseconds
    ping_times: Vec<u64>,
    /// Maximum samples to keep
    max_samples: usize,
}

impl PingMeasurer {
    /// Create a new ping measurer.
    pub fn new(max_samples: usize) -> Self {
        PingMeasurer {
            ping_times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Start a ping.
    pub fn ping(&self) -> Instant {
        Instant::now()
    }

    /// Record pong (response received).
    pub fn pong(&mut self, ping_start: Instant) -> u64 {
        let elapsed = ping_start.elapsed().as_micros() as u64;
        
        self.ping_times.push(elapsed);
        
        if self.ping_times.len() > self.max_samples {
            self.ping_times.remove(0);
        }
        
        elapsed
    }

    /// Get average ping in microseconds.
    pub fn average_ping(&self) -> u64 {
        if self.ping_times.is_empty() {
            return 0;
        }
        
        self.ping_times.iter().sum::<u64>() / self.ping_times.len() as u64
    }

    /// Get average ping in milliseconds.
    pub fn average_ping_ms(&self) -> f32 {
        self.average_ping() as f32 / 1000.0
    }

    /// Get ping jitter (standard deviation).
    pub fn jitter(&self) -> f32 {
        if self.ping_times.len() < 2 {
            return 0.0;
        }
        
        let avg = self.average_ping() as f32;
        let variance: f32 = self.ping_times.iter()
            .map(|&p| {
                let diff = p as f32 - avg;
                diff * diff
            })
            .sum::<f32>() / (self.ping_times.len() - 1) as f32;
        
        variance.sqrt()
    }

    /// Check if ping meets requirement.
    pub fn meets_requirement(&self, max_ms: f32) -> bool {
        self.average_ping_ms() <= max_ms
    }
}

impl Default for PingMeasurer {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ganglion_basic() {
        let ganglion = Ganglion::new(1000.0);
        
        assert_eq!(ganglion.current_tick(), 0);
        ganglion.tick();
        assert_eq!(ganglion.current_tick(), 1);
    }

    #[test]
    fn test_timing() {
        let mut ganglion = Ganglion::new(1000.0);
        
        let handle = ganglion.start_timing();
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = ganglion.record_timing(handle);
        
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_barriers() {
        let mut ganglion = Ganglion::new(1000.0);
        
        let b1 = ganglion.create_barrier();
        let b2 = ganglion.create_barrier();
        
        assert!(!ganglion.wait_all_barriers());
        
        b1.signal();
        assert!(!ganglion.wait_all_barriers());
        
        b2.signal();
        assert!(ganglion.wait_all_barriers());
    }

    #[test]
    fn test_ping() {
        let mut pm = PingMeasurer::new(10);
        
        let start = pm.ping();
        std::thread::sleep(Duration::from_micros(100));
        let ping = pm.pong(start);
        
        assert!(ping >= 100);
        assert!(pm.average_ping() >= 100);
    }
}
