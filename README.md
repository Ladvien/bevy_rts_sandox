## Bevy Tests
https://github.com/bevyengine/bevy/blob/main/tests/how_to_test_systems.rs

https://hugopeixoto.net/articles/rust-gamedev-ecs-bevy-p2.html

# RayCasting via Mouse
https://vaporsoft.net/getting-the-cursor-position-in-a-bevy-3d-game-using-rapier/

http://clynamen.github.io/blog/2021/01/04/terrain_generation_bevy/

https://github.com/aevyrie/bevy_mod_raycast

# Physics
https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy

# Bevy RTS Example
https://github.com/Escapingbug/bevy_rts_base/blob/master/src/systems/unit.rs

# TODO:
1. Raycast not intersect with certain objects.
2. Raycast not intersect (or ignore) sides of objects
3. Ensure loops aren't running unnecessarily.
4. Write a few bevy unit tests.
5. Implement FSM / Behavior Tree for animations, etc.
6. Implement logging and replace println calls with log calls.

## Worklog

### 2023-01-06
From what I can the `make_scene_pickable` method is actually running through _all_ entities, including the children of the container entity. This makes it so all sides of a scene are raycastable. Unfortunately, it's easy to make everything raycastable, but not select pieces of a scene.

The easiest solution seems to be using the `bevy_scene_hook` crate, which allows adding components to pieces of a scene by name.

https://github.com/nicopap/bevy-scene-hook

This means I'll also need to adjust my GTLF files with appropriate names for tagging parts of a scene with intersectable components.

### 2023-01-10
I've added checks in the rotation and movement system to remove the flagging component once an entity has reached its destination. For example, when a unit is told to go somewhere the `Destination` component is added. Once they reach their destination, with some approximation, the `Destination` component is removed to ensure the query excludes them from further computation.

#### Unit Tests
I realize I can take two approaches with this platform, one, aim for slow and robust, or quick and dirty. The first is what I am aiming for. I don't want to make spaghetti. As such, need some tests:

https://github.com/bevyengine/bevy/blob/main/tests/how_to_test_systems.rs


### 2023-01-11
Learning the words for stuff. Apparently, "flocking" is the term used for a group of units moving in sync. It consists of three components:

* Alignment
* Cohesion
* Separation

Here's an introductory article:
https://gamedevelopment.tutsplus.com/tutorials/3-simple-rules-of-flocking-behaviors-alignment-cohesion-and-separation--gamedev-3444

I was also watching the Ultima Online postmortem:
https://www.youtube.com/watch?v=lnnsDi7Sxq0

### 2023-01-12
Was working on placing units around cursor, evenly, in all four quadrants around the cursor.

Also, found this page. Pretty cool. Seems to walk through popular algorithms for game dev.
https://www.redblobgames.com/