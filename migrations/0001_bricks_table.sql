create table sets (
    number varchar not null,
    theme varchar,
    sub_theme varchar,
    year varchar,
    set_name varchar,
    minifigs integer,
    pieces integer, 
    width integer,
    height integer,
    depth integer,
    launch_date varchar,
    exit_date varchar 
);

create unique index set_number_idx on sets (number);
