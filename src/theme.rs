pub enum Theme {
	Default,
	Nightmode,
	Vector,
}

impl Theme {
	pub fn css_class(&self) -> &'static str {
		match self {
			Theme::Default => "theme-Default",
			Theme::Nightmode => "theme-Nightmode",
			Theme::Vector => "theme-Vector",
		}
	}
}
