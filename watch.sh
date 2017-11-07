platform='unknown'
unamestr=`uname`
if [[ "$unamestr" == 'Linux' ]]; then
   platform='linux'
elif [[ "$unamestr" == 'Darwin' ]]; then
   platform='macosx'
fi

if [[ "$platform" == "linux" ]]; then
  while inotifywait -qqre modify "src" "js"; do
    ./build-wasm.sh
  done
else
  fswatch -l 5 -o -x "src" "js" | while read; do
    ./build-wasm.sh
  done
fi
