# SSB64 Collision Processed JSON Format
## Schema

## Enum Values
### points.properties
This value defines the interaction of the plane composed of points with a character. It is a series of flags, 
so these properties can be combined with a "|".

| flag       | explanation |
|------------|:------------|
| normal     | The default flag state and signifies the absence of any other flag. |
| fall-thru  | Characters can pass thru a plane (made of points with this flag) by pressing down when standing on that plane |
| ledge-grab | The point can be grabbed (if it, or a point it is connected to, is at the edge of plane |

### points.floor
The type of the floor. Mainly this is for changing the friction of a plane's surface, but this is also how the special effects
are made for certain objects (lava, spikes, etc.) in Break the Targets and Board the Platforms stages 

## Example
```JSON
{
  "points" : [
    {
      "id": 0,
      "x": -100,
      "y": 0,
      "properties": "LEDGE_GRAB",
      "floor": "Normal"
    },
    {
      "id": 1,
      "x": 100,
      "y": 0,
      "properties": "NORMAL",
      "floor": "Normal"
    },

  ],
  "collision" : [
    {
      "id" : 1,
      "top" : {
        "planes" : [
          [0, 1],
          [2, 3],
          [4, 5]
        ]
      },
      "bottom" : [

      ],
      "right" : [

      ],
      "left" : [

      ]
    }
  ],
  "spawns" : [
    {
      "type": 0,
      "x" : -10,
      "y" : 100
    }
  ]
}
```
