## External Native Build `cargo`

## Background and detailed explanation

What are we doing here?

Android has greatly simplified the way C++ projects are built (compared to the way it was done previously). 
As long as you use CMake, it is enough to add this configuration to the `build.gradle`:

```groovy
    externalNativeBuild {
        cmake {
            path "src/main/cpp/CMakeLists.txt"
        }
    }
```

... and the plugin does the rest: passes correct build environment to CMake, passes correct
NDK headers and builds the shared library and puts it in a standard location so that it can be
loaded and used from Java (or Kotlin). It also takes care of building multiple libraries for
all necessary architectures.

### The Goal

We want to do the same, but with Rust! We will create the plugin that compile Rust library that
 can be loaded from Java the same way the CMake library is loaded. 
We want to be able to include this configuration in Android project:

```groovy
    externalNativeBuild {
        cargo {
            path "src/main/rust/Cargo.toml"
        }
    }
```

The plugin should take care of passing all the necessary parameters to the Cargo to make this
happen.

### The current state

Inside the `build.gradle` file, you will find huge chunk of commented-out code. It contains
the plugin `class CargoBuild extends DefaultTask` that is proves that this is possible:
we can extend gradle DSL to add the `cargo` property to `externalNativeBuild`, and we can query 
this property when the Android project is built. The actual plugin code is incomplete and hacked
together just to get something working.

This inline groovy plugin is not a good way to do things. The better approach is to move
this plugin to a separate project. It can then be imported by adding a dependency:

```groovy
dependencies {
    classpath 'com.nercury:cargobuild:1.0.0'
}
```

And then applying this plugin down bellow:

```groovy
apply plugin: 'com.nercury.cargobuild'
```

### The plugin

The plugin is written in kotlin, and does not work at this moment.
It can be found here: https://github.com/Nercury/gradle-cargo-plugin

To work on plugin locally, it has a gradle task to publish it to local maven repository.
To do that, run `publishToMavenLocal` gradle task. Then, this Android project will be
able to pick this plugin up.

### The goals of this repository

This repository will eventually become the example how to use this plugin.
