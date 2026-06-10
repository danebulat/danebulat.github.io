/// Creates a HashMap<String, Vec<String>> with concise syntax.
///
/// # Example
/// ```ignore
/// let map = hashmap! {
///     "software": "rust", "typescript", "sql",
///     "libraries": "leptos", "axum",
///     "devops": "docker", "postgres",
/// };
/// ```
#[macro_export]
macro_rules! hashmap {
    (
        $(
            $key:expr => [$($val:expr),* $(,)?]
        ),* $(,)?
    ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), vec![$($val.to_string()),*]);
            )*
            map
        }
    };
}
