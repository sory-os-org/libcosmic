use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpringConfig {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub precision: f32,
}

impl SpringConfig {
    pub const SNAPPY: Self = Self {
        stiffness: 300.0,
        damping: 25.0,
        mass: 1.0,
        precision: 0.5,
    };

    pub const GENTLE: Self = Self {
        stiffness: 150.0,
        damping: 18.0,
        mass: 1.0,
        precision: 1.0,
    };

    pub const WOBBLY: Self = Self {
        stiffness: 200.0,
        damping: 10.0,
        mass: 1.0,
        precision: 0.5,
    };

    pub const STIFF: Self = Self {
        stiffness: 500.0,
        damping: 35.0,
        mass: 1.0,
        precision: 0.5,
    };

    pub const MODAL: Self = Self {
        stiffness: 300.0,
        damping: 30.0,
        mass: 1.0,
        precision: 0.5,
    };

    pub const SLIDE: Self = Self {
        stiffness: 250.0,
        damping: 22.0,
        mass: 1.0,
        precision: 1.0,
    };

    pub const CRITICAL: Self = Self {
        stiffness: 225.0,
        damping: 30.0,
        mass: 1.0,
        precision: 0.001,
    };
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self::GENTLE
    }
}

#[derive(Debug, Clone)]
pub struct Spring<T> {
    pub value: T,
    pub velocity: T,
    pub target: T,
    pub config: SpringConfig,
    pub last_update: Option<Instant>,
}

impl Spring<f32> {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            config: SpringConfig::default(),
            last_update: None,
        }
    }

    pub fn new_with_config(value: f32, config: SpringConfig) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            config,
            last_update: None,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn snap_to(&mut self, target: f32) {
        self.value = target;
        self.velocity = 0.0;
        self.target = target;
        self.last_update = None;
    }

    pub fn update(&mut self, now: Instant) -> bool {
        let dt = match self.last_update {
            Some(last) => {
                let dt = now.duration_since(last).as_secs_f32();
                if dt <= 0.0 {
                    return false;
                }
                dt.min(0.05)
            }
            None => {
                self.last_update = Some(now);
                return false;
            }
        };

        let displacement = self.value - self.target;
        let force = -self.config.stiffness * displacement - self.config.damping * self.velocity;
        let acceleration = force / self.config.mass;

        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        self.last_update = Some(now);

        let is_moving = self.velocity.abs() > self.config.precision
            || (self.value - self.target).abs() > self.config.precision;

        if !is_moving && self.velocity.abs() > 0.0 {
            self.value = self.target;
            self.velocity = 0.0;
        }

        is_moving
    }

    pub fn is_at_rest(&self) -> bool {
        (self.value - self.target).abs() <= self.config.precision && self.velocity.abs() <= self.config.precision
    }
}

impl Spring<f64> {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            config: SpringConfig::default(),
            last_update: None,
        }
    }

    pub fn new_with_config(value: f64, config: SpringConfig) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            config,
            last_update: None,
        }
    }

    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    pub fn snap_to(&mut self, target: f64) {
        self.value = target;
        self.velocity = 0.0;
        self.target = target;
        self.last_update = None;
    }

    pub fn update(&mut self, now: Instant) -> bool {
        let dt = match self.last_update {
            Some(last) => {
                let dt = now.duration_since(last).as_secs_f64();
                if dt <= 0.0 {
                    return false;
                }
                dt.min(0.05)
            }
            None => {
                self.last_update = Some(now);
                return false;
            }
        };

        let prec = self.config.precision as f64;
        let stf = self.config.stiffness as f64;
        let damp = self.config.damping as f64;
        let mass = self.config.mass as f64;

        let displacement = self.value - self.target;
        let force = -stf * displacement - damp * self.velocity;
        let acceleration = force / mass;

        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        self.last_update = Some(now);

        let is_moving = self.velocity.abs() > prec || (self.value - self.target).abs() > prec;

        if !is_moving && self.velocity.abs() > 0.0 {
            self.value = self.target;
            self.velocity = 0.0;
        }

        is_moving
    }

    pub fn is_at_rest(&self) -> bool {
        let prec = self.config.precision as f64;
        (self.value - self.target).abs() <= prec && self.velocity.abs() <= prec
    }
}
