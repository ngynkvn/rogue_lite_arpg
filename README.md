![image](https://github.com/user-attachments/assets/8e19637c-eb1f-4170-b202-663c1434e073)

# Dump Schedule

```bash

cargo run -F dump -- --help
cargo run -F dump -- dump-schedule Update -o update-schedule.dot && \
  dot -Tsvg update-schedule.dot -o update-schedule.svg
```
