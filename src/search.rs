use crate::{parser::PrintableLine, state::ApplicationState};

pub fn get_search_state(lines: &Vec<PrintableLine>, search: &str) -> ApplicationState {
    for (i, line) in lines.iter().enumerate() {
        if let PrintableLine::Text(line) = line {
            if line.contains(&search) {
                return ApplicationState::Search(Some(i), search.to_string());
            }
        }
    }

    return ApplicationState::Search(None, search.to_string());
}

pub fn get_next_search_state(
    lines: &Vec<PrintableLine>,
    state: ApplicationState,
) -> ApplicationState {
    let (current_line, search) = match state {
        ApplicationState::Search(Some(current_line), search) => (current_line, search),
        _ => unreachable!(),
    };

    for (i, line) in lines.iter().enumerate() {
        if i <= current_line {
            continue;
        }

        if let PrintableLine::Text(line) = line {
            if line.contains(&search) {
                return ApplicationState::Search(Some(i), search.to_string());
            }
        }
    }

    return ApplicationState::Search(None, search.to_string());
}
