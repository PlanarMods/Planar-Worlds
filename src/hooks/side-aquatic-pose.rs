fn side_aquatic_pose(mob: &Mob) -> ::core::ops::ControlFlow<Option<glam::Mat4>> {
    if mod_planar_worlds::plane::active()
        && matches!(
            mob.species(),
            Species::Squid
                | Species::GlowSquid
                | Species::Cod
                | Species::Salmon
                | Species::TropicalFish
        )
    {
        std::ops::ControlFlow::Break(None)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
