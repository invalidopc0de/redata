What is reData?
===============

reData is a centralized data storage service for hardware test data.  The goal is to
provide a self-hosted service where users can store and retrieve binary data,
timeseries data, logs, etc.

Development
===========

reData uses skaffold and minikube to do local development.  To get started:

1. Setup minikube (https://minikube.sigs.k8s.io/docs/start/)
2. Install skaffold (https://skaffold.dev/docs/install/#standalone-binary)
3. Install pre-commit (https://pre-commit.com)
4. Run `pre-commit install --hook-type commit-msg` to install git precommit hooks

To build the required images run: `skaffold build`

To run a development environment run: `skaffold run`

Skaffold also includes ability to watch the source code and trigger rebuilds
automatically.  This can be done with: `skaffold dev`

NOTE: The example setup in `/k8s` uses hard-coded secrets (access keys, etc).  These
are purely for development purposes and should be changed for a production setup.

Style
=====

- Please use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) for commit messages

Roadmap
=======

- [ ] Run management (create/update/delete)
- [ ] Run key/value properties
- [ ] Blob data upload/download
- [ ] Timeseries CSV upload
- [ ] Timeseries Parquet upload
- [ ] Timeseries data plotting
- [ ] Automated tool support (Run badges)


Example Use Cases
=================

*Given the author's familiarity with rocket testing, these use cases are a little
biased 🚀

***Rocket Engine Test Campaign***

A rocket engine test campaign typically consists of a series of tests that run for a
few seconds/minutes and then are reviewed by a team afterward.

*What data is produced?*

Typical data includes timeseries periodic telemetry, system logs, and video data of the
test.

*How is the data used?*

Engine tests are experiments, with the goal of learning how the engine performs, with the
outcome frequently unknown.  Much of this data review is done by humans manually looking
at the data to understand how the system behaved.

***Hardware-In-The-Loop (HITL) Testing***

TODO


***Flight Data Recording***

TODO
