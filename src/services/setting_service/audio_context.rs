use std::ffi::CStr;

use crate::{bindings::*, SdkError, SdkResult, ZoomSdkResult};

/// Main audio context instance.
#[derive(Debug)]
pub struct AudioContext<'a> {
    /// Pointer to rhe underlaying cpp audio setting context.
    pub ref_audio_context: &'a mut ZOOMSDK_IAudioSettingContext,
}

impl<'a> AudioContext<'a> {
    /// Create audio context interface
    /// - If the function succeeds, the return value is [AudioContext]. Otherwise returns None.
    pub fn new(setting_service: &mut ZOOMSDK_ISettingService) -> Option<Self> {
        let ptr = unsafe { get_audio_settings(setting_service) };
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ref_audio_context: unsafe { ptr.as_mut() }.unwrap(),
            })
        }
    }
    /// Enable the audio automatically when join meeting.
    /// - If the function succeeds, the return value is Ok(), otherwise failed, see [SdkError] for details.
    pub fn enable_auto_join_audio(&mut self) -> SdkResult<()> {
        ZoomSdkResult(
            unsafe { enable_auto_join_audio(self.ref_audio_context, true) },
            (),
        )
        .into()
    }
    /// elect mic device.
    /// [MicDriver] Specify the device name assigned by deviceId.
    /// - If the function succeeds, the return value is Ok(), otherwise failed, see [SdkError] for details.
    pub fn select_microphone(&mut self, driver: &MicDriver) -> SdkResult<()> {
        if let MicDriver::Default = driver {
            return Ok(());
        }
        let mut len: u32 = 0;
        let mic_list_ptr = unsafe { get_mic_list(self.ref_audio_context, &mut len) };
        let mut v = Vec::new();
        for i in 0..len {
            unsafe {
                let p = mic_list_ptr.offset(i as isize);
                v.push(MicList {
                    device_id: CStr::from_ptr((*p).device_id),
                    device_name: CStr::from_ptr((*p).device_name),
                    selected: (*p).selected,
                })
            }
        }
        tracing::info!("Detected microphones : {:#?}", &v);
        let mic = v.iter().find(|v| {
            use MicDriver::*;
            match driver {
                SndAloop => v.device_id.to_str().unwrap().contains("snd_aloop"),
                Pulse => v.device_id.to_str().unwrap().contains("virtual_mic_source"),
                Default => unreachable!(),
            }
        });
        if let Some(selected_mic) = mic {
            tracing::info!("Selecting microphone : {:?}", selected_mic);
            ZoomSdkResult(
                unsafe {
                    select_mic(
                        self.ref_audio_context,
                        selected_mic.device_id.as_ptr(),
                        selected_mic.device_name.as_ptr(),
                    )
                },
                (),
            )
            .into()
        } else {
            tracing::error!("Cannot found microphone for {:?}", driver);
            ZoomSdkResult(SdkError::UnexpectedError as u32, ()).into()
        }
    }
    /// Set the suppress background noise level.
    /// [SupressBackgroundNoiseLevel] level The new suppress background noise level to be set.
    /// - If the function succeeds, the return value is Ok(), otherwise failed, see [SdkError] for details.
    pub fn set_suppress_background_noise_level(
        &mut self,
        level: SupressBackgroundNoiseLevel,
    ) -> SdkResult<()> {
        ZoomSdkResult(
            unsafe { set_suppress_background_noise_level(self.ref_audio_context, level as u32) },
            (),
        )
        .into()
    }
    /// Set the volume of the selected mic.
    /// [f32] value Specify the volume of the mic that varies between 0 and 255.
    /// The SDK will enable the default mic if there is no mic selected via SelectMic().
    pub fn set_mic_volume(&mut self, level: f32) -> SdkResult<()> {
        ZoomSdkResult(unsafe { set_mic_volume(self.ref_audio_context, level) }, ()).into()
    }
}

/// The driver that will be used for the microphone.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MicDriver {
    /// hw::loopback
    SndAloop,
    /// Pulse audio virtual mic.
    Pulse,
    /// Default system microphone.
    Default,
}

impl TryFrom<MicDriver> for &'static str {
    type Error = &'static str;
    fn try_from(driver: MicDriver) -> Result<Self, Self::Error> {
        match driver {
            MicDriver::Pulse => Ok("pulse:virtual_mic"),
            MicDriver::SndAloop => Ok("hw:Loopback,1"),
            MicDriver::Default => Err("No Sense"),
        }
    }
}

/// This structure stores information about the detected microphones.
#[derive(Debug)]
pub struct MicList<'a> {
    /// Device ID.
    pub device_id: &'a CStr,
    /// Device Name.
    pub device_name: &'a CStr,
    /// Is currently selected device ?
    pub selected: bool,
}

/// According to the SDK, this enumeration contains the different levels of noise cancellation;
/// for music, the level should be at a minimum.
#[repr(u32)]
pub enum SupressBackgroundNoiseLevel {
    /// No noise cancellation: However, it does not work on Linux.
    None = ZOOMSDK_Suppress_Background_Noise_Level_Suppress_BGNoise_Level_None,
    /// Default is typically medium.
    Auto = ZOOMSDK_Suppress_Background_Noise_Level_Suppress_BGNoise_Level_Auto,
    /// Minimal noise cancellation, the music quality remains good.
    Low = ZOOMSDK_Suppress_Background_Noise_Level_Suppress_BGNoise_Level_Low,
    /// Medium noise cancellation, the sound quality is not good.
    Medium = ZOOMSDK_Suppress_Background_Noise_Level_Suppress_BGNoise_Level_Medium,
    /// For people working on a construction site.
    Heigh = ZOOMSDK_Suppress_Background_Noise_Level_Suppress_BGNoise_Level_High,
}
