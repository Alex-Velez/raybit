use super::{FnOutput, Memory, Pointer};

/// Initialize audio device and context
pub unsafe fn init_audio_device(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Close the audio device and context
pub unsafe fn close_audio_device(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if audio device has been initialized successfully
pub unsafe fn is_audio_device_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set master volume (listener)
pub unsafe fn set_master_volume(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get master volume (listener)
pub unsafe fn get_master_volume(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load wave data from file
pub unsafe fn load_wave(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
pub unsafe fn load_wave_from_memory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Checks if wave data is ready
pub unsafe fn is_wave_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load sound from file
pub unsafe fn load_sound(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load sound from wave data
pub unsafe fn load_sound_from_wave(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Create a new sound that shares the same sample data as the source sound, does not own the sound data
pub unsafe fn load_sound_alias(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Checks if a sound is ready
pub unsafe fn is_sound_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Update sound buffer with new data
pub unsafe fn update_sound(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload wave data
pub unsafe fn unload_wave(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload sound
pub unsafe fn unload_sound(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload a sound alias (does not deallocate sample data)
pub unsafe fn unload_sound_alias(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export wave data to file, returns true on success
pub unsafe fn export_wave(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export wave sample data to code (.h), returns true on success
pub unsafe fn export_wave_as_code(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Play a sound
pub unsafe fn play_sound(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Stop playing a sound
pub unsafe fn stop_sound(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Pause a sound
pub unsafe fn pause_sound(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Resume a paused sound
pub unsafe fn resume_sound(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a sound is currently playing
pub unsafe fn is_sound_playing(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set volume for a sound (1.0 is max level)
pub unsafe fn set_sound_volume(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set pitch for a sound (1.0 is base level)
pub unsafe fn set_sound_pitch(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set pan for a sound (0.5 is center)
pub unsafe fn set_sound_pan(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Copy a wave to a new wave
pub unsafe fn wave_copy(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Crop a wave to defined samples range
pub unsafe fn wave_crop(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Convert wave data to desired format
pub unsafe fn wave_format(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load samples data from wave as a 32bit float data array
pub unsafe fn load_wave_samples(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload samples data loaded with LoadWaveSamples()
pub unsafe fn unload_wave_samples(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load music stream from file
pub unsafe fn load_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load music stream from data
pub unsafe fn load_music_stream_from_memory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Checks if a music stream is ready
pub unsafe fn is_music_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload music stream
pub unsafe fn unload_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Start music playing
pub unsafe fn play_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if music is playing
pub unsafe fn is_music_stream_playing(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Updates buffers for music streaming
pub unsafe fn update_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Stop music playing
pub unsafe fn stop_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Pause music playing
pub unsafe fn pause_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Resume playing paused music
pub unsafe fn resume_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Seek music to a position (in seconds)
pub unsafe fn seek_music_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set volume for music (1.0 is max level)
pub unsafe fn set_music_volume(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set pitch for a music (1.0 is base level)
pub unsafe fn set_music_pitch(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set pan for a music (0.5 is center)
pub unsafe fn set_music_pan(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get music time length (in seconds)
pub unsafe fn get_music_time_length(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current music time played (in seconds)
pub unsafe fn get_music_time_played(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load audio stream (to stream raw audio pcm data)
pub unsafe fn load_audio_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Checks if an audio stream is ready
pub unsafe fn is_audio_stream_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload audio stream and free memory
pub unsafe fn unload_audio_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Update audio stream buffers with data
pub unsafe fn update_audio_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if any audio stream buffers requires refill
pub unsafe fn is_audio_stream_processed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Play audio stream
pub unsafe fn play_audio_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Pause audio stream
pub unsafe fn pause_audio_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Resume audio stream
pub unsafe fn resume_audio_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if audio stream is playing
pub unsafe fn is_audio_stream_playing(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Stop audio stream
pub unsafe fn stop_audio_stream(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set volume for audio stream (1.0 is max level)
pub unsafe fn set_audio_stream_volume(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set pitch for audio stream (1.0 is base level)
pub unsafe fn set_audio_stream_pitch(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set pan for audio stream (0.5 is centered)
pub unsafe fn set_audio_stream_pan(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Default size for new audio streams
pub unsafe fn set_audio_stream_buffer_size_default(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Audio thread callback to request new data
pub unsafe fn set_audio_stream_callback(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Attach audio stream processor to stream, receives the samples as s
pub unsafe fn attach_audio_stream_processor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Detach audio stream processor from stream
pub unsafe fn detach_audio_stream_processor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Attach audio stream processor to the entire audio pipeline, receives the samples as s
pub unsafe fn attach_audio_mixed_processor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Detach audio stream processor from the entire audio pipeline
pub unsafe fn detach_audio_mixed_processor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}
