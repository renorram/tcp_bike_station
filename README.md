# TCP Bike Station
A Bike station app through TCP/IP.

*this is for a college assignment and for practice*

## Build Requirements

- [Rust 1.50+](https://www.rust-lang.org/tools/install)

## How to build

Clone the repository

```shell script
git clone 
```

Enter the repository folder

```shell script
cd repository_folder
```

Build the project

```shell script
cargo build 
```

To make a release build just use the `--release` flag:

```shell script
cargo build --release 
```

The binary file can be found at the folder `target/release`, for release build, or `target/debug` for the debug build. The name of the binary is __tcp_bike_station__.

## How to use

The application has 2 main commands, one for the server and the for the client. 
The specifications below assumes that you're using Rust's `cargo` tool to build/run the application. 
If you're only using the binary you can replace `cargo run` for `binary_name`.

### Server Command
The server command has the following syntax:

```shell script
cargo run server [port] [time_limit]
```

The arguments between square brackets are optional.

the `port` argument it's a decimal 16-bit sized value that specifies in which port the application is going to be served, the default is 8080.

the `time_limit` argument it's a decimal 8-bit sized value that specifies the time limit for a bike rent register not to be charged.

### Client Command
The client command has the following syntax:

```shell script
cargo run client [server_address]
```

The arguments between square brackets are optional.

The `server_address` argument it's socket address value that if specified must have the following format __(IP_ADDRESS:PORT)__: `255.255.255.255:8021`

You can use the `server_address` argument when the server it's on another machine or another port other than the default: `127.0.0.1:8080`

#### Renting a bike and finishing a rent

After start the client you can rent a bike or return it, doing so, finishing the a rent. (maybe a should use the word closing here but whatever hahah XD)

To rent a bike:

```
rent {ID_PERSON} {ID_STATION}
```

To finish the rent:

```
rent {ID_PERSON}
```

The value between brackets are mandatory.

__ID_PERSON__ is the person identifier, it can be any valid string.

__ID_STATION__ is the station identifier, it can be any valid string.

To exit the client and close the stream you can type `exit` and press enter.