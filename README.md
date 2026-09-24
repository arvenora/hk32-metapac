# hk32-metapac

Peripheral access crate for HK32 microcontrollers.

## Supported chips

- `HK32F030MF4P6`

The first listed chip is enabled by default. To select another chip, disable default features:

```toml
hk32-metapac = { version = "0.1", default-features = false, features = ["hk32f030md4p6", "rt"] }
```
