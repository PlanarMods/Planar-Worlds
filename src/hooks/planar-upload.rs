fn planar_upload(
    game: &crate::game::Game<'_>,
    uploads: &mut Vec<((i32, i32, i32), horizon_client::mesher::SectionMesh)>,
) {
    crate::game::planar_upload::prepare(game, uploads);
}
