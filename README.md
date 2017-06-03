# httpsh - Execute system command through an API

## Run httpsh
1. Clone this repo
2. Run httpsh
```bash
cargo run
```

## Exemple

```bash
curl http://localhost:8000/ping
```

```bash
curl -H "Content-Type: application/json" -X POST -d '{"name": "ls"}' http://localhost:8000/cmd/exec
```

