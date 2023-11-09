const G = 48271;
const M = 2 ** 31;

export function random(seed) {
    let state = seed;

    function next() {
        state = (state * G) % M;
        return state;
    }

    function reset() {
        state = seed;
    }

    return {
        next,
        reset,
    };
}
