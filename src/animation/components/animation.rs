use std::{ops::Range, time::Duration};

use bevy::{
    ecs::component::Component,
    time::{Timer, TimerMode},
};

#[derive(Component, Debug)]
pub(crate) struct Animation {
    frame: usize,
    frames: Range<usize>,
    frames_per_second: u8,
    is_paused: bool,
    timer: Timer,
}

impl Animation {
    pub fn new(frames: Range<usize>, frames_per_second: u8) -> Self {
        Self {
            frame: frames.start,
            frames,
            frames_per_second,
            is_paused: false,
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / (frames_per_second as f32)),
                TimerMode::Repeating,
            ),
        }
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn pause(&mut self) {
        // Skip if already paused
        if self.is_paused {
            return;
        }

        self.timer.pause();
        self.is_paused = true;
    }

    pub fn reset(&mut self) {
        self.frame = self.frames.start;
        self.timer.reset();
    }

    pub fn resume(&mut self) {
        // Skip if not paused
        if !self.is_paused {
            return;
        }

        self.timer.unpause();
        self.is_paused = false;
    }

    pub fn step(&mut self) -> (usize, usize) {
        let initial_frame = self.frame;

        // Skip if paused
        if self.is_paused {
            return (initial_frame, self.frame);
        }

        // Reset timer
        self.timer.reset();
        self.advance(1);

        (initial_frame, self.frame)
    }

    pub fn tick(&mut self, duration: Duration) -> (usize, usize) {
        let initial_frame = self.frame;

        // Skip if paused
        if self.is_paused {
            return (initial_frame, self.frame);
        }

        // Move timer forward
        self.timer.tick(duration);

        // If timer is finished, move to the next frame
        if self.timer.just_finished() {
            self.advance(self.timer.times_finished_this_tick() as usize);
        }

        (initial_frame, self.frame)
    }

    fn advance(&mut self, count: usize) {
        let delta = count % (self.frames.end - self.frames.start);
        let next_frame = self.frame + delta;

        // If we've reached the end of the loop, wrap around by # frames advanced
        self.frame = if next_frame >= self.frames.end {
            self.frames.start + (next_frame - self.frames.end)
        } else {
            next_frame
        };
    }
}
