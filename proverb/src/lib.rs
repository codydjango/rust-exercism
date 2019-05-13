pub fn build_proverb(list: &[&str]) -> String {
    let mut strings: Vec<String> = vec![];
    let mut mylist: Vec<&str> = vec![""; list.len()];

    mylist.copy_from_slice(list);
    mylist.reverse();

    let mut want: &str;
    
    // short-circuit if list is empty
    let mut lost = match mylist.pop() {
        None => return String::from(""),
        Some(x) => x,
    };

    while mylist.len() > 0 {
        want = lost;
        lost = mylist.pop().unwrap();
        strings.push(format!("For want of a {want} the {lost} was lost.", want=want, lost=lost));
    };

    strings.push(String::from("And all for the want of a nail."));

    return strings.join("\n");
}
