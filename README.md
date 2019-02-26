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
[Coming soon]
