# Forgery

A production-style game. A proof of concept Electron application.

# Design

## Buildings

A building is represented as a polygon. A building can have input and output
belt nodes that reside somewhere on the perimeter of the building polygon. A
belt node can be connected to a belt that the building can either take units
from or place units onto.

An input belt node is represented by an empty circle. An output belt node is
represented by a filled circle inside another circle. On mouse over, the node
grows slightly larger. On clicking, the node is selected and highlighted.
Hitting escape or clicking on another node deselects the node.

Different buildings may be placed in the world. Each building is available in a
tool bar to select. After selecting a building from the tool bar, clicking
anywhere in the world will place the building in that spot. If a building
already exists that would overlap a newly placed building, no building is
placed. Hitting escape or selecting another tool deselects the tool.

Clicking on a building in the world selects the building. Hitting escape or
selecting another building or tool deselects the building. Selecting a building
displays information about the building in a context info area of the screen.
The context area displays the name of the building and a key-value list of
useful information and statistics provided by the model. The context area also
displays warning information about the building, such warning the use that one
or more of its belt nodes are not connected.

A building has one storage area for each belt. The building may have more
internal storage depending on the type of building. 

A building has a building state machine. The building state machine contains the
following states:

  * Disconnected: One or more belt nodes are not connected to a belt.

  * Transferring: The internal storage for the input belts are not completely
    full, and the ones that aren't full have materials available on the belt.
    
  * Starved: The internal storage for the input belts are not completely full,
    and at least one of the ones that aren't have empty belts.

  * Working: The internal storage for the input belts is full and the internal
    storage for the output belts are not.

  * Blocked: The internal storage for the output belts are not completely empty
    and their belts are full.

  * Disabled: The building is manually disabled.

