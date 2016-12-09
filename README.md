NRGrip - extract audio data and cue sheet from an NRG audio CD image
====================================================================

This program works on a Nero Burning ROM's NRG image of an audio CD and is able
to:

- read and display its metadata;

- extract the cue sheet;

- extract the raw audio tracks as one single file, which can then be encoded by
  the user to a more convenient audio format such as FLAC, and possibly split
  according to the cue sheet.

For now, only NRG v2 is handled (not NRG v1), and not all of the metadata chunks
are decoded. If you have interest in adding support for additional chunks or
formats and have a test image handy, please contact the author or open a
ticket. In particular, it would be interesting to support ISRC/CD-Text; handling
of multisession and hybrid (audio and data) discs would also be a nice feature.

NRGrip is licensed under the terms of the Expat (MIT) license. See the `COPYING`
file.


Installing
----------

NRGrip is written in Rust. Make sure [Cargo](http://doc.crates.io/) is installed
on your system, then:

    cargo build --release
    cargo install


Usage
-----

To see the full usage, run:

    nrgrip --help

Basically, you can display the metadata and extract the data with:

    nrgrip -ix image.nrg

The cue sheet will be extracted as `image.cue`, and the audio data as
`image.raw`.

The raw audio data from a CD is 16 bit, little endian, 44100 Hz, stereo. To
play it, you may use, for instance, `aplay` from the ALSA utils, or `ffplay`
from `FFmpeg`:

    aplay -f cd image.raw
    ffplay -f s16le -ac 2 image.raw

Note that you can also play the NRG file directly, if the sector size is 2352
(no sub-channel).

To encode the raw audio data to FLAC (and embed the cue sheet in it):

    flac --endian=little --sign=signed --channels=2 --bps=16 \
      --sample-rate=44100 --cuesheet=image.cue image.raw

To split the FLAC file according to the cue sheet, you may use,
[cuetools](https://github.com/svend/cuetools) and
[shntool](http://www.etree.org/shnutils/shntool/):

    cuebreakpoints image.cue | shnsplit -o flac image.flac

Or [mp3splt](http://mp3splt.sourceforge.net/):

    mp3splt -c image.cue image.flac
