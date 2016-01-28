function assert(cond, msg) {
    if (!cond) {
        throw msg;
    }
}

function dist(a, b) {
    return Math.abs(a.x - b.x) + Math.abs(a.y - b.y);
}

function choose(arr){
    return arr[Math.floor(Math.random() * arr.length)];
}

function Grid(width, height) {
    this.width = width;
    this.height = height;

    this.grid = [];
    this.grid.length = width * height;

    var i;
    for (i = 0; i < this.grid.length; i++) {
        this.grid[i] = 0;
    }
}

Grid.prototype.get = function (x, y) {
    assert(x < this.width && x >= 0, "0 <= x < width");
    assert(y < this.height && y >= 0, "0 <= x < height");
    return this.grid[y * this.width + x];
};

Grid.prototype.set = function (x, y, v) {
    assert(x < this.width && x >= 0, "0 <= x < width");
    assert(y < this.height && y >= 0, "0 <= x < height");
    this.grid[y * this.width + x] = v;
    return v;
};

Grid.prototype.contains = function (x, y) {
    return x >= 0 && y >= 0 && x < this.width && y < this.height;
};

Grid.prototype.foreach = function (f) {
    var i, k;
    for (i = 0; i < this.width; i++) {
        for (k = 0; k < this.height; k++) {
            f(i, k, this.get(i, k));
        }
    }
};

Grid.prototype.map = function(f) {
    var i, k;
    for (i = 0; i < this.width; i++) {
        for (k = 0; k < this.height; k++) {
            var value = f(i, k, this.get(i, k));
            if (value !== undefined && value !== null) {
                this.set(i, k, value);
            }
        }
    }
};
