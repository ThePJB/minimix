# Minimix
A mini audio mixer for Rust


## Features
* wav playback in realtime
* wav load & save
* adding signals
* set volume of signals in dB

* signal gets put directly in callback buffer so u do need to interleave if its channels

## Issues 
* Supporting multiple channels
    * sln: samples are whatever. commutate signal -> 2 signals, pan = adjust separately, whatever
            play with signals addressed to channels as well as multi
            get this what if we dont even address it to whatever channel we just interleave
* Supporting different sample rates
    * sln: samples are whatever. resample it once. Interp resample 
* need example program
* windowed and ADSR'd sins for playing chord progressions and have panning and stuff too
* race condition: load & play
    * sln: dont read play queue while anything in load queue
* supporting devices (default device path) (query devices and select device when making the endpoint, or the 'Minimixer)
* signal power
* impl stop
* release

* probably a bug with n len and repeat of numerous channels if it not evenly divides

yea ideally i can just forward cpal device shit so i dont have to do it ay

seems like i broke it from 