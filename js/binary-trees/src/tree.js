export class TreeNode {
    key;

    /**@type {TreeNode} */
    left = void 0;

    /**@type {TreeNode} */
    right = void 0;

    constructor({ key, left, right }) {
        if (key == null) {
            throw new Error('Key required');
        }

        this.key = key;
        this.left = left;
        this.right = right;
    }

    searchInternal(key, node) {
        if (key < this.key) {
            if (node.left) {
                this.searchInternal(key, node.left);
            }
        } else if (key > this.key) {
            if (node.right) {
                this.searchInternal(key, node.right);
            }
        } else {
            return node;
        }
    }

    search(key) {
        return this.searchInternal(key, this);
    }

    insert(key) {
        if (key < this.key) {
            if (this.left) {
                this.left.insert(key);
            } else {
                this.left = new TreeNode({ key });
            }

            return;
        }

        if (key > this.key) {
            if (this.right) {
                this.right.insert(key);
            } else {
                this.right = new TreeNode({ key });
            }
        }
    }

    remove(key, parent) {
        if (key < this.key) {
            if (this.left) {
                return this.left.remove(key, this);
            }
        } else if (key > this.key) {
            if (this.right) {
                return this.right.remove(key, this);
            }
        } else {
            if (this.left && this.right) {
                this.key = this.right.min().key;
                this.right.remove(this.key, this);
            } else if (parent.left === this) {
                parent.left = this.left ? this.left : this.right;
            } else if (parent.right === this) {
                parent.right = this.left ? this.left : this.right;
            }

            return true;
        }

        return false;
    }

    min() {
        if (this.left == null) {
            return this;
        } else {
            return this.left.min();
        }
    }
}

export class BinarySearchTree {
    /**@type {TreeNode} */
    root;

    constructor(root) {
        this.root = root;
    }

    search(key) {
        if (!this.root) {
            return void 0;
        }

        return this.root.search(key);
    }

    insert(key) {
        if (!this.root) {
            this.root = new TreeNode({ key });
            return;
        }

        this.root.insert(key);
    }

    remove(key) {
        if (!this.root) {
            return void 0;
        }

        this.root.remove(key);
    }
}
