enum State {
    Birth(BirthState),
    Loading(LoadingState),
    Gaming(GamingState),
}

struct BirthState {
    a: i32,
}

struct LoadingState {
    a: i32,
}

struct GamingState {
    a: i32,
}
