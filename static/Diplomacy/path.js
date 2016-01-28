function sign(a) {
    assert(a === a, "sign(NaN)");
    assert(a !== Infinity, "sign(Inf)");
    assert(a !== -Infinity, "sign(-Inf)");

    if (a === 0) {
        return 0;
    } else if (a > 0) {
        return 1;
    } else {
        return -1;
    }
}

var pmap = null;
var pfgrid = null;
function setupPath(m) {
    pmap = m;
    pfgrid = new PF.Grid(m.w, m.h);
    pmap.tiles.map(function (x, y, v){
        if(!pmap.isWalkable({x:x,y:y})) {
            pfgrid.setWalkableAt(x, y, false);
        }
    });
}

function path(s, d) {
    var finder = new PF.AStarFinder({
        allowDiagonal: true
    });
    var path = finder.findPath(s.x, s.y, d.x, d.y, pfgrid.clone());
    return path;

    var sx = Math.floor(s.x),
        sy = Math.floor(s.y),
        dx = Math.floor(d.x),
        dy = Math.floor(d.y);
    var path = [];
    var cx = sx,
        cy = sy;

    while (cx !== dx || cy !== dy) {
        cx += sign(dx - cx);
        cy += sign(dy - cy);
        path.push([cx, cy]);
    }

    return path;
}
