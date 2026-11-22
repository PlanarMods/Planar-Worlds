fn planar_remote_use(
    game: &mut crate::game::Game<'_>,
    hand: Hand,
) -> ::core::ops::ControlFlow<bool> {
    use crate::game::planar_side as side;
    use mod_planar_worlds::plane as plane;
    if !side::local(game) || plane::layer() == plane::BACKGROUND {
        return std::ops::ControlFlow::Continue(());
    }
    let background = side::target_layer(game) == plane::BACKGROUND;
    let solid = game.pick();
    if solid.is_some() && !background {
        return std::ops::ControlFlow::Continue(());
    }
    let hit = solid.or_else(|| {
        let stack = game.with_hand(hand, |g| g.held().cloned())?;
        game.items().get(&stack.item)?.block.as_ref()?;
        side::empty_target_hit(game)
    });
    if hit.is_none() {
        return if background {
            std::ops::ControlFlow::Break(true)
        } else {
            std::ops::ControlFlow::Continue(())
        };
    }
    let previous = game.tick_hit.replace(Some(hit));
    let used = if background {
        plane::background(|| game.remote_use_item_on(hand))
    } else {
        game.remote_use_item_on(hand)
    };
    game.tick_hit.set(previous);
    std::ops::ControlFlow::Break(used)
}
