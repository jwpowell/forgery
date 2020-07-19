const workspaceId = "workspace";
const workspaceGridClass = "workspace";
const workspaceGridSquareClass = "workspace-square";


function BeltSegment() {
    this.element = document.createElement("div");
    this.element.className = "belt";
    this.setStatus(false);
}
BeltSegment.prototype.setStatus = function (enabled) {
    if (enabled) {
        this.element.classList.remove("belt-empty");
        this.element.classList.add("belt-full");
    } else {
        this.element.classList.add("belt-empty");
        this.element.classList.remove("belt-full");
    }
};
BeltSegment.prototype.place = function (coordinate) {
    this.coordinate = coordinate;
};
BeltSegment.prototype.draw = function (grid) {
    gridSquare = grid.getGridSquare(this.coordinate);
    gridSquare.element.appendChild(this.element);
};

function BeltView(belt) {
    this.belt = belt;

    this.beltSegments = [];
};
BeltView.prototype.place = function () {
    const enabledSegments = {};
    console.debug(this.belt.contents.length());
    for (var i = 0; i < this.belt.contents.length(); ++i) {
        const supplyTime = this.belt.contents.elements[i].supplyTime;
        const segmentIndex = this.belt.capacity - (supplyTime - this.belt.clock.time);
        console.debug("segmentIndex: " + segmentIndex);
        enabledSegments[segmentIndex] = true;
    }

    for (var i = 0; i < this.belt.capacity; ++i) {
        const beltSegment = new BeltSegment();
        beltSegment.place(new Coordinate(10 + i, 70));

        if (enabledSegments[i] == true) {
            beltSegment.setStatus(true);
        }

        this.beltSegments.push(beltSegment);
    }
};
BeltView.prototype.draw = function (grid) {
    for (var i = 0; i < this.beltSegments.length; ++i) {
        const beltSegment = this.beltSegments[i];
        beltSegment.draw(grid);
    }
};

function BuildingView(building) {
    this.building = building;

    this.element = document.createElement("div");
    this.element.className = "building";
    this.setStatus(false);
}
BuildingView.prototype.setStatus = function (enabled) {
    if (enabled) {
        this.element.classList.remove("building-idle");
        this.element.classList.add("building-working");
    } else {
        this.element.classList.add("building-idle");
        this.element.classList.remove("building-working");
    }
};
BuildingView.prototype.place = function (coordinate) {
    this.coordinate = coordinate;
};
BuildingView.prototype.draw = function (grid) {
    this.setStatus(this.building.status());
    gridSquare = grid.getGridSquare(this.coordinate);
    gridSquare.element.appendChild(this.element);
};
BuildingView.prototype.delete = function () {
    this.element.remove();
};

function SourceView(source) {
    this.source = source;

    this.element = document.createElement("div");
    this.element.className = "source";
}
SourceView.prototype.place = function (coordinate) {
    this.coordinate = coordinate;
};
SourceView.prototype.draw = function (grid) {
    gridSquare = grid.getGridSquare(this.coordinate);
    gridSquare.element.appendChild(this.element);
};



function Workspace(width, height) {
    this.width = width;
    this.height = height;
    this.grid = new Grid(workspaceId, workspaceGridClass, workspaceGridSquareClass);
}
Workspace.prototype.load = function () {
    this.grid.generate(this.width, this.height);
};
Workspace.prototype.draw = function (drawable) {
    drawable.draw(this.grid);
};