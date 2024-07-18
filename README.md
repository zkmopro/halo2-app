# Halo2 Mopro Sample App

This is a sample app that demonstrates how to use the Halo2 adapter in an iOS and Android app. The app generates a proof for a Fibonacci circuit and verifies it.

The app uses the `halo2_fibonacci` circuit from the [Halo2 Fibonacci Mopro sample circuit](https://github.
com/ElusAegis/halo2-fibonacci-sample.git). It contains the Halo2 logic for setting up the keys and generating 
and verifying the proof.

## Getting started

Clone this repo then run either `cargo run --bin ios` or `cargo run --bin android` to build for the selected target. 

Then, follow the guide from the [Mopro Documentation](https://zkmopro.org/docs/getting-started) to use the resulting library in your app.

## Production build

If you are planning to use this in a production app, you should build the library in the release mode. To do this, 
run the following:

```sh
    CONFIGURATION=release cargo run --bin ios
```

Or 

```sh
    CONFIGURATION=release cargo run --bin android
```


## Testing 

To run the integration tests, run the following in the root of the project:

```sh
  curl -L https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar -o jna-5.13.0.jar
  CLASSPATH=jna-5.13.0.jar cargo test -- --nocapture
```