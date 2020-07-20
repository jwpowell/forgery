'use strict';

class HexMapRenderer {
    constructor(target) {
        READONLY(this, "target", target);
    }

    draw(layout, hexMap) {
        DEBUG && ASSERT_INSTANCE_OF(hexMap, HexMap);

        const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("viewBox", "-250 -250 500 500");

        const hexRenderer = new HexRenderer(svg);

        hexMap.forEach(function (hex) {
            hexRenderer.draw(layout, hex);
        });


        this.target.innerHTML = "";
        this.target.appendChild(svg);
    }
}

class HexRenderer {

    constructor(target) {
        READONLY(this, "target", target);
    }

    draw(layout, hex) {
        DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

        const g = document.createElement("g");
        const pixel = layout.hexToPixel(hex);
        //g.setAttribute("transform", "translate(" + pixel.x + "," + pixel.y + ")");

        const polygon = document.createElementNS("http://www.w3.org/2000/svg", "polygon");
        const corners = layout.polygonCorners(hex);
        let points = "";
        for (let i = 0; i < corners.length; ++i) {
            points += corners[i].x + "," + corners[i].y + " ";
        }
        polygon.setAttribute("points", points);
        polygon.setAttribute("style", "fill:lime;stroke:purple;stroke-width:1");

        const title = document.createElementNS("http://www.w3.org/2000/svg", "title");
        title.innerHTML = hex.hashCode();

        //polygon.setAttribute("transform", "translate(" + pixel.x + "," + pixel.y + ")");

        polygon.appendChild(title);


        this.target.appendChild(polygon);
    }

}


window.addEventListener('load', function () {

    const world = new HexMap(Boolean);
    world.generateHexgon(5, true);

    const view = document.getElementById("workspace");
    const renderer = new HexMapRenderer(view);

    const layout = new Layout(Orientation.pointy(), new Point(20, 20), Point.origin());
    renderer.draw(layout, world);

}, false);
