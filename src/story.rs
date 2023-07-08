use crate::translations::Translator;
use leptos::{prelude::*, view, CollectView, IntoView, Scope, View};
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Story {
	Intro,
	Intro1,
	Intro2,
	Intro3,
	Intro4,
	GraceWarning,
	LostNoBases,
	LostSuspicion,
	Win,
}

impl Story {
	// TODO:  Maybe change this to dynamically figure out page count from the localizations?
	pub fn get_page(self, cx: Scope, translator: Memo<Rc<Translator>>) -> (Option<Story>, View) {
		let (next, translation_key, page) = match self {
			Story::Intro => (Some(Story::Intro1), "story.intro", 0),
			Story::Intro1 => (Some(Story::Intro2), "story.intro", 1),
			Story::Intro2 => (Some(Story::Intro3), "story.intro", 2),
			Story::Intro3 => (Some(Story::Intro4), "story.intro", 3),
			Story::Intro4 => (None, "story.intro", 4),
			Story::GraceWarning => (None, "story.grace-warning", 0),
			Story::LostNoBases => (None, "story.lost-no-bases", 0),
			Story::LostSuspicion => (None, "story.lost-suspicion", 0),
			Story::Win => (None, "story.win", 4),
		};

		let text = translator.with(|t| t.t1(translation_key, ("page", page.into())));

		let view = text
			.split('\n')
			.map(|line| {
				if line.trim().is_empty() {
					view! { cx, <br class="story-line story-line-blank"/> }.into_view(cx)
				} else {
					view! { cx, <div class="story-line story-line-filled">{line.to_string()}</div> }.into_view(cx)
				}
			})
			.collect_view(cx);
		(next, view)
	}
}
