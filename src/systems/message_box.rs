use crate::prelude::*;

#[system]
#[write_component(MessageBox)]
pub fn message_box(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let message = <&MessageBox>::query().iter(ecs).next();

    if let Some(message) = message {
        draw_batch.draw_box(
            Rect::with_size(DISPLAY_WIDTH - 10, DISPLAY_HEIGHT - 5, 20, 10),
            ColorPair::new(WHITE, BLACK),
        );

        draw_batch.print_centered(DISPLAY_HEIGHT - 2, " You found:");
        draw_batch.print_centered(DISPLAY_HEIGHT + 2, message.0.clone());
    }

    draw_batch.submit(15000).expect("Batch error");
}

#[system]
#[read_component(MessageBox)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn input(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    *turn_state = if matches!(key, Some(VirtualKeyCode::Return)) {
        let message_box = <Entity>::query()
            .filter(component::<MessageBox>())
            .iter(ecs)
            .next();
        if let Some(message_box) = message_box {
            commands.remove(*message_box);
        }

        TurnState::AwaitingInput
    } else {
        TurnState::MessageBox
    };
}
