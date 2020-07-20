'use strict';

// Based on: https://www.redblobgames.com/grids/hexagons/implementation.html

const DEBUG = true;

const ASSERT = function (assertion, message = "assertion failed") {
    if (!assertion) {
        const err = new Error(message);
        throw err;
    }
};

const ASSERT_INSTANCE_OF = function (value, type) {
    const message = value + " is not an instance of " + type.name;
    if (type === Boolean) {
        ASSERT(typeof value === "boolean", message)
        return;
    }
    ASSERT(value instanceof type, message);
};

const ASSERT_TYPE = function (value, typeName) {
    ASSERT((typeof value) == typeName, value + " is not type " + typeName);
};

const ASSERT_NUMBER = function (value) {
    ASSERT_TYPE(value, "number");
};

const ASSERT_INTEGER = function (value) {
    ASSERT(Number.isSafeInteger(value), value + " is not an integer");
};

// Make that shit readonly.
const READONLY = function (obj, name, value) {
    obj.__defineGetter__(name, function () { return value; });
};

const LERP = function (a, b, t) {
    DEBUG && ASSERT_NUMBER(a);
    DEBUG && ASSERT_NUMBER(b);
    DEBUG && ASSERT_NUMBER(t);

    return a * (1 - t) + b * t;
};

class Hex { // Vector storage, cube constructor
    static directions() {
        return HEX_DIRECTIONS;
    }

    constructor(q, r, s) {
        READONLY(this, "q", q);
        READONLY(this, "r", r);
        READONLY(this, "s", s);

        DEBUG && ASSERT_INTEGER(q);
        DEBUG && ASSERT_INTEGER(r);
        DEBUG && ASSERT_INTEGER(s);
        DEBUG && ASSERT(q + r + s == 0, "invalid hex coordinates");
    }

    equals(b) {
        DEBUG && ASSERT_INSTANCE_OF(b, Hex);
        return this.q == b.q && this.r == b.r && this.s == b.s;
    }

    add(b) {
        DEBUG && ASSERT_INSTANCE_OF(b, Hex);
        return new Hex(this.q + b.q, this.r + b.r, this.s + b.s);
    }

    subtract(b) {
        DEBUG && ASSERT_INSTANCE_OF(b, Hex);
        return new Hex(this.q - b.q, this.r - b.r, this.s - b.s);
    }

    multiply(k) {
        DEBUG && ASSERT_NUMBER(k);
        return new Hex(this.q * k, this.r * k, this.s * k);
    }

    length() {
        return (Math.abs(this.q) + Math.abs(this.r) + Math.abs(this.s)) / 2;
    }

    distance(b) {
        DEBUG && ASSERT_INSTANCE_OF(b, Hex);
        return this.length(this.subtract(b));
    }

    direction(direction) {
        DEBUG && ASSERT_INTEGER(direction);
        DEBUG && ASSERT(0 <= direction && direction < 6);
        return Hex.directions()[direction];
    }

    neighbor(direction) {
        DEBUG && ASSERT_INSTANCE_OF(direction, Hex);
        return this.add(this.direction(direction));
    }

    rotateLeft(orientation) {
        DEBUG && ASSERT_INSTANCE_OF(orientation, Orientation);

        return new Hex(-this.s, -this.q, -this.r);
    }

    rotateRight(orientation) {
        DEBUG && ASSERT_INSTANCE_OF(orientation, Orientation);

        return new Hex(-this.r, -this.s, -this.q);
    }

    lerp(b, t) {
        DEBUG && ASSERT_INSTANCE_OF(b, Hex);
        DEBUG && ASSERT_NUMBER(t);

        return new FractionalHex(
            LERP(this.q, b.q, t),
            LERP(this.r, b.r, t),
            LERP(this.s, b.s, t)
        );
    }

    linedraw(b) {
        DEBUG && ASSERT_INSTANCE_OF(b, Hex);

        const N = this.distance(b);
        results = [];
        const step = 1.0 / Math.max(N, 1);
        for (let i = 0; i <= N; ++i) {
            results.push(
                this.lerp(b, step * i).round()
            );
        }

        return results;
    }

    hashCode() {
        return this.q + "," + this.r + "," + this.s;
    }
}

const HEX_DIRECTIONS = [
    new Hex(1, 0, -1), new Hex(1, -1, 0), new Hex(0, -1, 1),
    new Hex(-1, 0, 1), new Hex(-1, 1, 0), new Hex(0, 1, -1)
];

class FractionalHex {
    constructor(q, r, s) {
        READONLY(this, "q", q);
        READONLY(this, "r", r);
        READONLY(this, "s", s);

        DEBUG && ASSERT_NUMBER(q);
        DEBUG && ASSERT_NUMBER(r);
        DEBUG && ASSERT_NUMBER(s);
        DEBUG && ASSERT(q + r + s == 0, "invalid fractional hex coordinates");
    }

    round() {
        let q = Math.round(this.q);
        let r = Math.round(this.r);
        let s = Math.round(this.s);

        const qDiff = Math.abs(q - this.q);
        const rDiff = Math.abs(r - this.r);
        const sDiff = Math.abs(s - this.s);

        if (qDiff > rDiff && qDiff > sDiff) {
            q = -r - s;
        } else if (rDiff > sDiff) {
            r = -q - s;
        } else {
            s = -q - r;
        }

        return new Hex(q, r, s);
    }
}

