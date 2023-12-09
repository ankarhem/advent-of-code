default_year := '2023'
work day year=default_year:
    cargo watch -x "nextest run --bin {{year}}_{{day}}"

solve day year=default_year:
    cargo solve {{year}} {{day}} --release

bench day year=default_year:
    cargo solve {{year}} {{day}} --release --time

just download day year=default_year:
    cargo download {{year}} {{day}}

init day year=default_year:
    cargo scaffold {{year}} {{day}}
    just download {{day}} {{year}}
