# THIS FILE DEFINES YOUR MAIN FUNCTION'S NAMESPACE

fn main() -> Status {
    println("Hello, World!")
}

data World = ()

impl World {
    // system fn's are called on all instances
    system fn globalise() { println("The world just got bigger") }
}

@test
fn globalise_world() {
    let old_world = World()
    let new_world = World()
    World::globalise()
}
