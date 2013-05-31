while true; do
    inotifywait -e close_write -e moved_to -e create ./htmlsrc
    ./compile.sh
done
