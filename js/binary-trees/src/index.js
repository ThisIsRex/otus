import { random } from './random.js';
import { BinarySearchTree } from './tree.js';

const SEED = 0x1234;

{
    const tree = new BinarySearchTree();

    [10, 35, 67, 5, 42, 3, 6].forEach((num) => {
        tree.insert(num);
    });

    //console.log(JSON.stringify(tree, null, 4));

    tree.remove(5);

    //console.log(JSON.stringify(tree, null, 4));
}

function seq() {
    let state = 0;

    function next() {
        return state++;
    }

    function reset() {
        state = 0;
    }

    return {
        next,
        reset,
    };
}

const N = 10000;
const NN = N / 10;

function test(gen, desc) {
    const tree = new BinarySearchTree();

    let start = performance.now();

    for (let i = 0; i < N; i++) {
        tree.insert(gen.next());
    }

    console.log(`${desc} add time, ms`, performance.now() - start);

    //search
    gen.reset();

    //skip
    for (let i = 0; i < NN; i++) {
        gen.next();
    }

    start = performance.now();

    for (let i = 0; i < NN; i++) {
        tree.search(gen.next());
    }

    console.log(`${desc} search time, ms`, performance.now() - start);

    //remove
    start = performance.now();

    for (let i = 0; i < NN; i++) {
        tree.remove(gen.next());
    }

    console.log(`${desc} remove time, ms`, performance.now() - start);
}

test(random(SEED), 'random');

test(seq(), 'seq');
