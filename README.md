## External Native Build `cargo`

Tasks

- [ ] Plugin step to collect config from project environment
    - [ ] `externalNativeBuild` `cargo`
    - [ ] `architecture`
    - [ ] `ndk` options with compiler paths for the `architecture`
- [ ] Plugin step to discover cargo and probably rustc (invisible shell settings in gradle script)
    - [ ] MacOS
    - [ ] Linux
    - [ ] Windows
- [ ] Plugin step to figure out where to put built libs, create config model for that
- [ ] Plugin step to configure build environment (stored in `.externalNativeBuild/cargo`)
    - [ ] Override C compiler paths over `.cargo/config`
    - [ ] Environment variables?
    - [ ] Copying project there and patching it? (probably we should avoid this)
- [ ] Build crates
    - [ ] Build pure-rust lib
    - [ ] Build rust lib with simple C dependency (`libz-sys`)
    - [ ] Build rust lib with C dependency that depends on another C project (`freetype-sys`)
    - [ ] Build rust lib that depends on CMake lib (`harfbuzz`)
    - [ ] Build rust lib with a dependency that does crazy stuff in its build script (`sdl2`)

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

... and the plugin does the rest: passes the correct build environment to CMake, passes the correct
NDK headers and builds the shared library and puts it in a standard location so that it can be
loaded and used from Java (or Kotlin). It also takes care of building multiple libraries for
all the necessary architectures.

### The Goal

We want to do the same, but with Rust! We will create the plugin that can compile Rust library. 
This library should be loadable from Java the same way the CMake library is. 
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

Inside the `build.gradle` file, you will find a huge chunk of commented-out code. It contains
the plugin `class CargoBuild extends DefaultTask` that proves that this task we outlined is possible:
it is possible to extend gradle DSL and add the `cargo` property to `externalNativeBuild`, it is possible to query 
this property when the Android project is built. The actual plugin code is incomplete and hacked
together just to get something working.

However, this inline Groovy plugin is not a good way to do things. The better approach is to move
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

Currently this crashes (because the DSL is not patched properly yet), 
but it does dump out all kinds of information in the console.

### The plugin

The plugin is written in Kotlin, and does not work at this moment.
It can be found here: https://github.com/Nercury/gradle-cargo-plugin

To work on plugin locally, it has a gradle task to publish it to local maven repository.
To do that, run `publishToMavenLocal` gradle task. Then, this Android project will be
able to pick this plugin up.

### The goals of this repository

This repository will eventually become the example of how to use this plugin.
