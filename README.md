# Drew's fast Rust MetalKit bindings

Provides select Rust bindings for Apple [MetalKit](https://developer.apple.com/documentation/metalkit) framework.  May be compared to alternative crates [metal](https://crates.io/crates/metal), [objrs_frameworks_metal](https://crates.io/crates/objrs_frameworks_metal),
[metalkit](https://crates.io/crates/metalkit) and [objrs_frameworks_metal_kit](https://crates.io/crates/objrs_frameworks_metal_kit).

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features of this library:

* Fast.  This crate is *significantly* faster than other crates.  If you're struggling to get 60fps or ProMotion/adaptive refresh rate speeds, this
  is the solution for you.
    * The full set of optimization is far too many to list, but the big idea is to either do what native ObjC/Swift applications do, or do something faster.
    * Compile-time selectors.  Most Rust crates do a runtime lookup for ObjC methods, which involves acquiring a lock and other yucky stuff, either on the first call or every call.  Instead, we do what real ObjC compilers do, which is way faster.  For more details, see [objr](https://github.com/drewcrawford/objr)
    * Smart pointers that provide global ARC inference.  Like ARC, you don't need to write manual retain/release calls.  Unlike ARC, the compiler
      usually doesn't need to write them either, meaning lower runtime memory management cost than even native code.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Runtime autorelease eliding, which keeps your objects out of autoreleasepools.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Pointer packing for optional types so they fit in a `u64`.  For more details, see [objr](https://github.com/drewcrawford/objr)
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with bound APIs and are not documented separately.  For details on how they work, see the native documentation.
* Free for noncommercial or "small commercial" use.

# Implementation status

I mostly implement the parts of the API I'm using in other projects.  At the moment, this consists primarily of MTKTextureLoader.

The [objr](https://github.com/drewcrawford/objr) macro system makes it very ergonomic to add new bindings for specific missing features
or new Metal APIs.