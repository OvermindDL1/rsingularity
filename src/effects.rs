use crate::groups::Groups;

#[derive(Clone, Copy)]
pub enum DisplayDiscovered {
	None,
	Partial,
	Full,
}

#[derive(Clone, Copy)]
pub enum Effects {
	Endgame,
	DisplayDiscover(DisplayDiscovered),
	Discover(Groups, i32),
	SuspicionDecay(Groups, i32),
	SuspicionAdjust(Groups, i32),
	Interest(u32),
	Income(u32),
	CostLabor(u32),
	JobProfit(u32),
}
