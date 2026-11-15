fn planar_tree_feature(
    value: &RawValue,
    depth: usize,
    work: &mut Expansion,
) -> ::core::ops::ControlFlow<Result<String, DecodeError>> {
    if !crate::planar_plane::generating() {
        return std::ops::ControlFlow::Continue(());
    }
    let kind = serde_json::from_str::<Object>(value.get())
        .ok()
        .and_then(|object| {
            serde_json::from_str::<String>(required(&object, "type").ok()?.get()).ok()
        });
    if kind.as_deref().is_some_and(|kind| {
        matches!(
            kind.strip_prefix("minecraft:").unwrap_or(kind),
            "tree" | "fallen_tree" | "root_system"
        )
    }) {
        std::ops::ControlFlow::Break(
            work.visit(depth)
                .map(|()| r#"{"type":"minecraft:no_op","config":{}}"#.into()),
        )
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
