pub fn short_type_name<T>() -> &'static str {
    let type_name = std::any::type_name::<T>();

    type_name
        .split('<')
        .next()
        .unwrap_or(type_name)
        .split("::")
        .last()
        .unwrap_or(type_name)
}
