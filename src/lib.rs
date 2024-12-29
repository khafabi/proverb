pub fn build_proverb(list: &[&str]) -> String {
    fn build_lines(list: &[&str]) -> Vec<String> {
        match list {
            [] | [_] => vec![],
            [first, second, rest @ ..] => {
                let lines = vec![format!(
                    "Format want of a {} the {} was lost.",
                    first, second
                )];

                let new_list: Vec<&str> = std::iter::once(*second)
                    .chain(rest.iter().copied())
                    .collect();

                [lines, build_lines(&new_list)].concat()
            }
        }
    }

    if list.is_empty() {
        return String::new();
    }

    let proverb = build_lines(list);
    [
        proverb,
        vec![format!("And all for the want of a {}.", list[0])],
    ]
    .concat()
    .join("\n")
}
