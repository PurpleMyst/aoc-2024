macro_rules! doit {
    ($($day:ident: $solve:ident),+$(,)?) => {
        $(use $day::solve as $solve;)+
        iai::main!($($solve),+);
    };
}

doit!(
    day01: day01_solve,
    day02: day02_solve,
);
