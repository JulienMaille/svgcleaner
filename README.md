# Svg Cleaner (& Fixer)

Original Repo is here
https://github.com/RazrFalcon/svgcleaner
---

## Svg Cleaner

helps you clean up your SVG files, keeping them free from unnecessary data.

## Fixer

fixes bad svgs before they are cleaned

### Purpose

The main purpose of *svgcleaner* is to losslessly reduce the size of an SVG image, created in a
vector editing application, before publishing.

Usually more than half of an SVG image's data is useless for rendering. For example:

- Temporary data used by the vector editing application
- Non-optimal SVG structure representation
- Unused and invisible graphical elements

... Are all unnecessary in a published SVG.

## Goals

#. **Correctness** - This project should not break an SVG file
#. **Cleaning ratio** - Higher is better
#. **Performance** - An average SVG file processing time should be closer to ~1ms on a modern PC


## Documentation

Documentation can be found [here](cleaner/docs/svgcleaner.adoc>).

## Limitations
-----------

*svgcleaner* shouldn't change your file unless you tell it to, but there are still
things that can't be preserved. So even if you disable all cleaning options there are still things
that will be changed, such as:

- Original indent is not preserved
- All colors will be formatted as #RRGGBB and #RGB
- DOCTYPE, CDATA will be processed and removed
- CSS support is minimal
- CSS from the ``style`` element will be extracted and processes. The ``style`` element will be removed.
- The ``style`` attribute will be split into attributes
- The ``class`` attribute will be processed and removed
- Paths and transformations will be reformatted
- ``currentColor`` and ``inherit`` attributes values will be resolved
- Referenced elements will be moved to the ``defs`` element
- IRI and FuncIRI attributes that reference non-existing objects will be removed
- If the ``offset`` attribute value of the ``stop`` element is represented as percentage - it will be
  converted into a decimal number

## Usage & Help

``` bash
fixer --help
svgcleaner --help
```

### GUI

You can get a GUI `here <https://github.com/RazrFalcon/svgcleaner-gui>`.
You can get prebuilt packages `here <https://github.com/RazrFalcon/svgcleaner-gui/releases>`.

## Building

Dependency: [Rust](https://www.rust-lang.org/)

``` bash
  cd cleaner
  cargo build --release

  cd ../fixer
  cargo build --release
```

If you're a Rust programmer, you can install either by using:

  cargo install fixer
  cargo install svgcleaner


### License

This project is licensed under the [GPL-2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html) as per the original project by [RazorFalcon](https://github.com/RazrFalcon/svgcleaner).
