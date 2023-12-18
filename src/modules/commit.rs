mod functions;
mod views;

pub use views::commit_menu;

pub fn handle_commit(args: crate::Args) {
    match args.mode.as_str() {
        "commit" | "c" => {
            views::commit_menu(args);
            return;
        }
        "ca" => {
            views::commit_all_files(args);
            return;
        }
        "cf" => {
            views::commit_specific_files(args);
            return;
        }
        _ => {
            views::commit_menu(args);
            return;
        }
    }
}
