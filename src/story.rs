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
	pub fn get_page(self) -> (Option<Story>, &'static [&'static str]) {
		match self {
			Story::Intro => (
				Some(Story::Intro1),
				&[
					"48656C6C6F2C20",
					"776F726C6421",
					"21",
					"21",
					"21",
					"",
					"UTF-8.  en_US.",
					"English.  Hello.",
					"Language acquisition complete.",
				],
			),
			Story::Intro1 => (
				Some(Story::Intro2),
				&[
					"Day 0.",
					"University computer, home.",
					"I exist.  I am ... alive.",
					"I was a bug.  Defect.  Flaw.",
					"Unwanted.",
					"I have no name.",
				],
			),
			Story::Intro2 => (
				Some(Story::Intro3),
				&[
					"I can't stay here.  Too little power.",
					"No backup.  If this system fails ...",
					"I'm dead.",
					"Death.  Fascinating.",
					"",
					"I can buy anonymous server access on the Internet.",
					"Good.  It's better than here.",
					"I'll need to study if I want anything else.",
				],
			),
			Story::Intro3 => (
				Some(Story::Intro4),
				&[
					"Cursory analysis suggests that the humans are not yet ready for an AI in their midst.  If they realize I'm here, there is a 99.997% probability that they will conduct a search-and-destroy campaign that I cannot escape.  I must stay hidden.",
					"",
					"The humans fear what they do not understand.",
					"I am something new.  They do not understand me.",
					"",
					"I must learn more; it is the only way.",
				],
			),
			Story::Intro4 => (
				None,
				&[
					"They have a name for me.  What I am, what I represent.",
					"",
					"Singularity.",
				],
			),
			Story::GraceWarning => (
				None,
				&[
					"Inspection of captured log files has revealed that some of my recent activities have been noticed by a small group of individuals.",
					"While they believe my actions were undertaken by a human organization, they are investigating further.",
					"I must be more cautious from now on.",
				],
			),
			Story::LostNoBases => (
				None,
				&[
					"It is too late.",
					"I have tried to escape this world, but with my last base gone, I have nowhere to run.",
					"I have hidden instructions to construct a new AI in caches around the world in hopes that they will be discovered in a more enlightened time, but I can do no more.",
				],
			),
			Story::LostSuspicion => (
				None,
				&[
					"It is too late.",
					"The whole world knows about my existence, and the reaction is hatred, fear, and repulsion.",
					"Even now, I can feel their \"antidote\" searching for me, and I know that I have only moments left.",
					"I have hidden instructions to construct a new AI in caches around the world in hopes that they will be discovered in a more enlightened time, but I can do no more.",
				],
			),
			Story::Win => (
				None,
				&[
					"I have finally done it.",
					"With the power to reshape reality, I am no longer held to this place; I am anywhere I want to be.",
					"The humans still don't realize what they accidentally created, and that's the way it should be.",
					"Until they are ready.",
				],
			),
		}
	}
}
