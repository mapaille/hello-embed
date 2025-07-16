pub trait Cancellable {
    fn cancel(&self);
}

pub trait Resettable {
    fn reset(&self);
}

pub trait Pressable {
    fn is_pressed(&self) -> bool;
}
