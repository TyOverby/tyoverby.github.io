
var ce = document.querySelector("canvas");
var canvas = ce.getContext("2d");

ce.width = 1000;
ce.height = 1000;

var factor = 10;

var map = new Map(ce.width / factor,
                  ce.height / factor,
                  factor, factor, canvas);

setupPath(map);

var entityManagers = [];

var context = {
    map: map,
    canvas: canvas,
    width: map.tiles.width,
    height: map.tiles.height,
    ww: factor,
    hh: factor,
    entityManagers: entityManagers
};

function setText(id, type, value) {
    var ele = document.querySelector("#"+ id+">."+type);
    ele.innerText = value;
}

var dummy = ["a", "b", "c", "d"];

var frame = 0;
var tweenLevel = 3;
var loop = function () {
    frame ++;
    if (frame % tweenLevel === 0) {
        map.draw();
        map.drawOverlay();
        entityManagers.forEach(function (em){
            em.step();
            em.draw();
        });

        var i;
        for(i = 0; i < 4; i++) {
            var em = entityManagers[i];
            var team = dummy[em.teamId - 1];
            setText(team, "total", em.entities.length);

            setText(team, "avgh", (em.entities.map(function(e){
                return e.health;
            }).reduce(function (a, b){
                return a + b;
            }, 0) / em.entities.length).toFixed());

            setText(team, "avgp", (em.entities.map(function(e){
                return e.strength;
            }).reduce(function (a, b){
                return a + b;
            }, 0) / em.entities.length).toFixed());
        }
    } else {
        map.draw();
        map.drawOverlay();
        entityManagers.forEach(function (em){
           em.drawTween((frame % tweenLevel) / tweenLevel);
        });
    }

    window.requestAnimationFrame(loop);
}.bind(this);


var positions = {
    1: [10, 10],
    2: [context.width - 10, 10],
    3: [10, context.height - 10],
    4: [context.width - 10, context.height - 10]
};

var i;
for (i = 1; i <= 4; i++) {
    var em = new EntityManager(i, context);
    var pos = positions[i];
    var s = new Settlement(pos[0], pos[1], i, context);
    while (!map.isWalkable(s)) {
        s.x++;
        s.y++;
    }
    s.resources['WATER'] = 8 * 10;
    s.resources['WOOD'] = 8 * 10;
    em.settlements.push(s);
    entityManagers.push(em);
}

loop();
