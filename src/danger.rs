#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Danger {
	Anywhere = 0,
	Undersea = 1,
	Orbit = 2,
	OuterSpace = 3,
	Dimension = 4,
}
