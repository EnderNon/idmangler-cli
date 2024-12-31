# Shiny
This data block encodes for Shiny data present on gear items.
- Field name: `shiny`
- Field type: `{key, value}`
- Example: `"shiny": {"key": "playersKilled","value": 20}`
- ## Key
  - This is the type of the shiny stat present. 
  - The potential keys for any shiny value are listed [HERE](https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json).
  - Field name: `key`
  - Field type: `String`
  - Example: `"key": "playersKilled"`
- ## Value
  - This is the value of the shiny stat present.
  - Field name: `value`
  - Field type: `Signed 64-bit Integer`
  - Field range: `-9,223,372,036,854,775,808` to `9,223,372,036,854,775,807`
  - Example: `"value":"20"`