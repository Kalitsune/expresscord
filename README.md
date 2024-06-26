# Expresscord
Expresscord is a website that provides various way to express yourself on discord by including people profile pictures in gifs or images

> [!WARNING]
> This project is still very much in development and shouldn't be used in production servers
> you are free to contribute as you'd like though!

## Getting started
As the project is currently under development, there is no official way of deploying it. However, once it's done you'll be able to deploy it using docker.

### Environment
| Name           | Description                                                                                     | Default                           |
|----------------|-------------------------------------------------------------------------------------------------|-----------------------------------|
| `ADDRESS`      | The address to bind the webserver to.                                                           | `127.0.0.1`                       |
| `PORT`         | The port to open the webserver on.                                                              | `8080`                            |
| `RUST_LOG`     | Used to define the log level.                                                                   | `expresscord=info,actix_web=info` |
| `DISCORD_TOKEN | The token of a discord bot (see [developer portal](https://discord.com/developers/applications) | None                              |

## Development
### Testing changes
Make sure you have rust installed and use
```shell
cargo run
```
to start the webserver.

### Compiling
#### using cargo
Cargo make this step fairly easy, you'll get an executable by running
```shell
cargo build
```

#### Using docker
To be implemented.

## Roadmap
- [ ] Make the basics
- [ ] Create a docker image
- [ ] Add a pat expression similar to [petpet](https://benisland.neocities.org/petpet/)
- [ ] More to come...

## License
This project is distributed under the Apache 2.0 license, you can read a copy of it in the [License file](./LICENSE)