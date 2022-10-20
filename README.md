
# SDL2 rust wrapper

The intension here is to expose as little as possible from SDL2 as needed.  Static linking, all dev, testing, usage is likely to be done only on windows so have low expectations for other platforms.

Read about SDL2 here:  [https://www.libsdl.org/](https://www.libsdl.org/)

Read about SDL2's zlib license here:  [zlib license](https://www.zlib.net/zlib_license.html)

SDL2 static library is not going to be included in this repository.  Checkout their releases page:  [SDL2 Releases](https://github.com/libsdl-org/SDL/releases)

Initial development is being done against 2.24.0.  It's unclear if this project is going to concern itself with staying current as time goes on.

Recommendation is to not use this repository for anything important.  This is a hobby level effort.

# SDL2_ttf rust wrapper

Also wrapping SDL2_tff.  

Read about SDL2_ttf here:  [https://github.com/libsdl-org/SDL_ttf](https://github.com/libsdl-org/SDL_ttf)

SDL2_ttf license appears more complicated than just SDL2.  It looks like it's zlib, but wraps FreeType and Harfbuzz projects which have their own licensing requirements.  

SDL2_ttf library is not going to be included in this repository.  Checkout their release page:  [SDL2_ttf Releases](https://github.com/libsdl-org/SDL_ttf/releases)

# Misc

* Right now you still need the SDL2.dll in order to use this crate.  This has something to do with how SDL2 is built when being used with the visual studio c++ build tools.