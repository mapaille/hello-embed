pub struct Frame<const X: usize, const Y: usize>(pub &'static[[bool; X]; Y]);
