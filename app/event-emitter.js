function EventEmitter() {
    this.events = {};
}

EventEmitter.prototype.on = function (eventName, listener) {
    if (typeof this.events[eventName] !== 'object') {
        this.events[eventName] = [];
    }

    this.events[eventName].push(listener);
};

EventEmitter.prototype.emit = function (eventName) {
    // Abort of there are no listeners.
    if (this.events[eventName] == null) {
        return
    }

    var i = 0;
    var listeners = [];
    var length = 0;
    // Ignore the event name.
    // Accept any args to pass to the event listener.
    var args = [].slice.call(arguments, 1);

    listeners = this.events[eventName].slice();
    length = listeners.length;

    for (i; i < length; ++i) {
        listeners[i].apply(this, args);
    }
};