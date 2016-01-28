var EntityManager = function (teamId, context) {
    this.teamId = teamId;
    this.context = context;
    this.env = context;
    this.entities = [];
    this.settlements = [];
    this.idlers = [];
    this.innerTargets = null;
    this.enemies = {

    };
};

EntityManager.prototype.findNearestSettlement = function (entity) {
    var i;
    var closest = this.settlements[0];
    for (i = 1; i < this.settlements.length; i++) {
        if (dist(entity, this.settlements[i]) < dist(entity, closest)) {
            closest = this.settlements[i];
        }
    }

    return closest;
};

EntityManager.prototype.isOnSettlement = function(entity) {
    var i;
    for (i = 0; i < this.settlements; i++) {
        var s = this.settlements[i];
        if (s.x === entity.x && s.y === entity.y) {
            entity.health = entity.maxHealth;
            return true;
        }
    }
    return false;
};

EntityManager.prototype.step = function() {
    var w = this.context.width;
    var h = this.context.height;

    if (this.innerTargets === null) {
        this.innerTargets = this.findTargets();
    }

    if (this.entities.length < 20) {
        this.settlements.forEach(function (s){
            s.saving = false;
        });
    }
    if ((this.entities.length >= 20 && this.settlements.length === 1) ||
        (this.entities.length >= 30 && this.settlements.length === 2) ||
        (this.settlements.length >= 3)) {
        this.settlements[this.settlements.length - 1].saving = true;
        if (this.settlements[this.settlements.length - 1].resources.WOOD >= 20 &&
            this.settlements[this.settlements.length - 1].resources.WATER >= 10) {
            var far = this.findUnitsAway(this.settlements[this.settlements.length - 1], 30);
            if(far.length !== 0) {
                var chosen = choose(far);
                var s = new Settlement(chosen.x, chosen.y, this.teamId, this.env);
                this.settlements.push(s);
                this.innerTargets = this.findTargets();
                this.settlements[0].saving = false;
            }
        }
    }

    this.idlers.forEach(function(e){
        if(e.carrying !== null) {
            var sett;
            if (Math.random() < 2/3){
                sett = this.findNearestSettlement(e);
            } else {
                sett = this.settlements[this.settlements.length - 1];
            }
            if (e.x === sett.x && e.y === sett.y) {
                sett.resources[e.carrying] ++;
                e.carrying = null;
            } else {
                e.path = path(e, sett);
                e.busy = true;
                return;
            }
        }

        var pos = choose(this.innerTargets);
        e.path = path(e, pos);
        e.target = pos.target;
        e.busy = true;
    }.bind(this));

    /*
    this.idlers.forEach(function(e){
        var pos = {x:0, y:0};
        do {
            pos.x = Math.floor(Math.random() * this.context.width);
            pos.y = Math.floor(Math.random() * this.context.height);
        } while (!this.map.isWalkable(pos));
        e.path = path(e, pos);
        e.busy = true;
    });
    */

    this.idlers.length = 0;

    this.entities = this.entities.filter(function (e){
        return e.health > 0;
    });

    this.entities.forEach(function (e){
        this.isOnSettlement(e);
        e.move();
    }.bind(this));

    this.settlements.forEach(function (s){
        if (s.canSpawn()) {
            this.entities.push(new Entity(s.x, s.y,
                    this.teamId, this.context, this));
        }
    }.bind(this));
    this.duel();
};

EntityManager.prototype.draw = function() {
    this.entities.forEach(function(e) {
        e.draw();
    });

    this.settlements.forEach(function(s){
        s.draw();
    });

    /*
    this.innerTargets.forEach(function(t){
        var c = this.context.canvas;
        c.fillStyle = TeamColors[this.teamId];
        c.save();
        c.globalAlpha = 0.5;
        c.fillRect(t.x * this.env.ww,
            t.y * this.env.hh,
            this.env.ww,
            this.env.hh);
        c.restore();
    }.bind(this));
    */
};

EntityManager.prototype.drawTween = function(x) {
    this.entities.forEach(function(e) {
        e.drawTween(x);
    });
    this.settlements.forEach(function(s){
        s.draw();
    });
};

