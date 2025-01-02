# Item type
- This data block encodes for the type of the item that's being encoded.
- This will support crafteds in the future, but for now it only supports non crafted types.
- If you input a crafted value, literally nothing will happen, I think.
- Field name: `item_type`
- Field type: `String`
- Field potential values: `"Gear"`, `"Tome"`, `"Charm"`
- Example: `"item_type": "Gear"`