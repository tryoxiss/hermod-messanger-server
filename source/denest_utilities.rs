enum Continue 
{
    Yes,
    No
}

fn safe_unwrap(item: Option) -> Option<Some<T>, None>
{
    match item
    {
        Some(T) => return item.unwrap(),
        None(E) => break
    }
}