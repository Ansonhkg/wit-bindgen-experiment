// src/lib.rs

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
  // the name of the world in the `*.wit` input file
  world: "host",

  // For all exported worlds, interfaces, and resources, this specifies what
  // type they're corresponding to in this module. In this case the `MyHost`
  // struct defined below is going to define the exports of the `world`,
  // namely the `run` function.
  exports: {
      world: MyHost,
  },
});

struct MyHost;

impl Guest for MyHost {
    fn run() {
        print("Hello, world!");
    }
}