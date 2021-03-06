# Reddit Place Analysis

> Made using Rust™ 🚀

This is the ultimate cli tool for the analysis of the two
[Reddit Place](https://www.reddit.com/r/place/) events in 2022 and 2017.

Download the full r/place dataset
[here](https://placedata.reddit.com/data/canvas-history/2022_place_canvas_history.csv.gzip)
and place it in the `data` directory.

I (and you) can found your hash using [this website](http://kisielo85.cba.pl)
(even though a future version of this tool will be able to do this
automatically).

My hash:
`bCrZRP7T31V14qwiWNzeBDKckEr+7q5aWwtYi/xnGSI57DwO4pWc5Ce1axjS3yNhF9wvmA2THtL/lwbIIeF69A==`

## Todo

* [ ] Comment the code
* [ ] Add a `--help` option
* [ ] Add a `--version` option
* [ ] Add a `--verbose` option
* [ ] Add a `--heatmap` option
* [ ] Add a `--pixelsize` option
* [ ] Add a `--userhash` option
* [ ] Add a `--user` option
* [ ] Add a `--background_opacity` option
* [ ] Implement the sorting of the pixels by their time of placement
* [ ] Implement an algorithm to find the user's hash using user-collected
      datasets
* [ ] Implement the configuration file
* [ ] Implement the possibility to work with multiple user's hashes or names
* [x] Use `Path` instead of `&str` for each paths
* [ ] Reformat the place_pixels function
* [ ] Considering using the
      [`piecewise-linear`](https://crates.io/crates/piecewise-linear) crate for
      the `get_heatmap_color` function
* [ ] Add a timestamp for each pixel
