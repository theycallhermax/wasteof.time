# wasteof.time
An open-source wasteof.money frontend/backend implementation

## Running
### Backend
```console
cargo run -q -p backend
```
### Frontend
To run the frontend, you need `trunk`, and the WASM target added. See the [Yew tutorial](https://yew.rs/docs/tutorial) for more deatils.
```console
cd frontend
trunk watch
```

## Systemd Services
## Backend
```systemd
[Unit]
Description=wasteof.time Backend

[Service]
Restart=on-failure
RestartSec=5s
ExecStart=cd wasteof.time; cargo run -q -p backend

[Install]
WantedBy=default.target
```
## Frontend
```systemd
[Unit]
Description=wasteof.time Frontend

[Service]
Restart=on-failure
RestartSec=5s
ExecStart=cd wasteof.time/frontend; trunk serve

[Install]
WantedBy=default.target
```
