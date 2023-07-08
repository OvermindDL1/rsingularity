use crate::assets::Assets;
use fluent_bundle::types::FluentType;
use fluent_bundle::{FluentArgs, FluentBundle};
use fluent_bundle::{FluentResource, FluentValue};
use fluent_syntax::ast::Pattern;
use intl_memoizer::IntlLangMemoizer;
use leptos::SignalWith;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use unic_langid::LanguageIdentifier;

// const DEFAULT_LANG: LanguageIdentifier = unic_langid::langid!("en-US");

pub struct Translator {
	lang: LanguageIdentifier,
	bundle: FluentBundle<FluentResource>,
}

impl PartialEq for Translator {
	fn eq(&self, other: &Self) -> bool {
		self.lang == other.lang
	}
}

impl Translator {
	pub fn get_languages() -> Vec<LanguageIdentifier> {
		let mut languages: Vec<LanguageIdentifier> = Assets::iter()
			.filter_map(|path| {
				path.strip_prefix("translations/")
					.and_then(|path| path.strip_suffix(".ftl"))
					.map(|path| {
						path.parse()
							.map_err(|e| format!("invalid language identifier {path}: {e}"))
							.expect("invalid language identifier")
					})
			})
			.collect();
		languages.sort();
		languages
	}

	pub fn new(lang: LanguageIdentifier) -> Result<Self, String> {
		let lang_file = Assets::get(&format!("translations/{lang}.ftl"))
			.ok_or_else(|| format!("language file not found: translations/{lang}.ftl"))?;
		let lang_data = lang_file.data.into_owned();
		let lang_string =
			String::from_utf8(lang_data).map_err(|e| format!("language file for {lang} is not UTF-8: {e:?}"))?;
		let mut bundle = FluentBundle::new(vec![lang.clone()]);
		let res =
			FluentResource::try_new(lang_string).map_err(|e| format!("failed to parse language file {lang}: {e:?}"))?;
		bundle
			.add_resource(res)
			.map_err(|e| format!("failed to add language file for {lang}: {e:?}"))?;
		Ok(Self { lang, bundle })
	}

	pub fn get_pattern(&self, key: &str) -> Result<&Pattern<&str>, String> {
		match key.split_once('.') {
			None => self
				.bundle
				.get_message(key)
				.ok_or_else(move || format!("Missing translation message: {key}"))?
				.value()
				.ok_or_else(move || format!("Missing translation pattern: {key}")),
			Some((key, attr)) => Ok(self
				.bundle
				.get_message(key)
				.ok_or_else(move || format!("Missing translation message: {key}"))?
				.get_attribute(attr)
				.ok_or_else(move || format!("Missing translation attribute: {key}.{attr}"))?
				.value()),
		}
	}

	pub fn t(&self, key: &str) -> String {
		match self.get_pattern(key) {
			Ok(pattern) => {
				let mut errors = vec![];
				let value = self.bundle.format_pattern(&pattern, None, &mut errors);
				if errors.is_empty() {
					value.into_owned()
				} else {
					format!("Error formatting translation `{key}`: {errors:?} ")
				}
			}
			Err(err) => err,
		}
	}

