# WaveSight

Honest open-source WiFi & UWB sensing platform — see motion, breath and
presence without cameras, without cloud, without hype.

WaveSight turns ordinary WiFi (and later UWB) radio signals into spatial
intelligence: who is in the room, are they breathing, did they fall, are
they asleep. It runs on a small ESP32 mesh and processes everything on
the edge.

## Why another wifi-sensing project

The existing public projects in this space over-promise on marketing and
under-deliver on physics. Single-antenna ESP32 cannot reconstruct a true
MIMO array. So we are doing three things differently:

1. Real diversity — ESP32-C5 / C6 (WiFi 6 MIMO) plus optional UWB anchors.
2. Honest metrics — every shipped number has a reproducible benchmark.
3. Three vertical applications shipped in the same monorepo.

## Status

Alpha. See ROADMAP.md.

## License

MIT.