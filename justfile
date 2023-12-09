default_year := '2023'
work day year=default_year:
    cargo watch -x "nextest run --bin {{year}}_{{day}}"

solve day year=default_year:
    cargo solve {{year}} {{day}} --release

bench day year=default_year:
    cargo solve {{year}} {{day}} --release --time

download day year=default_year:
    cargo download {{year}} {{day}}

init day year=default_year:
    cargo scaffold {{year}} {{day}}
    cargo download {{year}} {{day}}

fix:
    cargo clippy --fix -- -D warnings
    cargo fmt
