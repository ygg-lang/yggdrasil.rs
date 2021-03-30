//
// fn get_completion_word(text: String, tp: Position) -> String {
//     let line = tp.line as usize;
//     let num = tp.character as usize;
//     text.lines().nth(line).map(|e| get_word(e, num)).unwrap_or_default()
// }
//
// fn get_word(line: &str, index: usize) -> String {
//     // FIXME: panic when mis-sync!!!
//     let num = index.min(line.len());
//     let (f, e) = (&line[..num], &line[num..]);
//     let mut v = VecDeque::new();
//     for c in f.chars().rev() {
//         if c.is_xid_continue() || c == '\\' || c == '<' {
//             v.push_front(c)
//         }
//         else {
//             break;
//         }
//     }
//     for c in e.chars() {
//         if c.is_xid_continue() {
//             v.push_back(c)
//         }
//         else {
//             break;
//         }
//     }
//     return v.iter().collect();
// }
