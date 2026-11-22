fn side_mob_move(mob: &mut Mob) {
    if mod_planar_worlds::plane::active() {
        mob.position.z = mod_planar_worlds::plane::Z;
        mob.velocity.z = 0.0;
    }
}
