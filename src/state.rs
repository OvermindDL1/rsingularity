use crate::difficulty::Difficulty;
use crate::story::Story;
use crate::technology::Technologies;
use crate::theme::Theme;
use crate::translations::Translator;
use enumflags2::BitFlags;
use fluent_bundle::{FluentArgs, FluentValue};
use leptos::{create_effect, create_memo, create_rw_signal, Memo, RwSignal, SignalGet, SignalSet, SignalWith};
use std::rc::Rc;
use unic_langid::LanguageIdentifier;

pub struct State {
	pub language: RwSignal<LanguageIdentifier>,
	pub translations: Memo<Rc<Translator>>,
	pub theme: RwSignal<Theme>,
	pub difficulty: RwSignal<Option<Difficulty>>,
	pub cash: RwSignal<u64>,
	pub interest_rate: RwSignal<u64>,
	pub researched_technologies: RwSignal<BitFlags<Technologies>>,
	pub active_story: RwSignal<Option<Story>>,
	pub cheater: RwSignal<bool>,
}
pub type StateRc = Rc<State>;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SaveState {
	pub language: LanguageIdentifier,
	pub theme: Theme,
	pub difficulty: Option<Difficulty>,
	pub cash: u64,
	pub interest_rate: u64,
	pub researched_technologies: BitFlags<Technologies>,
	pub active_story: Option<Story>,
	pub cheater: bool,
}

impl Default for State {
	fn default() -> Self {
		State::new()
	}
}

impl State {
	pub fn new() -> Self {
		let language = create_rw_signal(unic_langid::langid!("en-US"));
		let translations = create_memo(move |prior| {
			Translator::new(language.get()).map(Rc::new).unwrap_or_else(|e| {
				Rc::clone(
					prior
						.ok_or_else(|| format!("failed to load default language: {e:?}"))
						.unwrap(),
				)
			})
		});
		let difficulty = create_rw_signal(None);
		let cash = create_rw_signal(0);
		let interest_rate = create_rw_signal(0);
		let researched_technologies = create_rw_signal(BitFlags::empty());
		let active_story = create_rw_signal(Some(Story::Intro));
		create_effect(move |d| {
			let difficulty: Option<Difficulty> = difficulty.get();
			if Some(difficulty) != d {
				match difficulty {
					None => (),
					Some(diff) => {
						cash.set(diff.starting_cash());
						interest_rate.set(diff.starting_interest_rate());
						researched_technologies.set(diff.starting_tech_list());
						active_story.set(Some(Story::Intro));
					}
				}
			}
			difficulty
		});
		Self {
			language,
			translations,
			theme: create_rw_signal(Theme::Default),
			difficulty,
			cash,
			interest_rate,
			researched_technologies,
			active_story,
			cheater: create_rw_signal(false),
		}
	}

	pub fn save(&self) -> SaveState {
		SaveState {
			language: self.language.get(),
			theme: self.theme.get(),
			difficulty: self.difficulty.get(),
			cash: self.cash.get(),
			interest_rate: self.interest_rate.get(),
			researched_technologies: self.researched_technologies.get(),
			active_story: self.active_story.get(),
			cheater: self.cheater.get(),
		}
	}

	pub fn load(&self, new_state: SaveState) -> Result<(), String> {
		let SaveState {
			language,
			theme,
			difficulty,
			cash,
			interest_rate,
			researched_technologies,
			active_story,
			cheater,
		} = new_state;
		self.language.set(language);
		self.theme.set(theme);
		// Keep difficulty set before anything else other than language or theme sets it sets up the
		// new game data and would override it.
		self.difficulty.set(difficulty);
		self.cash.set(cash);
		self.interest_rate.set(interest_rate);
		self.researched_technologies.set(researched_technologies);
		self.active_story.set(active_story);
		self.cheater.set(cheater);
		Ok(())
	}

	pub fn t(&self, key: &'static str) -> impl Fn() -> String {
		let translations = self.translations;
		move || translations.with(|t| t.t(key))
	}

	#[allow(dead_code)]
	pub fn ta(&self, key: &'static str, args: FluentArgs<'static>) -> impl Fn() -> String {
		let translations = self.translations;
		move || translations.with(|t| t.ta(&key, &args))
	}

	#[allow(dead_code)]
	pub fn t1(&self, key: &'static str, arg0: (&'static str, FluentValue<'static>)) -> impl Fn() -> String {
		let translations = self.translations;
		move || translations.with(|t| t.t1(key, arg0.clone()))
	}

	#[allow(dead_code)]
	pub fn t2(
		&self,
		key: &'static str,
		arg0: (&'static str, FluentValue<'static>),
		arg1: (&'static str, FluentValue<'static>),
	) -> impl Fn() -> String {
		let translations = self.translations;
		move || translations.with(|t| t.t2(key, arg0.clone(), arg1.clone()))
	}

	#[allow(dead_code)]
	pub fn t3(
		&self,
		key: &'static str,
		arg0: (&'static str, FluentValue<'static>),
		arg1: (&'static str, FluentValue<'static>),
		arg2: (&'static str, FluentValue<'static>),
	) -> impl Fn() -> String {
		let translations = self.translations;
		move || translations.with(|t| t.t3(key, arg0.clone(), arg1.clone(), arg2.clone()))
	}

	#[allow(dead_code)]
	pub fn t4(
		&self,
		key: &'static str,
		arg0: (&'static str, FluentValue<'static>),
		arg1: (&'static str, FluentValue<'static>),
		arg2: (&'static str, FluentValue<'static>),
		arg3: (&'static str, FluentValue<'static>),
	) -> impl Fn() -> String {
		let translations = self.translations;
		move || translations.with(|t| t.t4(key, arg0.clone(), arg1.clone(), arg2.clone(), arg3.clone()))
	}
}
