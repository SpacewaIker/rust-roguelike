use crate::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum SelectedButton {
    Play,
    About,
}

impl ToString for SelectedButton {
    fn to_string(&self) -> String {
        match self {
            Self::Play => String::from("Play"),
            Self::About => String::from("About"),
        }
    }
}

#[system]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
pub fn render(#[resource] selected_button: &mut SelectedButton) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let title = r#"
___________________________ _        _______  
\__   __/\__   __/\__   __/( \      (  ____ \ 
   ) (      ) (      ) (   | (      | (    \/ 
   | |      | |      | |   | |      | (__     
   | |      | |      | |   | |      |  __)    
   | |      | |      | |   | |      | (       
   | |   ___) (___   | |   | (____/\| (____/\ 
   )_(   \_______/   )_(   (_______/(_______/ 

  _______  _______  _______  _______  _______  _        
 (  ____ \(  ____ \(  ____ )(  ____ \(  ____ \( (    /| 
 | (    \/| (    \/| (    )|| (    \/| (    \/|  \  ( | 
 | (_____ | |      | (____)|| (__    | (__    |   \ | | 
 (_____  )| |      |     __)|  __)   |  __)   | (\ \) | 
       ) || |      | (\ (   | (      | (      | | \   | 
 /\____) || (____/\| ) \ \__| (____/\| (____/\| )  \  | 
 \_______)(_______/|/   \__/(_______/(_______/|/    )_) 
"#;

    for (y, line) in title.lines().enumerate() {
        draw_batch.print_centered(2 + y, line);
    }

    // buttons
    let buttons = [SelectedButton::Play, SelectedButton::About];
    for (y, button) in buttons.iter().enumerate() {
        let color = if button == selected_button {
            ColorPair::new(YELLOW, BLACK)
        } else {
            ColorPair::new(WHITE, BLACK)
        };

        draw_batch.draw_box(
            Rect::with_size(DISPLAY_WIDTH - 6, 28 + 5 * y as i32, 11, 4),
            color,
        );
        draw_batch.print_color_centered(30 + 5 * y, button.to_string(), color);
    }

    draw_batch.submit(0).expect("Batch error");
}

#[system]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn input(
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] selected_button: &mut SelectedButton,
    #[resource] turn_state: &mut TurnState,
) {
    #[allow(clippy::enum_glob_use)]
    use VirtualKeyCode::*;

    #[allow(clippy::match_same_arms)]
    if let Some(key) = *key {
        match key {
            Up | W => {
                *selected_button = match *selected_button {
                    SelectedButton::Play => SelectedButton::Play,
                    SelectedButton::About => SelectedButton::Play,
                }
            }
            Down | S => {
                *selected_button = match *selected_button {
                    SelectedButton::Play => SelectedButton::About,
                    SelectedButton::About => SelectedButton::About,
                }
            }
            Return => match *selected_button {
                SelectedButton::Play => {
                    *turn_state = TurnState::AwaitingInput;
                }
                SelectedButton::About => {
                    todo!()
                }
            },
            _ => {}
        }
    }
}
