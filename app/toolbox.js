const toolboxId = "toolbox";
const toolboxGridClass = "toolbox";
const toolboxGridSquareClass = "toolbox-square";

function Building(numInputs, numOutputs) {
    this.numInputs = numInputs;
    this.numOutputs = numOutputs;

    this.element = document.createElement("div");
    this.element.className = "building";
}
Building.prototype

function Tool(name) {
    this.name = name;

    this.eventEmitter = new EventEmitter();
}
Tool.events = {
    SelectTool: "tool-select",
};
Tool.prototype.place = function () {

}

function Toolbox(tools) {
    var toolbox = this;
    this.grid = new Grid(toolboxId, toolboxGridClass, toolboxGridSquareClass);
    this.tools = tools;

    this.eventEmitter = new EventEmitter();

    this.grid.on(Grid.events.Select, function (gridSquare) {
        var x = gridSquare.coordinate.x;
        // Emit the selected Tool.
        toolbox.eventEmitter.emit(Toolbox.events.SelectTool, toolbox.tools[x]);
    });
}
Toolbox.events = {
    SelectTool: "tool-select"
}
Toolbox.prototype.on = function (eventName, listener) {
    this.eventEmitter.on(eventName, listener);
};
Toolbox.prototype.load = function () {
    this.grid.generate(this.tools.length, 1);
}

