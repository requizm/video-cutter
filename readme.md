It uses ffmpeg to create clips from the video. Check [input.json](input.json) for example input file.

## TODO

- Use the same video stream for the clips that point to the same video file. (probably using with ffpmeg internal functions, not command line)

## Json schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "videos": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "videoPath": {
            "type": "string",
            "format": "relative-ref"
          },
          "clips": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": {
                  "type": "string"
                },
                "start": {
                  "type": "string",
                  "pattern": "^\\d{2}:\\d{2}:\\d{2}$"
                },
                "end": {
                  "type": "string",
                  "pattern": "^\\d{2}:\\d{2}:\\d{2}$"
                }
              },
              "required": [
                "name",
                "start",
                "end"
              ]
            }
          }
        },
        "required": [
          "videoPath",
          "clips"
        ]
      }
    },
    "outputFolder": {
      "type": "string",
      "format": "uri-reference"
    },
    "resolution": {
      "type": "string",
      "pattern": "^\\d+x\\d+$"
    },
    "fps": {
      "type": "integer",
      "minimum": 0
    },
    "videoFormat": {
      "type": "string"
    },
    "preset": {
      "type": "string",
      "enum": [
        "ultrafast",
        "superfast",
        "veryfast",
        "faster",
        "fast",
        "medium",
        "slow",
        "slower",
        "veryslow"
      ]
    }
  },
  "required": [
    "videos",
    "outputFolder"
  ]
}
```
