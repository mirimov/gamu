run:
    build/game.out

lint:
    clang-tidy src/main.c  --  -I./include -std=c23

build: lint
    meson compile -C build

setup:
    meson setup build
    ln -s build/compile_commands.json .

clean:
    rm -rf build/ builddir