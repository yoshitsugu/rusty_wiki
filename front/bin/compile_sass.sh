#! /bin/bash

fswatch -0 static_src/css -i ".*\.scss$" -e ".*" | while IFS= read -r -d "" path
do
  echo "Re-building CSS (due to change in ${path})"
  sassc --style compressed \
    static_src/css/styles.scss \
    static/css/styles.css
done
