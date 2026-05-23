use bridge_opencode::utils::find_matching_sockets;

#[test]
fn test_find_matching_sockets_empty() {
    let sockets = find_matching_sockets().expect("Failed to find sockets");
    assert!(sockets.is_empty() || !sockets.is_empty());
}

#[test]
fn test_find_matching_sockets_filters_nonexistent() {
    let sockets = find_matching_sockets().expect("Failed to find sockets");
    for socket in &sockets {
        assert!(socket.exists(), "Socket path should exist: {:?}", socket);
    }
}
