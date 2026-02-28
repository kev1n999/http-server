# Http Server
It is a simple http server written in rust for my studies about http protocol

---

# How to use
1. Clone the repository 
```
git clone https://github.com/kev1n999/http-server 
```
2. Open the project 
```
cd http-server
```
3. Build the project
```
cargo build
``` 
4. Create a environment variable to server port
```
export SERVER_PORT=8000 
```
5. Run 
```
cargo run
```

--- 

# Available Routes
```
-----------------------------------
| GET /calculator                 |
----------------------------------- 
| POST /create-people             |
| Content-Type: application/json  |
| BODY: {                         |
|   name: "person name",          |
|   age: "17",                    | 
|  }                              |
-----------------------------------
```
