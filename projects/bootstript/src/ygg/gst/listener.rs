// fn traverse(&self) -> Result<()> {
//     let cursor = &mut self.tree.walk();
//     self.visitor.enter_node(&cursor.node())?;
//     let mut recurse = true;
//     loop {
//         if recurse && cursor.goto_first_child() {
//             recurse = self.visitor.enter_node(&cursor.node())?;
//         }
//         else {
//             self.visitor.leave_node(&cursor.node())?;
//             if cursor.goto_next_sibling() {
//                 recurse = self.visitor.enter_node(&cursor.node())?;
//             }
//             else if cursor.goto_parent() {
//                 recurse = false;
//             }
//             else {
//                 break;
//             }
//         }
//     }
//     Ok(())
// }