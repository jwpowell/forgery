const workspaceWidth = 100;
const workspaceHeight = 100;

const tools = [
    new Tool("Belt"),
    new Tool("Buffer"),
    new Tool("Splitter"),
    new Tool("Merger"),
    new Tool("Source")
];

window.addEventListener('load', function () {
    const forgery = new Forgery();
    forgery.setup();
    forgery.run();
}, false);

function Queue() {
    this.elements = [];
}
Queue.prototype.enqueue = function (element) {
    this.elements.push(element);
};
Queue.prototype.dequeue = function () {
    return this.elements.shift();
};
Queue.prototype.isEmpty = function () {
    return this.elements.length == 0;
};
Queue.prototype.peek = function () {
    return !this.isEmpty() ? this.elements[0] : null;
};
Queue.prototype.peekBack = function () {
    return !this.isEmpty() ? this.elements[this.elements.length - 1] : null;
};
Queue.prototype.length = function () {
    return this.elements.length;
};
Queue.prototype.forEach = function (func) {
    this.elements.forEach(func);
}

function Clock() {
    this.time = 0;
    this.eventEmitter = new EventEmitter();

    const clock = this;
    this.ticker = setInterval(function () {
        clock.tick();
    }, 1000);
}
Clock.events = {
    Second: "clock-second",
    Minute: "clock-minute",
    Hour: "clock-hour",
}
Clock.prototype.on = function (eventName, listener) {
    this.eventEmitter.on(eventName, listener);
};
Clock.prototype.tick = function () {
    ++this.time;

    this.eventEmitter.emit(Clock.events.Second)

    if (this.time % 60 == 0) {
        this.eventEmitter.emit(Clock.events.Minute)
    }

    if (this.time % 3600 == 0) {
        this.eventEmitter.emit(Clock.events.Hour)
    }

    console.debug("time: " + this.time);
};

function Material(name) {
    this.name = name;
}
Material.prototype.clone = function () {
    return new Material(this.name);
}

// Material that is on a Belt.
// Material have to navigate the whole Belt before it can be supplied.
function BeltMaterial(material, supplyTime) {
    // The time the material will be supplied.
    // Time maps to Belt capacity. 
    // The higher the capacity the longer it takes for material to be transported.
    this.supplyTime = supplyTime;
    this.material = material;
}
BeltMaterial.prototype.ready = function (time) {
    this.supplyTime <= time ? true : false;
}

function Belt(clock, capacity) {
    this.clock = clock;
    this.capacity = capacity;
    this.contents = new Queue();

    this.eventEmitter = new EventEmitter();

    this.clock.on(Clock.events.Second, this.run.bind(this));
}
Belt.events = {
    MaterialReady: 'belt-material-ready'
};
Belt.prototype.on = function (eventName, listener) {
    this.eventEmitter.on(eventName, listener);
};
Belt.prototype.run = function () {
    if (this.isMaterialReady) {
        this.eventEmitter.emit(Belt.events.MaterialReady);
    }
};
Belt.prototype.isFull = function () {
    return this.contents.length < this.capacity
};
Belt.prototype.canConsume = function (time) {
    // A belt can only consume one material per tick.
    const material = this.contents.peekBack();
    if (material != null && material.supplyTime != time) {
        return true;
    }
    return false;
};
Belt.prototype.consume = function (supplier) {
    // Check if the belt is full or has already consumed this time tick.
    if (this.isFull() || !this.canConsume()) {
        return false;
    }

    const material = supplier.supply();
    if (material != null) {
        // Consume the material.
        const supplyTime = this.clock.time + this.capacity;
        this.contents.enqueue(new BeltMaterial(material, supplyTime));
        return true;
    }

    return false;
};
Belt.prototype.isMaterialReady = function () {
    // Check the next material on the belt.
    var beltOut = this.contents.peek();
    // Check if the next material is ready.
    if (beltOut != null && beltOut.ready(time)) {
        return true;
    }

    return false;
};
Belt.prototype.supply = function () {
    // Check if the next material is ready.
    if (this.isMaterialReady()) {
        // Supply the material.
        return this.contents.dequeue();
    }

    return null;
};


