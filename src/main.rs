use std::time::{Instant, Duration};
use std::thread::sleep;

/// Stopwatch definition
struct Stopwatch { start_point: Instant
    , interval_start: f64
    , lap_start: f64
    , end: f64
    , elapsed: f64
    , laps:Vec<f64>
}

///Stopwatch implementation
impl Stopwatch {
  fn new() -> Stopwatch {
    let ts = Instant::now();

    Stopwatch { start_point: ts
        , interval_start: 0.0
        , lap_start: 0.0
        , end:3.0
        , elapsed:0.0
        , laps: Vec::new()
    }
  }

  //Returns a timepoint as a float
  fn get_timepoint(&self) -> f64 {
    let now = Instant::now();
    let duration = now.duration_since(self.start_point);

    let secs = duration.as_secs() as f64;
    let nsecs = duration.subsec_nanos() as f64 / 1.0e9;

    let tp = secs + nsecs;

    return tp;
  }

  /// starts the stopwatch timer
  /// return: -1 value if the clock is already running or new start point
  ///
  /// Starts a new timer interval in the stopwatch. If the sotpwatch is active,
  /// elapsed time is incremented and a new interval is started.
  pub fn start(&mut self) -> f64 {
    let tp = self.get_timepoint();
    self.end = tp;

    //If we're already running, return -1.0
    if self.interval_start != 0.0 {
      //SDF generate an error?
      return -1.0;
    }

    //Not running so set the interval_start and lap_start to the current 
    //timepoint
    self.interval_start = tp;

    //Return the current start point
    return tp;
  }

  /// Ends the current time interval which effectively stops the timer.
  pub fn stop(&mut self)->f64 {
      //Get the current timepoint
      let tp = self.get_timepoint();
      self.end = tp;

      //Check if we're already stopped
      if self.interval_start == 0.0 {
        //SDF generate an error?
        return -1.0;
      }

      //Update elapsed time
      let interval = tp - self.interval_start;
      self.elapsed = self.elapsed + interval;

      //Update lap
      self.laps.push(interval);

      //Clear lap_start and interval_start variables
      self.interval_start = 0.0;
      self.lap_start = 0.0;
      return self.elapsed;
  }

  //Function to stop one lap and start the next one
  pub fn lap( &mut self ) -> f64 {
    //Get the current timepoint
    let tp = self.get_timepoint();
    self.end = tp;

    //Check if we're already stopped
    if self.interval_start == 0.0 {
      //SDF generate an error?
      return -1.0;
    }

    let interval = tp - self.interval_start;
    self.elapsed = self.elapsed + interval;

    self.laps.push(interval);

    self.interval_start = tp;

    return interval;
  }

  //Returns the time for a specific lap
  pub fn get_lap(&mut self, index: usize ) -> f64 {
      if index < self.laps.len() {
        return self.laps[index];
      }

      return -1.0;
  }

  pub fn get_lap_count( &mut self  ) -> usize {
    return self.laps.len();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_start_stop() {
    let mut sw = Stopwatch::new(); 
    sw.start();
    let elapsed = sw.stop();

    //This is the maximum allowed time for sequential start/stops
    let max=0.000001;

    assert!(elapsed > 0.0, "ERROR: elapsed time of {} <= 0.0", elapsed);
    assert!(elapsed < max, "ERROR: elapsed time of {} > {}", elapsed, max);
  }

  #[test] 
  //This test validates the start/stop timing with a given delay
  fn test_start_stop_delay() {
    let delay_time = 1;
    let delay = Duration::new(delay_time, 0);
    let max=delay_time as f64 + 0.0003;
    let min=delay_time as f64 - 0.0003;

    let mut sw = Stopwatch::new(); 
    sw.start();
    sleep(delay);
    let elapsed = sw.stop();
    assert!(elapsed > 0.0, "ERROR: elapsed time of {} <= 0.0", elapsed);
    assert!(elapsed < max, "ERROR: elapsed time of {} > max time of {}", elapsed, max);
    assert!(elapsed > min, "ERROR: elapsed time of {} < min time of {}", elapsed, min);
  }

  #[test] 
  //This test validates the start/stop timing with a given delay
  fn test_start_stop_delay_twice() {
    let delay_time = 0.5;
    let range = 0.0006;

    let delay = Duration::new(0, (1.0e9*delay_time) as u32);
    let max = 2.0 * delay_time + range;
    let min = 2.0 * delay_time - range;

    println!("delay: {}, MIN: {}, MAX: {}",delay_time, min, max);

    let mut sw = Stopwatch::new(); 
    sw.start();
    sleep(delay);
    sw.stop();
    sleep(delay);
    sw.start();
    sleep(delay);
    let elapsed = sw.stop();

    assert!(elapsed > 0.0, "ERROR: elapsed time of {} <= 0.0", elapsed);
    assert!(elapsed < max, "ERROR: elapsed time of {} > max time of {}", elapsed, max);
    assert!(elapsed > min, "ERROR: elapsed time of {} < min time of {}", elapsed, min );
  }

  #[test]
  //Test lap function by creating 5 laps at regular intervales
  fn test_lap() {
    let delay_time = 0.5;
    let range = 0.0005;
    let laps = 10;
    let min = delay_time - range;
    let max = delay_time + range;

    let delay = Duration::new(0, (1.0e9*delay_time) as u32);

    let mut sw = Stopwatch::new(); 
    sw.start();

    for _n in 0..laps {
      sleep(delay);
      sw.lap();
    }

    let mut total = 0.0;
    for i in 0..laps {
      total = total + sw.get_lap(i);
    }

    let values = sw.get_lap_count();
    assert!( values == laps, "ERROR: Values {} does match laps {}", values, laps);

    let avg = total as f64/ laps as f64;
    assert!( avg > min, "ERROR: average {} less than min of {}", avg, min );
    assert!( avg < max, "ERROR: average {} greater than max of {}", avg, max );
  }

  #[test]
  //Test getLap with known invalid indices
  fn get_lap() {
      let mut sw = Stopwatch::new();

      let mut result = sw.get_lap(10);
      assert!( result == -1.0, "ERROR Invalid lap did not result in -1 return");

      //Start twice
      result = sw.start();
      assert!(result > 0.0, "ERROR: start did not provide a positive timestamp");
      result = sw.start();
      assert!(result == -1.0, "ERROR: duplicate start did not fail {} != -1.0", result );
  }
}

/// Time to learn rust
///
fn main() {
    let mut sw = Stopwatch::new(); 

    let delay = Duration::new(1, 0 );
    
    sw.start();

    if true {
      sleep( delay);
    }

    let elapsed = sw.stop();

    println!("Runtime: {}", elapsed);
}

