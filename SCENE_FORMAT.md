# Scene File Format

Scene files are specified in XML. The first and only tag in the root of the file should be `<scene>` under which all other tags should be nested. Other than this root tag, The following are available:
* camera
* model
* light

One and only one camera tag *must* be present for a scene file to be valid. The other two tags can be repeated as many times as desired. The next sections will specify each of these tags and their corresponding options with an annotated example. The annotations will be text that follows the pattern `[SOME-TEXT]`. In a real file, these bracketed names are expected to be replaced by a value (a floating point number unless otherwise specified).

## Camera

The camera tag can be specified as follows:

```
<camera>
    <projection> [IMAGE_WIDTH] [IMAGE_HEIGHT] [FOV] [NEAR_CLIP] [FAR_CLIP] </projection>
    <position> [X] [Y] [Z] </position>
    <lookat> [X] [Y] [Z] </lookat>
    <up> [X] [Y] [Z] </up>
</camera>
```

Where image width and height are in pixels (and must be positive integers) and FOV is specified in radians. The lookat tag specifies the point in 3D space the camera will be looking at, and the up tag defines how the camera should be oriented (such that if positive Y is the up axis then the camera has no roll rotation). The up tag should be a unit vector.

## Model

The model tag can be specified as follows:

```
<model>
    <mesh> [PATH] </mesh>
    <rotation> [ROLL] [PITCH] [YAW] </rotation>
    <position> [X] [Y] [Z] </position>
</model>
```

path should be a string enclosed in double quotes `"`. Paths should be given relative to the location of the scene file. Roll, pitch, and yaw should be specified in radians.

## Light

```
<light>
    <strength> [STRENGTH] </strength>
    <position> [X] [Y] [Z] </position>
    <color> [R] [G] [B] </color>
</light>
```

Strength should be a floating point number between 0.0 and 1.0. R G & B are the red green and blue components of color. Each should be an integer between 0 and 255.


## Example File

The following is a full example of a scene (the one used to render the image `data/example_render.png`):

```
<scene>
  <camera>
    <projection> 1920 1080 0.57 0.01 1000</projection>
    <position> 0 -1 -4.3 </position>
    <lookat> 0 -0.08 0 </lookat>
    <up> 0 1 0 </up>
  </camera>

  <model>
    <mesh> "table.obj" </mesh>
    <rotation> 0.0 0.0 0.5 </rotation>
    <position> 0 -1.0 0.0 </position>
  </model>

  <model>
    <mesh> "bunny.obj" </mesh>
    <rotation> 0.0 0.0 0.0 </rotation>
    <position> 0 -0.08 0.0 </position>
  </model>

  <model>
    <mesh> "skybox.obj" </mesh>
    <rotation> 0.0 0.0 0.0 </rotation>
    <position> 0.0 0.0 -140 </position>
  </model>

  <light>
    <strength> 0.4 </strength>
    <position> -2.0 1.0 0 </position>
    <color> 200 200 200 </color>
  </light>
</scene>
```
