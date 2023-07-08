use crate::translations::Translator;
use leptos::{Memo, SignalWith};
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Groups {
	News,
	Science,
	Covert,
	Public,
}

impl Groups {
	pub fn decay_rate(self) -> u32 {
		match self {
			Groups::News => 150,
			Groups::Science => 50,
			Groups::Covert => 100,
			Groups::Public => 200,
		}
	}

	pub fn name(self, translator: Memo<Rc<Translator>>) -> String {
		let key = match self {
			Groups::News => "news",
			Groups::Science => "science",
			Groups::Covert => "covert",
			Groups::Public => "public",
		};
		translator.with(|t| t.t(key))
	}
}
