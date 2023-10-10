# Minimix
A mini audio mixer for Rust. See the examples.

## Purpose
This is designed to be a primitive, that sits on top of cpal. If you were making game audio in cpal you would basically have to make this.

It is also designed to be a primitive for DSP. This crate implements the bare minimum functionality but stay tuned for future package implementing music/DSP operations, FIR, IIR etc. If you can describe a sound at the sample level and know how to code it this is a usable foundation.

## Features
* device selection is forwarded from cpal or just use default device
* wav load & save
* signal playback

## Signal Abstraction
Its literally an array of samples, i.e. there is no concept of sample rate or channels for simplicity, do that yourself. Supporting multiple channels is a matter of interleaving the signals. e.g. mono to stereo is  `signal.interleave(signal.clone())` (well more like `s.interleave2(&s.clone())` xD). See the examples.

Additionally signal has no concept of sample rate so just resample everything.

## Features
These can be toggled off to reduce dependencies as they are not needed
* WAV I/O (riff-wave)
* Plotting (png)
* Playback (cpal)
* Synthesis (rustfft)

## Todo
* implement sound stopping
* what about channels and repeats, sounds whack
* optional features for all that other shit 

## Issues 
* race condition: load & play (I mean its not going to be a problem in a game or whatever but thats why in the examples there is a 10ms sleep)
* not the highest quality resampling - its a whole thing - feel free to use libsamplerate-rs

## Random notes 
* signal power
* bruh panning and shit is gonna be lit as fuck