use nalgebra_glm as glm;

pub trait Animation {
    fn update(&mut self, delta_time: f32);
    fn is_finished(&self) -> bool;
}

pub struct FadeAnimation {
    current: f32,
    start: f32,
    end: f32,
    duration: f32,
    elapsed: f32,
}

pub struct Vec2Animation {
    current: glm::Vec2,
    start: glm::Vec2,
    end: glm::Vec2,
    duration: f32,
    elapsed: f32,
}

impl FadeAnimation {
    pub fn new(start: f32, end: f32, duration: f32) -> Self {
        Self {
            current: start,
            start,
            end,
            duration,
            elapsed: 0.0,
        }
    }

    pub fn value(&self) -> f32 {
        self.current
    }
}

impl Animation for FadeAnimation {
    fn update(&mut self, delta_time: f32) {
        self.elapsed += delta_time;
        let t = (self.elapsed / self.duration).min(1.0);
        self.current = self.start + (self.end - self.start) * t;
    }

    fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

impl Vec2Animation {
    pub fn new(start: glm::Vec2, end: glm::Vec2, duration: f32) -> Self {
        Self {
            current: start,
            start,
            end,
            duration,
            elapsed: 0.0,
        }
    }

    pub fn value(&self) -> glm::Vec2 {
        self.current
    }
}

impl Animation for Vec2Animation {
    fn update(&mut self, delta_time: f32) {
        self.elapsed += delta_time;
        let t = (self.elapsed / self.duration).min(1.0);
        self.current = self.start + (self.end - self.start) * t;
    }

    fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

pub struct AnimationManager {
    position_animations: Vec<Vec2Animation>,
    fade_animations: Vec<FadeAnimation>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            position_animations: Vec::new(),
            fade_animations: Vec::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // 벡터 순회하면서 한 번에 업데이트
        self.position_animations.retain_mut(|anim| {
            anim.update(delta_time);
            !anim.is_finished()
        });

        self.fade_animations.retain_mut(|anim| {
            anim.update(delta_time);
            !anim.is_finished()
        });
    }
}