EntityManager.prototype.enqueueIdler = function (e) {
    this.idlers.push(e);
};

EntityManager.prototype.findTargets = function (e) {
    var selected = false;
    var xMin, xMax, yMin, yMax;
    this.settlements.forEach(function(s) {
        if(!selected)  {
            xMin = s.x;
            xMax = s.x;
            yMin = s.y;
            yMax = s.y;
            selected = true;
        } else {
            if(s.x < xMin) { xMin = s.x; }
            if(s.x > xMax) { xMax = s.x; }
            if(s.y < yMin) { yMin = s.y; }
            if(s.y < yMax) { yMax = s.y; }
        }
    });

    var outshoot = Math.floor(this.env.map.w / 3);//25; this.env.map.w / 8;

    var startX = xMin - outshoot;
    var startY = yMin - outshoot;
    var endX   = xMax + outshoot;
    var endY   = yMax + outshoot;

    this.boundingRect = {
        'sx': startX,
        'sy': startY,
        'ex': endX,
        'ey': endY
    };

    var targets = [];

    for(var i = startX; i < endX; i++) {
        if(!this.env.map.tiles.contains(i, 0)) {
            continue;
        }

        for(var k = startY; k < endY; k++) {
            if(!this.env.map.tiles.contains(i, k) ||
               !this.env.map.tiles.contains(i, k + 1) ||
               !this.env.map.tiles.contains(i, k - 1)) {
                continue;
            }

            var isValid = function (x) {
                return x == this.env.map.TileTypes.BRUSH ||
                       x == this.env.map.TileTypes.WATER;
            }.bind(this)

            var on = this.env.map.tiles.get(i, k);
            var above = this.env.map.tiles.get(i, k + 1);
            var below = this.env.map.tiles.get(i, k - 1);
            //assert(false, "" + i + ", " + k);
            if(isValid(on)) {
                if(!isValid(above)) {
                    targets.push({x: i, y: k + 1, target: {x: i, y: k}});
                }
                if(!isValid(below)) {
                    targets.push({x: i, y: k - 1, target: {x: i, y: k}});
                }
            }
        }
    }
    return targets;
};

EntityManager.prototype.findUnitsNear = function (pos, radius) {
    var units = [];
    this.entities.forEach(function (e){
        if (dist(pos, e) <= radius)  {
            units.push(e);
        }
    });
    return units;
};

EntityManager.prototype.findUnitsAway = function (pos, radius) {
    var units = [];
    this.entities.forEach(function (e){
        if (dist(pos, e) >= radius)  {
            units.push(e);
        }
    });
    return units;
};

EntityManager.prototype.duel = function () {
    function getPowers(arr) {
        return arr.map(function (a){
            return a.strength;
        }).reduce(function (a,b){
            return a + b
        }, 0);
    }

    this.context.entityManagers.forEach(function (em) {
        if (this === em) return;
        if (this.entities.length == 0) return;
        //if (!this.enemies[em.teamId]) return;

        if (this.boundingRect) {
            em.entities.forEach(function (o) {
                if(o.x < this.boundingRect.ex &&
                   o.x > this.boundingRect.sx &&
                   o.y < this.boundingRect.ey &&
                   o.y > this.boundingRect.sy){

                    var nearbyUs = this.findUnitsNear(o, 2);
                    var nearbyThem = em.findUnitsNear(o, 2);
                    if (nearbyUs.length === 0) {
                        return;
                    }

                    var ourPower = getPowers(nearbyUs);
                    var theirPower = getPowers(nearbyThem);

                    var ourDamagePer = ourPower / nearbyThem.length;
                    nearbyThem.forEach(function (e){
                        e.health -= ourDamagePer;
                        e.path = path(e, em.findNearestSettlement(e));
                    }.bind(this));

                    var theirDamagePer = theirPower / nearbyUs.length;
                    nearbyUs.forEach(function (e){
                        e.health -= theirDamagePer;
                        e.path = path(e, this.findNearestSettlement(e));
                    }.bind(this));

                }
            }.bind(this));
        }
    }.bind(this));
};
