#ifndef _C_SETTING_SERVICE_INTERFACE_H_
#define _C_SETTING_SERVICE_INTERFACE_H_

#include "../zoom-meeting-sdk-linux/h/setting_service_interface.h"

/// \brief Get audio setting interface.
/// \return If the function succeeds, the return value an object pointer to IAudioSettingContext.
/// Otherwise failed, returns NULL.
/// For more details, see \link IAudioSettingContext \endlink.
extern "C" ZOOMSDK::IAudioSettingContext* get_audio_settings(ZOOMSDK::ISettingService *setting_service);

/// \brief Enable or disable the audio automatically when join meeting.
/// \param bEnable TRUE indicates to enable the audio automatically when join meeting.
/// \return If the function succeeds, the return value is SDKErr_Success.
///Otherwise failed. To get extended error information, see \link SDKError \endlink enum.
extern "C" ZOOMSDK::SDKError enable_auto_join_audio(ZOOMSDK::IAudioSettingContext* audio_setting, bool value);

struct mic_list {
    const zchar_t *device_id;
    const zchar_t *device_name;
    bool selected;
};

/// \brief Get the mic device list.
/// \return If the function succeeds, the return value is the camera device list.
/// Otherwise failed, returns NULL.
extern "C" struct mic_list* get_mic_list(ZOOMSDK::IAudioSettingContext* audio_setting, unsigned int *len);

/// \brief Select mic device.
/// \param deviceId Specify the device to be selected.
/// \param deviceName Specify the device name assigned by deviceId.
/// \return If the function succeeds, the return value is SDKErr_Success.
/// Otherwise failed. To get extended error information, see \link SDKError \endlink enum.
extern "C" ZOOMSDK::SDKError select_mic(
    ZOOMSDK::IAudioSettingContext* audio_setting,
    const zchar_t* deviceId,
    const zchar_t* deviceName
);

/// \brief Set the suppress background noise level.
/// \param level The new suppress background noise level to be set.
/// \return If the function succeeds, the return value is SDKErr_Success.
/// Otherwise failed. To get extended error information, see \link SDKError \endlink enum.
extern "C" ZOOMSDK::SDKError set_suppress_background_noise_level(
    ZOOMSDK::IAudioSettingContext* audio_setting,
    ZOOMSDK::Suppress_Background_Noise_Level level
);

/// \brief Set the volume of the selected mic.
/// \param value Specify the volume of the mic that varies between 0 and 255.
/// The SDK will enable the default mic if there is no mic selected via SelectMic().
extern "C" ZOOMSDK::SDKError set_mic_volume(
    ZOOMSDK::IAudioSettingContext* audio_setting,
    FLOAT level
);

#endif