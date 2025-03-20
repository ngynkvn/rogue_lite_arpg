graph:
  cargo run -F debugdump -- dump-schedule update -o update-schedule.dot
  sed -i '' -e 's/table>>\"/table>>/' \
    -e 's/\"<<table/<<table/' \
    -e 's/\\\"/\"/g' \
  update-schedule.dot 
  dot -Tsvg update-schedule.dot -o update-schedule.svg
