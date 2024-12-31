# Powder Limit
- This data block encodes for the number of powders allowed.
- It doesn't do anything useful, it isn't  the powder limit visible on the item.
- If you are encoding powders, this must be present, and must be more than or equal to the number of powders you have encoded.
- Field name: `powder_limit`
- Field type: `Unsigned 8-bit Integer`
- Field range: `0` to `255`
- Example: `"powder_limit": 255`