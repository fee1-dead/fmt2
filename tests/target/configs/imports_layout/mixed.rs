// rustfmt-imports_indent: Block
// rustfmt-imports_layout: Mixed

use comment::{FindUncommented, contains_comment, recover_comment_removed, rewrite_comment};
use lists::{
    DefinitiveListTactic, ListFormatting, ListItem, ListTactic, SeparatorTactic, definitive_tactic,
    itemize_list, shape_for_tactic, struct_lit_formatting, struct_lit_shape, struct_lit_tactic,
    write_list,
};