class Orientation {
    constructor(f0, f1, f2, f3, b0, b1, b2, b3, startAngle) {
        READONLY(this, "f0", f0);
        READONLY(this, "f1", f1);
        READONLY(this, "f2", f2);
        READONLY(this, "f3", f3);
        READONLY(this, "b0", b0);
        READONLY(this, "b1", b1);
        READONLY(this, "b2", b2);
        READONLY(this, "b3", b3);
        // In multiples of 60 degrees.
        READONLY(this, "startAngle", startAngle);

        DEBUG && ASSERT_NUMBER(f0);
        DEBUG && ASSERT_NUMBER(f1);
        DEBUG && ASSERT_NUMBER(f2);
        DEBUG && ASSERT_NUMBER(f3);
        DEBUG && ASSERT_NUMBER(b0);
        DEBUG && ASSERT_NUMBER(b1);
        DEBUG && ASSERT_NUMBER(b2);
        DEBUG && ASSERT_NUMBER(b3);
        DEBUG && ASSERT_NUMBER(startAngle);
    }

    static pointy() {
        return LAYOUT_POINTY;
    }

    static flat() {
        return LAYOUT_FLAT;
    }
}

const LAYOUT_POINTY = new Orientation(
    Math.sqrt(3.0), Math.sqrt(3.0) / 2.0, 0.0, 3.0 / 2.0,
    Math.sqrt(3.0) / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0,
    0.5
);

const LAYOUT_FLAT = new Orientation(
    3.0 / 2.0, 0.0, Math.sqrt(3.0) / 2.0, Math.sqrt(3.0),
    2.0 / 3.0, 0.0, -1.0 / 3.0, Math.sqrt(3.0) / 3.0,
    0.0
);

class Point {
    constructor(x, y) {
        READONLY(this, "x", x);
        READONLY(this, "y", y);

        DEBUG && ASSERT_NUMBER(x);
        DEBUG && ASSERT_NUMBER(y);
    }

    static origin() {
        return ORIGIN;
    }
}

const ORIGIN = new Point(0, 0);

class Layout {
    constructor(orientation, size, origin) {
        READONLY(this, "orientation", orientation);
        READONLY(this, "size", size);
        READONLY(this, "origin", origin);

        DEBUG && ASSERT_INSTANCE_OF(orientation, Orientation);
        DEBUG && ASSERT_INSTANCE_OF(size, Point);
        DEBUG && ASSERT_INSTANCE_OF(origin, Point);
    }

    hexToPixel(hex) {
        DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

        const x = (this.orientation.f0 * hex.q + this.orientation.f1 * hex.r) * this.size.x;
        const y = (this.orientation.f2 * hex.q + this.orientation.f3 * hex.r) * this.size.y;

        return new Point(x + this.origin.x, y + this.origin.y);
    }

    pixelToHex(point) {
        DEBUG && ASSERT_INSTANCE_OF(point, Point);

        pt = new Point(
            (point.x - this.origin.x) / this.size.x,
            (point.y - this.origin.y) / this.size.y
        );
        q = this.orientation.b0 * pt.x + this.orientation.b1 * pt.y;
        r = this.orientation.b2 * pt.x + this.orientation.b3 * pt.y;

        return new FractionalHex(q, r, -q - r);
    }

    hexCornerOffset(corner) {
        DEBUG && ASSERT_INTEGER(corner);

        const angle = 2.0 * Math.PI * (this.orientation.startAngle + corner) / 6;

        return new Point(this.size.x * Math.cos(angle), this.size.y * Math.sin(angle));
    }

    polygonCorners(hex) {
        DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

        const corners = [];
        const center = this.hexToPixel(hex);

        for (let i = 0; i < 6; ++i) {
            const offset = this.hexCornerOffset(i);
            corners.push(new Point(center.x + offset.x, center.y + offset.y));
        }

        return corners;
    }
}

class HexMap {
    constructor(type) {
        READONLY(this, "map", {});
    }

    get(hex) {
        DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

        return this.map[hex.hashCode()];
    }

    insert(hex) {
        DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

        this.map[hex.hashCode()] = hex;
    }

    remove(hex) {
        DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

        this.map.delete(hex.hashCode());
    }

    forEach(fn) {
        const keys = Object.keys(this.map);

        for (let i = 0; i < keys.length; ++i) {
            const key = keys[i];
            const hex = this.map[key];
            fn(hex);
        }
    }

    clear() {
        const keys = Object.keys(this.map);

        for (let i = 0; i < keys.length; ++i) {
            const key = keys[i];
            this.map.delete(key);
        }
    }

    generateHexgon(radius) {
        DEBUG && ASSERT_INTEGER(radius);

        this.clear();

        for (let q = -radius; q <= radius; ++q) {
            const r1 = Math.max(-radius, -q - radius);
            const r2 = Math.min(radius, -q + radius);

            for (let r = r1; r <= r2; ++r) {
                this.insert(new Hex(q, r, -q - r));
            }
        }
    }

    generateRectangle(width, height) {
        DEBUG && ASSERT_INTEGER(width);
        DEBUG && ASSERT_INTEGER(height);

        for (let r = 0; r < height; ++r) {
            rOffset = Math.floor(r / 2);
            for (let q = -rOffset; q < width - rOffset; ++q) {
                this.insert(new Hex(q, r, -q - r));
            }
        }
    }
}


