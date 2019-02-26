# dashboard-rtinfo-comparaison
This repository is kind of personal comparaison of basic job needed for my dashboard, in multiple langage to compare them.

The original code is the python one, sligthly modified to fit the example of the others test.

# Workflow
The workflow is quite simple:
1. Download rtinfo json from rtinfo server, via HTTP
2. Print the amount of hosts found on the json (length of an array)
3. Create a new json adapted to dashboard format, encapsulate rtinfo inside this one
4. Send this json to a redis channel

The adapted json is just `{"id": "rtinfo", "payload": {rtinfo-original-object}}` which is needed by the dashboard server.

# But why ?
The original project was made in full python, single process single threaded (but async). Recent changes made this more 
modulable and now, some part of the project can even be written in another language than python, since there is a redis
channel in between clients and server.

The rtinfo module was quite simple (see the workflow), and I wanted to test Rust. This was a good opportunity to me
to test a little bit some possibilities in Rust.

When the rust project was working, I wanted to compare it with language I use more often. I wanted to compare amount of lines,
binary size, amount of the cpu usage and memory usage for the same job. This is why this repository exists.

# Results
- Loc stand for Lines of Codes
- Binary Size is the final binary, in release mode (if any) and stripped from debug symbols
- RAM is the memory usage after some amount of time, memory usage is computed using VmRSS value
- User and Sys are time consumed to do 32 fetching iterations (not an infinite loop) on user space and kernel space

| Language | LoC | Binary Size | RAM (5s) | RAM (6h) | User CPU | Sys CPU  |
|----------|-----|-------------|----------|----------|----------|----------|
| Python   | 16  | /           | 30 MB    | /        | 0.344s   | 0.037s   |
| Rust     | 39  | 3.4M        | 8.41 MB  | /        | 0.241s   | 0.032s   |
| Go       | 54  | 5.1M        | 8.95 MB  | /        | 0.090s   | 0.030s   |
| C        | 102 | 83K         | 5.28 MB  | /        | 0.133s   | 0.039s   |

- Rust: 1.32.0
- Go: 1.11.5 linux/amd64
- C: gcc 7.3.0 linux/amd64
