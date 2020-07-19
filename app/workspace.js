const workspaceId = "workspace";
const workspaceGridClass = "workspace";
const workspaceGridSquareClass = "workspace-square";

function Workspace(width, height) {
    this.width = width;
    this.height = height;
    this.grid = new Grid(workspaceId, workspaceGridClass, workspaceGridSquareClass);
}

Workspace.prototype.load = function () {
    this.grid.generate(this.width, this.height);
}
