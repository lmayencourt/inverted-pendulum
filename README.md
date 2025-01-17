# Inverted pendulum

This repo contains the code for an inverted pendulum simulation.
The goal is to play with Rust simulation (bevy, rapier), graphics (egui) and machine-learning libraries.

This project is inspired by the [How to train simple AIs](https://www.youtube.com/watch?v=EvV5Qtp_fYg) video.

Try it [online](https://lmayencourt.github.io/inverted-pendulum/)!

## Requirements overview
Essential features are:
 - The program display an interactive pendulum simulation, with graphs info about position and speed of the pendulum.
 - The pendulum simulator can be played by a human player, or by an AI agent.
 - The AI agent can be trained on the same software.

# Solution strategy
- [Rust](https://www.rust-lang.org) as a development language.
- [bevy engine](https://bevyengine.org) as game engine.
- [Rapier]() as a physic engine.
- [Web Assembly]() as the deployment target.

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE.txt) or http://opensource.org/licenses/MIT)
