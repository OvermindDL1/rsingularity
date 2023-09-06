use rodio::dynamic_mixer::{DynamicMixer, DynamicMixerController};
use rodio::{dynamic_mixer, OutputStream, OutputStreamHandle, Sink};
use std::cell::{OnceCell, RefCell};
use std::sync::Arc;

static MIXER: OnceCell<RefCell<Mixer>> = OnceCell::new();

pub enum Mixer {
	Off,
	On {
		device: OutputStream,
		device_stream: OutputStreamHandle,
		mixer_controller: Arc<DynamicMixerController<f32>>,
		mixer_processor: DynamicMixer<f32>,
		sink: Sink,
	},
}

impl Mixer {
	pub fn get() -> &'static Mixer {
		MIXER.get_or_init(|| Mixer::new(false))
	}

	fn new(enabled: bool) -> Mixer {
		if !enabled {
			Mixer::Off
		} else {
			match Mixer::create_device() {
				Ok(mixer @ Mixer::On { .. }) => mixer,
				Ok(_mixer @ Mixer::Off) => {
					unreachable!(
						"Mixer::create_device() returned Mixer::Off which is not Mixer::On yet did not return an error"
					);
				}
				Err(e) => {
					leptos::warn!("Could not enable audio device, disabling audio: {e}");
					Mixer::Off
				}
			}
		}
	}

	fn create_device() -> Result<Mixer, String> {
		let (device, device_stream) = OutputStream::try_default().map_err(|e| e.to_string())?;
		let (mixer_controller, mixer_processor) = dynamic_mixer::mixer::<f32>(1, 44_100);
		let sink = Sink::try_new(&device_stream).map_err(|e| e.to_string())?;
		Ok(Mixer::On {
			device,
			device_stream,
			mixer_controller,
			mixer_processor,
			sink,
		})
	}

	pub fn disable_device(&mut self) {
		*self = Mixer::Off;
	}

	pub fn enable_device(&mut self) {
		*self = Mixer::new(true);
	}

	pub fn play_once() {
		if let Mixer::On { mixer_controller, .. } = Mixer::get() {
			// mixer_controller.
			// sink.append()
		}
	}
}

// pub fn play_once(asset_path: &Path) -> Result<SoundStreamHandle, String> {
// 	let host = cpal::default_host();
// 	let device = host.default_output_device().ok_or("no output device available")?;
// 	let config = device.default_output_config().map_err(|e| e.to_string())?;
// 	let stream = match config.sample_format() {
// 		SampleFormat::F32 => play_once_run::<f32>(&device, &config.into(), asset_path)?,
// 		SampleFormat::I16 => play_once_run::<i16>(&device, &config.into(), asset_path)?,
// 		SampleFormat::U16 => play_once_run::<u16>(&device, &config.into(), asset_path)?,
// 		SampleFormat::I8 => play_once_run::<i8>(&device, &config.into(), asset_path)?,
// 		SampleFormat::I32 => play_once_run::<i32>(&device, &config.into(), asset_path)?,
// 		SampleFormat::I64 => play_once_run::<i64>(&device, &config.into(), asset_path)?,
// 		SampleFormat::U8 => play_once_run::<u8>(&device, &config.into(), asset_path)?,
// 		SampleFormat::U32 => play_once_run::<u32>(&device, &config.into(), asset_path)?,
// 		SampleFormat::U64 => play_once_run::<u64>(&device, &config.into(), asset_path)?,
// 		SampleFormat::F64 => play_once_run::<f64>(&device, &config.into(), asset_path)?,
// 		f => return Err(format!("unsupported sample format {f:?}")),
// 	};
// 	Ok(stream)
// }
//
// fn play_once_run<T: Sample + SizedSample + FromSample<f32>>(
// 	device: &Device,
// 	config: &StreamConfig,
// 	asset_path: &Path,
// ) -> Result<Stream, String> {
// 	let sample_rate = config.sample_rate.0 as f32;
// 	let channels = config.channels as usize;
//
// 	// Produce a sinusoid of maximum amplitude.
// 	let mut sample_clock = 0f32;
// 	let mut next_value = move || {
// 		sample_clock = (sample_clock + 1.0) % sample_rate;
// 		(sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
// 	};
//
// 	let err_fn = |err| leptos::error!("an error occurred on stream: {err}");
//
// 	let stream = device
// 		.build_output_stream(
// 			config,
// 			move |data: &mut [T], _| write_data(data, channels, &mut next_value),
// 			err_fn,
// 			Some(Duration::from_secs(1)),
// 		)
// 		.unwrap();
// 	stream.play().map_err(|e| e.to_string())?;
// 	Ok(stream)
// }
//
// fn write_data<T: Sample + FromSample<f32>>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32) {
// 	for frame in output.chunks_mut(channels) {
// 		let value: T = T::from_sample(next_sample());
// 		for sample in frame.iter_mut() {
// 			*sample = value;
// 		}
// 	}
// }
