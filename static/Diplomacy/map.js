function Map(w, h, ww, hh, canvas) {
    this.w = w;
    this.h = h;
    this.ww = ww;
    this.hh = hh;
    this.canvas = canvas;
    this.buffer = null;
    this.buffer2 = null;

    this.TileTypes = {
        GRASS: 0,
        STONE: 1,
        WATER: 2,
        DIRT: 3,
        BRUSH: 4,
    };


    this.ReverseTileTypes = {
        0: 'GRASS',
        1: 'STONE',
        2: 'WATER',
        3: 'DIRT',
        4: 'BRUSH'
    };

    this.TileArr =
        [ 'BRUSH', 'WATER', 'GRASS', 'DIRT', 'GRASS', 'BRUSH', 'WATER'];
    this.pValues = [  0.2,     0.6,     0.9,     1.1,     1.2,     2.0];

    this.ColorMaps = {
        GRASS: "green",
        STONE: "grey",
        DIRT: "rgb(70, 120, 60)",
        BRUSH: "rgb(30, 90, 30)",
        WATER: "darkblue"
    };

    this.tiles = new Grid(w, h);
    this.rand = new Grid(w, h);

    noise.seed(Math.random());
    var noiseFactor = 14;

    this.tiles.map(function (x, y) {
        var random = noise.perlin2(x / noiseFactor, y / noiseFactor, 0) + 1;
        var i, f;
        for (i = 0; i < this.pValues.length; i++) {
            if (random <= this.pValues[i]) {
                f = i;
                break;
            }
        }

        var ret = this.TileTypes[this.TileArr[f]];
        assert(ret !== undefined);
        return ret;
    }.bind(this));

    this.rand.map(function (x, y) {
        var rand = Math.random() / 3;
        var shade = Math.abs(noise.perlin2(x / noiseFactor, y / noiseFactor));

        var randFactor = 1/3;
        return rand * randFactor + shade * (1 - randFactor);

    });

}

Map.prototype.draw = function() {
    if(this.buffer === null) {
        var c = document.createElement('canvas');
        c.width = this.w * this.ww;
        c.height = this.h * this.hh;
        this.buffer = c;

        this.tiles.foreach(function(x, y, value) {
            var ctx = this.buffer.getContext('2d');
            ctx.fillStyle = this.ColorMaps[this.ReverseTileTypes[value]];
            ctx.fillRect(x * this.ww, y * this.hh, this.ww, this.hh);
        }.bind(this));
    } else {
        this.canvas.drawImage(this.buffer, 0, 0);
    }
}

Map.prototype.drawOverlay = function () {
    this.tiles.foreach(function(x, y, value) {
    }.bind(this));
};

Map.prototype.drawOverlay = function() {
    if(this.buffer2 === null) {
        var c = document.createElement('canvas');
        c.width = this.w * this.ww;
        c.height = this.h * this.hh;
        this.buffer2 = c;

        this.tiles.foreach(function(x, y, value) {
            var ctx = this.buffer2.getContext('2d');
            ctx.fillStyle = "rgba(0,0,0," + this.rand.get(x,y) + ")";
            ctx.fillRect(x * this.ww, y * this.hh, this.ww, this.hh);
        }.bind(this));
    } else {
        this.canvas.drawImage(this.buffer2, 0, 0);
    }
}

Map.prototype.isWalkable = function(p) {
    var v = this.tiles.get(p.x, p.y);
    return v !== this.TileTypes.WATER &&
           v !== this.TileTypes.BRUSH;
};
