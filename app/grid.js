function Coordinate(x, y) {
    this.x = x;
    this.y = y;
}
Coordinate.prototype.key = function () {
    return this.x + "," + this.y;
};

function GridSquare(grid, coordinate) {
    var gridSquare = this;

    this.grid = grid;
    this.coordinate = coordinate;

    this.element = document.createElement("div");
    this.element.id = grid.rootId + "-" + coordinate.x + "-" + coordinate.y;
    this.element.className = grid.itemClassName;
    this.element.onclick = function (data) {
        gridSquare.grid.eventEmitter.emit(Grid.events.Select, gridSquare);
    }
}

function Grid(rootId, gridClassName, itemClassName) {
    this.rootId = rootId;
    this.gridClassName = gridClassName;
    this.itemClassName = itemClassName;

    this.gridSquares = {};

    this.root = document.getElementById(this.rootId);

    this.eventEmitter = new EventEmitter();
};
Grid.events = {
    Select: "grid-square-select",
};
Grid.prototype.on = function (eventName, listener) {
    this.eventEmitter.on(eventName, listener);
};
Grid.prototype.generate = function (width, height) {
    console.debug("grid.generate(" + width + ", " + height + ")");

    var grid = document.createElement("div");
    grid.className = this.gridClassName;
    grid.style.gridTemplateColumns = "repeat(" + width + ", 1fr)";

    for (hIndex = height - 1; hIndex >= 0; --hIndex) {
        for (wIndex = 0; wIndex <= width - 1; ++wIndex) {
            const coordinate = new Coordinate(wIndex, hIndex);
            var gridSquare = new GridSquare(this, coordinate);
            this.gridSquares[coordinate.key()] = gridSquare;
            grid.appendChild(gridSquare.element);
        }
    }

    this.root.innerHTML = "";
    this.root.appendChild(grid);
};
Grid.prototype.getGridSquare = function (coordinate) {
    return this.gridSquares[coordinate.key()];
};
