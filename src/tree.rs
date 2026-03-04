/// traverse tree returning reference, like yield in C#
pub fn traverse_tree_iter_ref<T, F>(root: &T, get_children: F) -> impl Iterator<Item = &T>
	where
		F: Fn(&T) -> Vec<&T>,
{
    let mut stack = vec![root];

    std::iter::from_fn(move || {
        if let Some(node) = stack.pop() {
            let children = get_children(node);
            for child in children.into_iter().rev() {
                stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    })
}
