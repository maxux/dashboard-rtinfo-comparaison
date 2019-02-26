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
- LoC stand for Lines of Codes
- Binary Size is the final binary, in release mode (if any) and stripped from debug symbols
- RAM is the memory usage at the beginin and after 128 iterations, memory usage is computed using VmRSS value
- User and Sys are time consumed to do 128 fetching iterations (not an infinite loop) on user space and kernel space,
with 0.1s of sleep between each loop

| Language | LoC     | Binary Size | RAM (5s)     | RAM (128)   | User CPU     | Sys CPU      |
|----------|---------|-------------|--------------|-------------|--------------|--------------|
| Python   | *16*    | /           | 29.80 MB     | 30.43 MB    | 0.818s       | 0.101s       |
| Rust     | **39**  | 3.4M        | 8.46 MB      | 11.36 MB    | 1.078s       | 0.122s       |
| Go       | 54      | 5.1M        | 8.95 MB      | 10.21 MB    | 0.381s       | 0.093s       |
| C        | 102     | **83K**     | **5.30 MB**  | **5.45 MB** | **0.328s**   | **0.070s**   |

- Rust: 1.32.0 linux/amd64
- Go: 1.11.5 linux/amd64
- C: gcc 7.3.0 linux/amd64
