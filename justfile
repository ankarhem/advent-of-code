default_year := '2023'
work year day:
    cargo watch -x "nextest run --bin {{year}}_{{day}}"

solve day year=default_year:
    cargo solve {{year}} {{day}} --release

bench day year=default_year:
    cargo solve {{year}} {{day}} --release --time

init day year=default_year:
    cargo scaffold {{year}} {{day}}
    cargo download {{year}} {{day}}
