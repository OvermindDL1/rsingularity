pub mod assets;
pub mod danger;
pub mod difficulty;
pub mod effects;
pub mod groups;
pub mod story;
pub mod technology;
pub mod translations;

use enumflags2::BitFlags;
use fluent_bundle::{FluentArgs, FluentValue};
use leptos::*;
use log::Level;
use std::rc::Rc;
use unic_langid::LanguageIdentifier;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlOptionElement, HtmlSelectElement};

use crate::difficulty::Difficulty;
use crate::story::Story;
use crate::technology::Technologies;
use crate::translations::Translator;

struct State {
	language: RwSignal<LanguageIdentifier>,
	translations: Memo<Rc<Translator>>,
	difficulty: RwSignal<Option<Difficulty>>,
	cash: RwSignal<u64>,
	interest_rate: RwSignal<u64>,
	researched_technologies: RwSignal<BitFlags<Technologies>>,
	active_story: RwSignal<Option<Story>>,
}
type StateRc = Rc<State>;

impl State {
	pub fn new(cx: Scope) -> Self {
		let language = create_rw_signal(cx, unic_langid::langid!("en-US"));
		let translations = create_memo(cx, move |prior| {
			Translator::new(language.get()).map(Rc::new).unwrap_or_else(|e| {
				Rc::clone(
					prior
						.ok_or_else(|| format!("failed to load default language: {e:?}"))
						.unwrap(),
				)
			})
		});
		let difficulty = create_rw_signal(cx, None);
		let cash = create_rw_signal(cx, 0);
		let interest_rate = create_rw_signal(cx, 0);
		let researched_technologies = create_rw_signal(cx, BitFlags::empty());
		let active_story = create_rw_signal(cx, Some(Story::Intro));
		create_effect(cx, move |d| {
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
			difficulty,
			cash,
			interest_rate,
			researched_technologies,
			active_story,
		}
	}

	pub fn t(&self, key: &'static str) -> impl Fn() -> String {
		let translations = self.translations;
		move || translations.with(|t| t.t(&key))
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

#[component]
fn MainMenu(cx: Scope, state: StateRc) -> impl IntoView {
	let State {
		language, difficulty, ..
	} = *state;
	view! { cx,
		<div id="main_menu">
			<div id="main_menu_title">{state.t("title")}</div>
			<div id="main_menu_difficulty">
				<div id="main_menu_difficulty_title">{state.t("difficulty.title")}</div>
				<button id="main_menu_new_very_easy_button" on:click=move |_ev| difficulty.set(Some(Difficulty::VeryEasy))>{state.t1("difficulty", ("level", Difficulty::VeryEasy.into()))}</button>
				<button id="main_menu_new_easy_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Easy))>{state.t1("difficulty", ("level", Difficulty::Easy.into()))}</button>
				<button id="main_menu_new_normal_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Normal))>{state.t1("difficulty", ("level", Difficulty::Normal.into()))}</button>
				<button id="main_menu_new_hard_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Hard))>{state.t1("difficulty", ("level", Difficulty::Hard.into()))}</button>
				<button id="main_menu_new_ultra_hard_button" on:click=move |_ev| difficulty.set(Some(Difficulty::UltraHard))>{state.t1("difficulty", ("level", Difficulty::UltraHard.into()))}</button>
				<button id="main_menu_new_impossible_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Impossible))>{state.t1("difficulty", ("level", Difficulty::Impossible.into()))}</button>
			</div>
			<div id="main_menu_load">
				<div id="main_menu_load_title">{state.t("load-game")}</div>
				<textarea id="main_menu_load_textarea" placeholder={state.t("load-game.placeholder")}></textarea>
			</div>
			<div id="main_menu_language">
				<label for="language-select">{state.t("language-selector")}</label>
				<select name="language" id="language-select" on:change=move |e| {
					let target = e.target().expect("on self");
					let select = target.dyn_ref::<HtmlSelectElement>().expect("is self");
					let selected = select.selected_options().get_with_index(0).expect("has selected option");
					let selected = selected.dyn_ref::<HtmlOptionElement>().expect("is option");
					let lang = selected.value().parse().expect("not valid language identifier");
					if Translator::get_languages().contains(&lang) {
						language.set(lang);
					} else {
						log::warn!("Language {} is not supported", lang);
					}
				}>
					 {move || {
						 let current_language=language.get();
						 Translator::get_languages().into_iter().map(|lang| {
							 let lang_str = lang.to_string();
							 view! { cx,
								 <option value={&lang_str} selected={current_language==lang}>{lang_str}</option>
							 }
						 }).collect_view(cx)
					 }}
				</select>
			</div>
		</div>
	}
}

#[component]
fn Game(cx: Scope, state: StateRc) -> impl IntoView {
	let State {
		translations,
		active_story,
		..
	} = *state;
	view! { cx,
		<div id="game">
			<div id="game_top_bar">TopBarHere</div>
			<div id="game_body">
				{move || {
					if let Some(story) = active_story.get() {
						let (next_page, story_view) = story.get_page(cx, translations);
						view! { cx,
							<div id="game_body_story">
								{story_view}
								{if next_page.is_some() {
									view! { cx,
										<div id="story_buttons" class="story_buttons_more">
											<button on:click=move |_ev| active_story.set(next_page)>Continue</button>
											<button on:click=move |_ev| active_story.set(None)>Skip</button>
										</div>
									}
								} else {
									view! { cx,
										<div id="story_buttons" class="story_buttons_more">
											<button on:click=move |_ev| active_story.set(None)>Ok</button>
										</div>
									}
								}}
							</div>
						}
					} else {
						view! { cx, <div>"hello world?"</div> }
					}
				}}
			</div>
			<div id="game_bottom_bar">BottomBarHere</div>
		</div>
	}
}

#[component]
fn App(cx: Scope) -> impl IntoView {
	let state = StateRc::new(State::new(cx));
	view! { cx,
		<div id="rsingularity">
			{move || match state.difficulty.get() {
				None => view! { cx, <MainMenu state=state.clone() /> },
				Some(_) => view! { cx, <Game state=state.clone() /> },
			}}
		</div>
	}
}

fn main() {
	// #[cfg(feature = "dev")]
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	console_log::init_with_level(Level::Trace).unwrap();

	mount_to_body(|cx| view! { cx, <App /> })
}
