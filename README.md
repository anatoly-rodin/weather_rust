# Weather Forecast CLI tool in Rust

My pilot project in Rust for getting weather information via terminal.

## First step before to move forward
To develop and build a project, you must have the Rust compiler installed on your computer.  
To install: https://rust-lang.org/tools/install/.

## Second step before to move forward
Open the terminal in project root directory and run:
````sh
cp .env.example .env
````
That command will copy `.env.example` into a new file with name `.env`.

## How to get your own API_KEY

To get your own API key, you need to:
* visit [https://openweathermap.org/](https://openweathermap.org/]) and sign in or register
* open [https://home.openweathermap.org/api_keys](https://home.openweathermap.org/api_keys) and create your own API key
* copy the value of API key and paste into `.env`-file. For example, in your `.env`-file you should see something like that `API_KEY=7d340c7a21`


## How to build and run the CLI tool
To build a project into a binary file, you need to open a terminal in the project's root folder and run the command:
````sh
cargo build --release
````
That command will run build process and requires some time to finish compiling the project.  
If you don't see any errors it means that compile process was successful.  
You will find your program file in `target/release/` directory with name `weather_rust`.  
To run the file you need to run the next command:
````sh
./weather_rust
````