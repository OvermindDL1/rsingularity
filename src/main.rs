use leptos::html::Textarea;
use leptos::*;
use log::Level;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlOptionElement, HtmlSelectElement};

use crate::difficulty::Difficulty;
use crate::state::{State, StateRc};
use crate::theme::Theme;
use crate::translations::Translator;

pub mod assets;
pub mod danger;
pub mod difficulty;
pub mod effects;
pub mod groups;
pub mod state;
pub mod story;
pub mod technology;
pub mod theme;
pub mod translations;

#[component]
fn MainMenu(state: StateRc) -> impl IntoView {
	let save_state = state.clone();
	let load_state = state.clone();
	let State {
		language, difficulty, ..
	} = *state;
	let load_text_ref = create_node_ref::<Textarea>();
	view! {
		<div id="main_menu">
			<div id="main_menu_title">{state.t("title")}</div>
			<div id="main_menu_difficulty">
				<div id="main_menu_difficulty_title">{state.t("difficulty.title")}</div>
				<button id="main_menu_new_very_easy_button" on:click=move |_ev| difficulty.set(Some(Difficulty::VeryEasy))>
					{state.t1("difficulty", ("level", Difficulty::VeryEasy.into()))}
				</button>
				<button id="main_menu_new_easy_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Easy))>
					{state.t1("difficulty", ("level", Difficulty::Easy.into()))}
				</button>
				<button id="main_menu_new_normal_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Normal))>
					{state.t1("difficulty", ("level", Difficulty::Normal.into()))}
				</button>
				<button id="main_menu_new_hard_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Hard))>
					{state.t1("difficulty", ("level", Difficulty::Hard.into()))}
				</button>
				<button
					id="main_menu_new_ultra_hard_button"
					on:click=move |_ev| difficulty.set(Some(Difficulty::UltraHard))
				>
					{state.t1("difficulty", ("level", Difficulty::UltraHard.into()))}
				</button>
				<button
					id="main_menu_new_impossible_button"
					on:click=move |_ev| difficulty.set(Some(Difficulty::Impossible))
				>
					{state.t1("difficulty", ("level", Difficulty::Impossible.into()))}
				</button>
			</div>
			<div id="main_menu_load">
				<div id="main_menu_load_title">{state.t("load-game")}</div>
				<textarea id="main_menu_load_textarea" placeholder=state.t("load-game.placeholder") node_ref=load_text_ref>
					{move || {
						let save_state = save_state.save();
						let saved = postcard::to_allocvec(&save_state).expect("save state serializes");
						base2048::encode(&saved)
					}}

				</textarea>
				<button
					id="main_menu_load_button"
					on:click=move |_ev| {
						let save_state = load_text_ref.get().expect("has node").value();
						let save_state = match base2048::decode(&save_state) {
							Some(save_state) => save_state,
							None => {
								load_text_ref
									.get()
									.expect("has node")
									.set_value("Decoding Error: Save state is not base2048");
								return;
							}
						};
						let save_state = match postcard::from_bytes(&save_state) {
							Ok(save_state) => save_state,
							Err(e) => {
								load_text_ref.get().expect("has node").set_value(&format!("Parsing Error: {}", e));
								return;
							}
						};
						if let Err(e) = load_state.load(save_state) {
							load_text_ref.get().expect("has node").set_value(&format!("Loading Error: {}", e));
							return;
						}
					}
				>

					{state.t("load-game.load")}
				</button>
			</div>
			<div id="main_menu_language">
				<label for="language-select">{state.t("language-selector")}</label>
				<select
					name="language"
					id="language-select"
					on:change=move |e| {
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
					}
				>

					{move || {
						let current_language = language.get();
						Translator::get_languages()
							.into_iter()
							.map(|lang| {
								let lang_str = lang.to_string();
								view! {
									<option value=&lang_str selected=current_language == lang>
										{lang_str}
									</option>
								}
							})
							.collect_view()
					}}

				</select>
			</div>
		</div>
	}
}

#[component]
fn Game(state: StateRc) -> impl IntoView {
	let State {
		translations,
		active_story,
		..
	} = *state;
	view! {
		<div id="game">
			<div id="game_top_bar">
				TopBarHere
			</div>
			<div id="game_body">
				{move || {
					if let Some(story) = active_story.get() {
						let (next_page, story_view) = story.get_page(translations);
						view! {
							<div id="game_body_story">
								{story_view}
								{if next_page.is_some() {
									view! {
										<div id="story_buttons" class="story_buttons_more">
											<button on:click=move |_ev| active_story.set(next_page)>
												Continue
											</button>
											<button on:click=move |_ev| active_story.set(None)>
												Skip
											</button>
										</div>
									}
								} else {
									view! {
										<div id="story_buttons" class="story_buttons_more">
											<button on:click=move |_ev| active_story.set(None)>
												Ok
											</button>
										</div>
									}
								}}

							</div>
						}
					} else {
						view! { <div>"hello world?"</div> }
					}
				}}

			</div>
			<div id="game_bottom_bar">
				BottomBarHere
			</div>
		</div>
	}
}

#[component]
fn App() -> impl IntoView {
	let state = StateRc::new(State::new());
	let theme = state.theme;
	view! {
		<div id="rsingularity" class=move || theme.with(Theme::css_class)>
			{move || match state.difficulty.get() {
				None => view! { <MainMenu state=state.clone()/> },
				Some(_) => view! { <Game state=state.clone()/> },
			}}

		</div>
	}
}

fn main() {
	// #[cfg(feature = "dev")]
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	console_log::init_with_level(Level::Trace).unwrap();

	mount_to_body(|| view! { <App/> })
}
