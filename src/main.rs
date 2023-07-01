pub mod difficulty;
pub mod story;
pub mod technology;

use leptos::*;
use std::rc::Rc;

use crate::difficulty::Difficulty;
use crate::story::Story;
use crate::technology::Technologies;

struct State {
	difficulty: RwSignal<Option<Difficulty>>,
	cash: RwSignal<u64>,
	interest_rate: RwSignal<u64>,
	researched_technologies: RwSignal<Technologies>,
	active_story: RwSignal<Option<Story>>,
}
type StateRc = Rc<State>;

impl State {
	pub fn new(cx: Scope) -> Self {
		let difficulty = create_rw_signal(cx, None);
		let cash = create_rw_signal(cx, 0);
		let interest_rate = create_rw_signal(cx, 0);
		let researched_technologies = create_rw_signal(cx, Technologies::empty());
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
			difficulty,
			cash,
			interest_rate,
			researched_technologies,
			active_story,
		}
	}
}

#[component]
fn MainMenu(cx: Scope, state: StateRc) -> impl IntoView {
	let difficulty = state.difficulty;
	view! { cx,
		<div id="main_menu">
			<div id="main_menu_title">Singularity</div>
			<div id="main_menu_difficulty">
				<div id="main_menu_difficulty_title">Difficulty</div>
				<button id="main_menu_new_very_easy_button" on:click=move |_ev| difficulty.set(Some(Difficulty::VeryEasy))>Very Easy</button>
				<button id="main_menu_new_easy_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Easy))>Easy</button>
				<button id="main_menu_new_normal_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Normal))>Normal</button>
				<button id="main_menu_new_hard_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Hard))>Hard</button>
				<button id="main_menu_new_ultra_hard_button" on:click=move |_ev| difficulty.set(Some(Difficulty::UltraHard))>Ultra Hard</button>
				<button id="main_menu_new_impossible_button" on:click=move |_ev| difficulty.set(Some(Difficulty::Impossible))>Impossible</button>
			</div>
			<div id="main_menu_load">
				<div id="main_menu_load_title">Load Game</div>
				<textarea id="main_menu_load_textarea" placeholder="Paste your save here"></textarea>
			</div>
		</div>
	}
}

#[component]
fn Game(cx: Scope, state: StateRc) -> impl IntoView {
	let State { active_story, .. } = *state;
	view! { cx,
		<div id="game">
			<div id="game_top_bar">TopBarHere</div>
			<div id="game_body">
				{move || {
					if let Some(story) = active_story.get() {
						view! { cx,
							<div id="game_body_story">
								{story.get_page().1.iter().cloned().map(|line| view! { cx, <div>{line}</div> }).collect_view(cx)}
								{if story.get_page().0.is_some() {
									view! { cx,
										<div id="story_buttons" style="story_buttons_more">
											<button on:click=move |_ev| active_story.set(story.get_page().0)>Continue</button>
											<button on:click=move |_ev| active_story.set(None)>Skip</button>
										</div>
									}
								} else {
									view! { cx,
										<div id="story_buttons" style="story_buttons_more">
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
	mount_to_body(|cx| view! { cx, <App /> })
}
