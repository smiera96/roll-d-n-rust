# roll-d-n-rust
A Rust library for rolling dice 🎲 any number of times and any number of sides.

## Setup 📗
Check the .env.example file and create a .env file with the variable that you need, by default it iss set up for Docker 🐳

Example to run it with **Docker**:
```
// .env
HOST=0.0.0.0
PORT=8080
```

Example to run it **locally**:
```
// .env
HOST=127.0.0.1
PORT=8080
```

⚠️ Ensure that your port is not being used by another service.
Feel free to change both environment variables.

## Installation (with Docker) 🐳

### Linux 🐧
```bash
make start
```

⚠️ If `make start` fails try with:
```bash
docker compose up --build -d
```

### Windows 💻
```bash
docker compose up --build -d
```

## Installation (without Docker) 🚫🐳
### Windows & Linux (💻 & 🐧)
```bash
cargo build
cargo run
```

## Tests (TODO: 😅😅😅)
```bash
cargo test
```

## How to use
I recommend downloading Postman collections and environments (check the project).
These are the available endpoints:
```
[GET] /api/health-check
[GET] /api/dice/roll?times=1&sides=20
```

## Why I created this
The main motivation was to try rust, I knew it had good random generation and I thought it was a good exercise.
A friend of mine told me about a roll game he is developing and I wanted to help him with something simple like a dice generator.
So Mr. B. enjoy 😊

## Future improvements
I created this project as a test in a couple of days and using things I learned in my loved <a href="https://codely.com/">CodelyTv</a> courses.
Sorry if the code is not so nice or if the structure is not the best 🙏🙏🙏

## Postamn Collection
On the "Postman" folder you can find a collection and an environment to test the API.]()

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
<a href="https://opensource.org/licenses/MIT">MIT License</a>