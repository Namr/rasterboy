# Rasterboy
[![MIT](https://img.shields.io/github/license/Namr/rasterboy)](https://github.com/namr/rasterboy/blob/main/LICENSE-MIT)
![Release](https://img.shields.io/badge/Release-0.1.0-blue)
![Build Status](https://img.shields.io/github/actions/workflow/status/Namr/rasterboy/build.yml)

A software rasterizer written in Rust completely from scratch; The only dependency is `std`. It can load OBJ meshes, render with phong lighting, and apply texutres to models with bilinear filtering. Scenes can be specified in a custom XML format. The following image was rendered with rasterboy:

![bunny on a table in the sky](data/example_render.png)

