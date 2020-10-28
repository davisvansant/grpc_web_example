###### <--- grpc web example --->
An experimental learning project to build a working app using `gRPC` in the browser

The main motivation here is to explore creative options around what `gRPC` can do, and extend that into the web as a potential alternative to shipping a CLI and/or embedding into apps and libraries

This is intended to be as simple as possible while learning the potential for this architecture

Say hello to a `gRPC` system over the web!

---

> Back End Overview

The back end is build using Rust, with Tonic as the framework

 - https://crates.io/crates/tonic

> Proxy Overview

Since `gRPC` over the web cannot natively talk `HTTP/2`, `Envoy` stands in as a proxy to help in the translation to speak to the backend

 - https://www.envoyproxy.io/

> Front End Overview

The front end is built with `Vue.js` as a static site designed to be ran from a `Rust` web framework - `actix-web`

  - https://vuejs.org/
  - https://github.com/actix/actix-web

> gRPC and Protocol Buffers Overview

The "glue" for all of this to work. The service is defined in the protocol buffers IDL and compiled to work by the front end and back end components, one in Rust and one in the "native" compiler (`protoc`), using the `grpc-web` plugin

- https://grpc.io/
- https://developers.google.com/protocol-buffers
- https://github.com/grpc/grpc-web

> Docker Overview

Once built, everything can be ran from a `Docker` container for portability (compose and k8s incoming)

 - https://www.docker.com/
