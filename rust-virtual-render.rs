// initialize macro to better work with dictionaries
macro_rules! Props(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(m.insert($key, $value);)+m
        }
     };
);

// the above allows us to do
// Node(1, Props!{"state" => "one"}, ["Text"]);
// instead of
// let mut Props = ::std::collections::HashMap::new();
// Props.insert("state", "one")