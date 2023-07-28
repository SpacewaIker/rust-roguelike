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
                                                     ..                      
 the                                           x .d88"                  s    
                                                5888R                  :8    
      u        ..    .     :       x.    .      '888R        .u       .88    
   us888u.   .888: x888  x888.   .@88k  z88u     888R     ud8888.    :888ooo 
.@88 "8888" ~`8888~'888X`?888f` ~"8888 ^8888     888R   :888'8888. -*8888888 
9888  9888    X888  888X '888>    8888  888R     888R   d888 '88%"   8888    
9888  9888    X888  888X '888>    8888  888R     888R   8888.+"      8888    
9888  9888    X888  888X '888>    8888  888R     888R   8888L        8888    
9888  9888    X888  888X '888>    8888 ,888B .  .888B . '8888c. .+  .8888Lu= 
"888*""888"  "*88%""*88" '888!`  "8888Y 8888"   ^*888%   "88888%    ^%888*   
 ^Y"   ^Y'     `~    "    `"`     `Y"   'YP       "%       "YP'       'Y"    
                     .                                                       
         .x+=:.     @88>              of                         oec :       
        z`    ^%    %8P                                         @88888       
           .   <k    .       .u    .      .u    .        .u     8"*88%       
         .@8Ned8"  .@88u   .d88B :@8c   .d88B :@8c    ud8888.   8b.          
       .@^%8888"  ''888E` ="8888f8888r ="8888f8888r :888'8888. u888888>      
      x88:  `)8b.   888E    4888>'88"    4888>'88"  d888 '88%"  8888R        
      8888N=*8888   888E    4888> '      4888> '    8888.+"     8888P        
       %8"    R88   888E    4888>        4888>      8888L       *888>        
        @8Wou 9%    888&   .d888L .+    .d888L .+   '8888c. .+  4888         
      .888888P`     R888"  ^"8888*"     ^"8888*"     "88888%    '888         
      `   ^"F        ""       "Y"          "Y"         "YP'      88R         
                                                                 88>         
                                                                 48          
                                                                 '8          
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
