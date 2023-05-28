
<div align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./rsc/dark.png">
  <img  width="200" height="200" alt="Text changing depending on mode. Light: 'So light!' Dark: 'So dark!'" src="./rsc/light.png">
</picture>
<h3 align="center">Quanttp2-rs</h3>

  <p align="center">
  Rust port of the <a href="https://github.com/TheRandonauts/quanttp2"> Quanttp 2 Entropy Server</a> / used by Randonautica to serve quantum random numbers
    <br />
    <a href="https://github.com/TheRandonauts/quanttp2-rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/TheRandonauts/quanttp2-rs/issues">Request Feature</a>
  </p>
</div>

## Config
Use the `PORT` env to configure the listening port, defaults to `3000`

## Building
```
$ cargo build
```

## Running
```
$ cargo run
```

## API Endpoints:

`/api/devices`
+ List all connected devices by `device_id`

`/api/status`
+ Provides analytical information regarding servers uptime and version information

`/api/reset`
+ Attempts to reset an unresponsive device
+ Required parameter: `device_id`

`/api/clear`
+ Attempts to clear a device's buffer
+ Required parameter: `device_id`

`/api/randint32`
+ Returns a random 32bit integer
+ Required parameter: `device_id`

`/api/randuniform`
+ Returns a random 32bit uniform floating point number
+ Required parameter: `device_id`

`/api/randnormal`
+ Returns a random number from a normal Gaussian distribution
+ Required parameter: `device_id`

`/api/randhex`
+ Returns `length` amount of random bytes in hexadecimal format
+ Required parameter: `device_id`, `length`

`/api/randbase64`
+ Returns `length` amount of random bytes in Base64 format
+ Required parameter: `device_id`, `length`

### JSON Formatted

Return the same data, but JSON formatted

+ `/api/json/devices`
+ `/api/json/randint32`
+ `/api/json/randuniform`
+ `/api/json/randnormal`
+ `/api/json/randbase64`

## Usage Examples


`http://localhost:3000/api/devices`
```Python
QWR70154
```

`http://localhost:3000/api/status`
```Python
>>> {"server":"localhost", "status":true, "uptime":3011319, "version":"0.1.4"}
```

`http://localhost:3000/api/json/randhex?length=10&device_id=QWR70154`
```Python
>>> "6e0ff2447b5999befd43"
```

`http://localhost:3000/api/randuniform?length=10&device_id=QWR70154`
```Python
>>> 0.5370516731852106
```


## Contributing

We don't have a very specific contributing guidelines right now so just feel free to do the following. Contributors are always appreciated!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.MD` for more information.

