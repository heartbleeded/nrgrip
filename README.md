nrgrip(1) - rip Nero Burning ROM audio NRG images
=================================================

SYNOPSIS
--------

`nrgrip` [-icrx] [options] <image.nrg>

`nrgrip` [-h | -V]

DESCRIPTION
-----------

NRGrip works on a Nero Burning ROM's NRG image of an audio CD and is able to:

* read and display its metadata;

* extract the cue sheet;

* extract the raw audio tracks as one single file, which can then be encoded by
  the user to a more convenient audio format such as FLAC, and possibly split
  according to the cue sheet.

For now, only NRG v2 is handled (not NRG v1), and not all of the metadata chunks
are decoded. If you have interest in adding support for additional chunks or
formats and have a test image handy, please contact the author or open an
issue. In particular, it would be interesting to support ISRC/CD-Text; handling
of multisession and hybrid (audio and data) discs would also be a nice feature.

OPTIONS
-------

At least one action switch must be provided, along with any number of option
switches.

### Actions

* `-i`, `--info`:
  display the NRG image metadata (default action)

* `-c`, `--extract-cue`:
  extract cue sheet from the NRG metadata

* `-r`, `--extract-raw`:
  extract the raw audio tracks

* `-x`, `--extract`:
  same as `-cr`

### Additional options

* `-S`, `--no-strip-subchannel`:
  don't strip the 96-bit subchannel from the extracted audio data (this option
  has no effect if the input image has standard 2352-byte sector size)

EXAMPLE
-------

The following command will display the metadata and extract both the cue sheet
and the audio data:

    nrgrip -ix image.nrg

The cue sheet will be extracted as `image.cue`, and the audio data as
`image.raw` in the current directory.

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

INSTALLATION
------------

NRGrip is written in Rust. Make sure [Cargo](http://doc.crates.io/) is installed
on your system, then you can install directly from the Git repository with:

    cargo install --git https://code.lm7.fr/mcy/nrgrip.git

Or if you cloned the repository already:

    cargo build --release
    cargo install

You may also generate the manpage with:

    make

COPYRIGHT
---------

NRGrip was written by Matteo Cypriani <<mcy@lm7.fr>> and is licensed under the
terms of the Expat (MIT) license. See the [COPYING](COPYING) file.

SEE ALSO
--------

Additional information can be found on
[NRGrip's wiki](https://code.lm7.fr/mcy/nrgrip/wiki).
