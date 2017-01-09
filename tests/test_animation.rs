extern crate green_moon;

use green_moon::{ANIMATE_NONE, ANIMATE_ONCE, ANIMATE_LOOP, ANIMATE_PING_PONG};
use green_moon::AnimationBuilder;

#[test]
fn test_animate_none() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];

    // ANIMATE_NONE is the default
    let mut animation = AnimationBuilder::new()
        .set_frames(frames)
        .build().unwrap();

    // Should not advance since we have ANIMATE_NONE
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(50);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(200);
    assert_eq!(animation.get_sprite_index(), 20);
}

#[test]
fn test_animate_none2() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];

    // Set ANIMATE_NONE explicitply
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_NONE)
        .set_frames(frames)
        .build().unwrap();

    // Should not advance since we have ANIMATE_NONE
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(50);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(200);
    assert_eq!(animation.get_sprite_index(), 20);
}

#[test]
fn test_animate_once() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_ONCE)
        .set_frames(frames)
        .build().unwrap();

    // Elapsed time < frame duration, so should not advance
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 20);

    // Now the animation should move on
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(50);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 21);

    // Now the animation should move on
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);

    // End of animation reached, only the last frame should be shown from now on
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(200);
    assert_eq!(animation.get_sprite_index(), 22);
}

#[test]
fn test_animate_once_single_rame() {
    let frames = vec![(20, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_ONCE)
        .set_frames(frames)
        .build().unwrap();

    // Elapsed time < frame duration, so should not advance
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 20);

    // Now the animation should move on, but last frame already reached
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(50);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(200);
    assert_eq!(animation.get_sprite_index(), 20);
}

#[test]
fn test_animate_loop() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_LOOP)
        .set_frames(frames)
        .build().unwrap();

    // Elapsed time < frame duration, so should not advance
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 20);

    // Now the animation should move on
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(50);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 21);

    // Now the animation should move on
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);

    // End of animation reached, start from beginning
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(200);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);
}

#[test]
fn test_animate_loop_single_rame() {
    let frames = vec![(20, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_LOOP)
        .set_frames(frames)
        .build().unwrap();

    // Elapsed time < frame duration, so should not advance
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 20);

    // Now the animation should move on, but last frame already reached
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(50);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(200);
    assert_eq!(animation.get_sprite_index(), 20);
}

#[test]
fn test_animate_ping_pong() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_PING_PONG)
        .set_frames(frames)
        .build().unwrap();

    // Elapsed time < frame duration, so should not advance
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 20);

    // Now the animation should move on
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(50);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(99);
    assert_eq!(animation.get_sprite_index(), 21);

    // Now the animation should move on
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);

    // End of animation reached, move backwards
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(0);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(200);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 22);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 21);
    animation.next(100);
    assert_eq!(animation.get_sprite_index(), 20);
}
