fn side_mob_move(mob: &mut Mob) {
    if horizon_world::planar_plane::active() {
        mob.position.z = horizon_world::planar_plane::Z;
        mob.velocity.z = 0.0;
    }
}