function Building(numInputs, numOutputs, internalBelts) {
    this.numInputs = numInputs;
    this.numOutputs = numOutputs;
    this.internalBelts = internalBelts;

    this.inputs = [];
    this.outputs = [];
}
Building.prototype.connectInput = function (belt) {
    const building = this;
    if (this.inputs.length < this.numInputs) {
        // Register MaterialReady event to each internal belt.
        for (var beltIndex = 0; beltIndex < building.internalBelts.length; ++beltIndex) {
            const internalBelt = building.internalBelts[beltIndex];
            belt.on(Belt.events.MaterialReady, function () {
                internalBelt.consume(belt)
            });
        }

        this.inputs.push(belt);
        return true;
    }

    return false;
};
Building.prototype.connectOutput = function (belt) {
    const building = this;
    if (this.outputs.length < this.numOutputs) {
        // Register all MaterialReady events to the output belt.
        for (var beltIndex = 0; beltIndex < building.internalBelts.length; ++beltIndex) {
            const internalBelt = building.internalBelts[beltIndex];
            internalBelt.on(Belt.events.MaterialReady, function () {
                belt.consume(internalBelt)
            });
        }

        this.outputs.push(belt);
        return true;
    }

    return false;
};

function Source(clock, material, numOutputs, materialPerSecond) {
    this.clock = clock;
    this.material = material;
    this.numOutputs = numOutputs;

    this.outputs = [];

    this.materialPerSecond = materialPerSecond;
    this.lastSuppliedTime = -1;
    this.materialProduced = [];

    this.eventEmitter = new EventEmitter();
    this.clock.on(Clock.events.Second, this.run.bind(this));
}
Source.events = {
    MaterialReady: 'source-material-ready'
};
Source.prototype.on = function (eventName, listener) {
    this.eventEmitter.on(eventName, listener);
};
Source.prototype.run = function () {
    this.materialProduced = [];
    const materialToProduce = Math.floor(Math.abs(this.clock.time - this.lastSuppliedTime) * this.materialPerSecond);
    if (materialToProduce > 0 || this.lastSuppliedTime == -1) {
        this.lastSuppliedTime = this.clock.time;
        console.info("producing " + materialToProduce + " " + this.material.name);

        // Produce that much material.
        for (var i = 0; i < materialToProduce; ++i) {
            this.materialProduced.push(this.material.clone());
            this.eventEmitter.emit(Source.events.MaterialReady);
        }
    }
};
Source.prototype.supply = function () {
    if (this.lastSuppliedTime != this.clock.time) {
        this.lastSuppliedTime = this.clock.time;
        return new Material();
    }
};
Source.prototype.connectOutput = function (belt) {
    const source = this;
    if (this.outputs.length < this.numOutputs) {
        this.eventEmitter.on(Source.events.MaterialReady, function () {
            belt.consume(source);
        });
        this.outputs.push(belt);
    };
};


function Forgery() {
    this.activeTool = null;
}

Forgery.prototype.setup = function () {
    this.toolbox = new Toolbox(tools);
    this.workspace = new Workspace(workspaceWidth, workspaceHeight);

    this.toolbox.load();
    this.workspace.load();

    this.toolbox.on(Toolbox.events.SelectTool, function (tool) {
        console.debug("selected tool: " + tool.name);
    });

    console.debug("Forgery setup");
};

Forgery.prototype.run = function () {
    const clock = new Clock();

    var totalCoalProduced = 0;
    clock.on(Clock.events.Second, function () {
        console.info("totalCoalProduced: " + totalCoalProduced);
    });

    const coalSource = new Source(clock, new Material("coal"), 1, 1);
    const sourceBelt = new Belt(clock, 10);
    coalSource.connectOutput(sourceBelt);

    const coalFactoryBelt = new Belt(clock, 10);
    const outputBelt = new Belt(clock, 1);
    const coalFactory = new Building(1, 1, [coalFactoryBelt]);
    coalFactory.connectInput(sourceBelt);
    coalFactory.connectOutput(outputBelt);

    outputBelt.on(Belt.events.MaterialReady, function () {
        if (outputBelt.supply() != null) {
            console.info("produced coal");
            totalCoalProduced = totalCoalProduced + 1
        }
    });

};





