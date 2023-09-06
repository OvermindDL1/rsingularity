use crate::assets::Assets;
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
#[cfg(feature = "embed-music")]
pub mod mixer;
pub mod state;
pub mod story;
pub mod technology;
pub mod theme;
pub mod translations;

#[component]
fn ClickyButton<C: Fn() + 'static, Name: Fn() -> String + 'static>(
	state: StateRc,
	#[prop(into)] id: String,
	on_click: C,
	name: Name,
) -> impl IntoView {
	let play_click = state.play_click();
	view! {
		<button
			id=id
			on:click=move |_ev| {
				play_click();
				on_click();
			}
		>

			{name}
		</button>
	}
}

#[component]
fn MainMenu(state: StateRc) -> impl IntoView {
	let save_state = state.clone();
	let load_state = state.clone();
	let State {
		language, difficulty, ..
	} = *state;
	let load_text_ref = create_node_ref::<Textarea>();
	view! {
		<div id="main-menu">
			<div id="main-menu-title">{state.t("title")}</div>
			<div id="main-menu-difficulty">
				<div id="main-menu-difficulty-title">{state.t("difficulty.title")}</div>
				<ClickyButton
					state=state.clone()
					id="main-menu-new-very-easy-button"
					on_click=move || difficulty.set(Some(Difficulty::VeryEasy))
					name=state.t1("difficulty", ("level", Difficulty::VeryEasy.into()))
				/>
				<ClickyButton
					state=state.clone()
					id="main-menu-new-easy-button"
					on_click=move || difficulty.set(Some(Difficulty::Easy))
					name=state.t1("difficulty", ("level", Difficulty::Easy.into()))
				/>
				<ClickyButton
					state=state.clone()
					id="main-menu-new-normal-button"
					on_click=move || difficulty.set(Some(Difficulty::Normal))
					name=state.t1("difficulty", ("level", Difficulty::Normal.into()))
				/>
				<ClickyButton
					state=state.clone()
					id="main-menu-new-hard-button"
					on_click=move || difficulty.set(Some(Difficulty::Hard))
					name=state.t1("difficulty", ("level", Difficulty::Hard.into()))
				/>
				<ClickyButton
					state=state.clone()
					id="main-menu-new-ultra-hard-button"
					on_click=move || difficulty.set(Some(Difficulty::UltraHard))
					name=state.t1("difficulty", ("level", Difficulty::UltraHard.into()))
				/>
				<ClickyButton
					state=state.clone()
					id="main-menu-new-impossible-button"
					on_click=move || difficulty.set(Some(Difficulty::Impossible))
					name=state.t1("difficulty", ("level", Difficulty::Impossible.into()))
				/>
			</div>
			<div id="main-menu-load">
				<div id="main-menu-load-title">{state.t("load-game")}</div>
				<textarea id="main-menu-load-textarea" placeholder=state.t("load-game.placeholder") node_ref=load_text_ref>
					{move || {
						let save_state = save_state.save();
						let saved = postcard::to_allocvec(&save_state).expect("save state serializes");
						base2048::encode(&saved)
					}}

				</textarea>
				<ClickyButton
					state=state.clone()
					id="main-menu-load-button"
					on_click=move || {
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
						leptos::log!("Loaded save successfully");
					}

					name=state.t("load-game.load")
				/>
			</div>
			<div id="main-menu-language">
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
			<div id="game-top-bar">
				TopBarHere
			</div>
			<div id="game-body">
				{move || {
					if let Some(story) = active_story.get() {
						let (next_page, story_view) = story.get_page(translations);
						view! {
							<div id="game-body-story">
								{story_view}
								{if next_page.is_some() {
									view! {
										<div id="story-buttons" class="story_buttons_more">
											<ClickyButton
												state=state.clone()
												id="story-button-continue"
												on_click=move || active_story.set(next_page)
												name=state.t("story-buttons.continue")
											/>
											<ClickyButton
												state=state.clone()
												id="story-button-skip"
												on_click=move || active_story.set(None)
												name=state.t("story-buttons.skip")
											/>
										</div>
									}
								} else {
									view! {
										<div id="story-buttons" class="story_buttons_more">
											<ClickyButton
												state=state.clone()
												id="story-button-ok"
												on_click=move || active_story.set(None)
												name=state.t("story-buttons.ok")
											/>
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
			<div id="game-bottom-bar">
				BottomBarHere
			</div>
		</div>
	}
}

#[component]
fn App() -> impl IntoView {
	let state = StateRc::new(State::new());
	let theme = state.theme;
	let click_ref = state.click_ref;
	let music_ref = state.music_ref;
	// let click_data = Assets::get_as_object_uri_or_path("sounds/click0.wav");
	// dbg!(click_data);
	view! {
		<audio
			node_ref=click_ref
			id="click"
			src=Assets::get_as_data_uri("sounds/click0.wav").expect("click sound wave file should exist")
		></audio>
		<audio node_ref=music_ref id="music" autoplay loop></audio>
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
