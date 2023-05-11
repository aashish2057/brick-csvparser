create table sets (
    number varchar not null,
    theme varchar,
    subtheme varchar,
    year varchar,
    set_name varchar,
    minifigs integer,
    pieces integer, 
    width integer,
    height integer,
    depth integer,
    launch_date date,
    exit_date date
);

create unique index set_number_idx on sets (number);