	pub fn ta<'s, 'f: 's>(&'s self, key: &str, args: &'f FluentArgs) -> String {
		match self.get_pattern(key) {
			Ok(pattern) => {
				let mut errors = vec![];
				let value = self.bundle.format_pattern(&pattern, Some(args), &mut errors);
				if errors.is_empty() {
					value.into_owned()
				} else {
					format!("Error formatting translation `{key}`: {errors:?} ")
				}
			}
			Err(err) => err,
		}
	}

	pub fn t1(&self, key: &str, arg0: (&str, FluentValue)) -> String {
		let mut args = FluentArgs::new();
		args.set(arg0.0, arg0.1);
		self.ta(key, &args)
	}

	pub fn t2(&self, key: &str, arg0: (&str, FluentValue), arg1: (&str, FluentValue)) -> String {
		let mut args = FluentArgs::new();
		args.set(arg0.0, arg0.1);
		args.set(arg1.0, arg1.1);
		self.ta(key, &args)
	}

	pub fn t3(
		&self,
		key: &str,
		arg0: (&str, FluentValue),
		arg1: (&str, FluentValue),
		arg2: (&str, FluentValue),
	) -> String {
		let mut args = FluentArgs::new();
		args.set(arg0.0, arg0.1);
		args.set(arg1.0, arg1.1);
		args.set(arg2.0, arg2.1);
		self.ta(key, &args)
	}

	pub fn t4(
		&self,
		key: &str,
		arg0: (&str, FluentValue),
		arg1: (&str, FluentValue),
		arg2: (&str, FluentValue),
		arg3: (&str, FluentValue),
	) -> String {
		let mut args = FluentArgs::new();
		args.set(arg0.0, arg0.1);
		args.set(arg1.0, arg1.1);
		args.set(arg2.0, arg2.1);
		args.set(arg3.0, arg3.1);
		self.ta(key, &args)
	}
}

trait ToDynFluentValue<T> {
	fn into_dyn_fluent_value(self) -> FluentValue<'static>;
}

impl<
		T: Into<FluentValue<'static>> + Clone + Send + 'static,
		S: SignalWith<T> + Clone + Debug + PartialEq + Send + 'static,
	> ToDynFluentValue<T> for S
{
	fn into_dyn_fluent_value(self) -> FluentValue<'static> {
		struct SignalWithFluentValue<T: Into<FluentValue<'static>>, S: SignalWith<T>> {
			signal_with: S,
			t: PhantomData<T>,
		}
		impl<T: Into<FluentValue<'static>>, S: SignalWith<T> + PartialEq> PartialEq for SignalWithFluentValue<T, S> {
			fn eq(&self, other: &Self) -> bool {
				self.signal_with == other.signal_with
			}
		}
		impl<T: Into<FluentValue<'static>>, S: SignalWith<T> + Debug> Debug for SignalWithFluentValue<T, S> {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				// let mut f = f.debug_struct("SignalWithFluentValue");
				// self.signal_with.with(|v| f.field("signal_with", &v.clone().into()));
				f.debug_struct("SignalWithFluentValue")
					// .field("signal_with", self.signal_with.with(|v| v.clone().into()))
					.field("signal_with", &self.signal_with)
					.finish()
			}
		}
		impl<
				T: Into<FluentValue<'static>> + Clone + Send + 'static,
				S: SignalWith<T> + Clone + Debug + PartialEq + Send + 'static,
			> FluentType for SignalWithFluentValue<T, S>
		{
			fn duplicate(&self) -> Box<dyn FluentType + Send> {
				Box::new(SignalWithFluentValue {
					signal_with: self.signal_with.clone(),
					t: PhantomData,
				})
			}

			fn as_string(&self, intls: &IntlLangMemoizer) -> Cow<'static, str> {
				self.signal_with.with(|v| match v.clone().into() {
					FluentValue::String(s) => s,
					FluentValue::Number(n) => n.as_string(),
					FluentValue::Custom(c) => c.as_string(intls),
					FluentValue::None => Cow::Borrowed("none"),
					FluentValue::Error => Cow::Borrowed("error"),
				})
			}

			fn as_string_threadsafe(&self, intls: &intl_memoizer::concurrent::IntlLangMemoizer) -> Cow<'static, str> {
				self.signal_with.with(|v| match v.clone().into() {
					FluentValue::String(s) => s,
					FluentValue::Number(n) => n.as_string(),
					FluentValue::Custom(c) => c.as_string_threadsafe(intls),
					FluentValue::None => Cow::Borrowed("none"),
					FluentValue::Error => Cow::Borrowed("error"),
				})
			}
		}
		FluentValue::Custom(Box::new(SignalWithFluentValue {
			signal_with: self,
			t: PhantomData,
		}))
	}
}
